use crate::nbs;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Retrieve standard gravitational parameter for body
pub fn get_gm(body: i32) -> f64 {
	let body_name = spice::cstr!(body.to_string());
	let value_name = spice::cstr!("GM");
	let mut dim: i32 = 0;
	let mut value: f64 = 0.0;

	unsafe {
		spice::c::bodvrd_c(body_name, value_name, 1, &mut dim, &mut value);
	}

	// Unit conversion: km^3/s^2 to m^3/s^2
	value * 1e9
}

/// Retrieve state vector for body relative to central body at t
pub fn get_state(body: i32, cb_id: i32, t: f64) -> [f64; 6] {
	spice::core::raw::spkezr(&body.to_string(), t, "J2000", "NONE", &cb_id.to_string()).0
}

/// Convert J2000 timestamp to UTC timestamp in string format
#[allow(dead_code)]
pub fn et2str(t: f64) -> String {
	let mut buff = [0 as c_char; 50];
	let format = CString::new("C").unwrap().into_raw();
	let result_str;
	unsafe {
		spice::c::et2utc_c(t, format, 0, 50, &mut buff[0]);

		result_str = CStr::from_ptr(&buff[0]).to_str().unwrap();
	}
	String::from(result_str)
}

/// Retrieve state vectors of specified bodies at t
pub fn states_at_instant(bodies: &[i32], t: f64) -> ndarray::Array2<f64> {
	let cb_id = bodies[0];

	let state: Vec<ndarray::Array1<f64>> = bodies
		.iter()
		.map(|&id| ndarray::arr1(&get_state(id, cb_id, t)) * 1e3)
		.collect();

	let views: Vec<ndarray::ArrayView1<f64>> = state.iter().map(|s| s.view()).collect();

	ndarray::stack(ndarray::Axis(0), &views[..]).unwrap()
}

/// Write data contained in system to SPK file
/// 'fraction_to_save' is the fraction of steps to save, e. g. 0.5 will save every 2nd step
pub fn write_to_spk(system: &nbs::NBodySystemData, fname: &str, cb_id: i32, fraction_to_save: f32) {
	if !(0.0..=1.0).contains(&fraction_to_save) {
		panic!("Please supply a fraction_to_save value between 0 and 1")
	}

	let fname = spice::cstr!(fname);
	// Internal file name
	let ifname = spice::cstr!("SPK_File");
	// Number of characters reserved for comments
	let ncomch = 50;
	// File handle (will be written to by FFI)
	let mut handle = 0;
	// Call to CSPICE funnction to open a new SPK file.
	unsafe { spice::c::spkopn_c(fname, ifname, ncomch, &mut handle) };

	// Extract states to actually write to the file
	let steps_to_skip = (1.0 / fraction_to_save) as usize;
	let states_to_save: Vec<&ndarray::Array2<f64>> =
		system.states.iter().step_by(steps_to_skip).collect();

	// Compute time differences between selected steps
	let dts_to_save: Vec<f32> = system
		.dts
		.chunks(steps_to_skip)
		.map(|c| c.iter().sum())
		.collect();

	// Create vec for J2000 timestamps corresponding to selected states
	let mut epochs_j2000 = Vec::<f64>::with_capacity(dts_to_save.len());
	let t0_j2000 = spice::str2et(&system.t0);
	epochs_j2000.push(t0_j2000);

	// Populate epoch vector with previous J2000 timestamp + dt
	for (idx, &dt) in dts_to_save.iter().skip(1).enumerate() {
		epochs_j2000.push(epochs_j2000[idx] + f64::from(dt));
	}

	// Extract index of central observing body that is used across NBSD fields
	let cb_idx = system
		.bodies
		.iter()
		.position(|&id| id == cb_id)
		.expect("Dataset does not contain specified observing body");

	// Create state matrix for central body to subtract from target body state matrices
	// to yield state relative to observing body
	let cb_states: Vec<ndarray::ArrayView1<f64>> = states_to_save
		.iter()
		.map(|&s| s.slice(ndarray::s![cb_idx, ..]))
		.collect();
	let cb_states_matrix = ndarray::concatenate(ndarray::Axis(0), &cb_states[..]).unwrap();

	for (idx, &id) in system.bodies.iter().enumerate() {
		// Skip observing body
		if id == cb_id {
			continue;
		}

		// Create state matrix for current target body with states in km and km/s
		let states = states_to_save
			.iter()
			.map(|&s| s.slice(ndarray::s![idx, ..]))
			.collect::<Vec<ndarray::ArrayView1<f64>>>();
		let mut states_matrix_km =
			(ndarray::concatenate(ndarray::Axis(0), &states[..]).unwrap() - &cb_states_matrix) / 1000.0;

		// SPICE segment identifier
		let segid = spice::cstr!(format!("Position of {} relative to {}", id, cb_id));
		// SPICE reference frame
		let frame = spice::cstr!("J2000");
		unsafe {
			spice::c::spkw09_c(
				// Handle for previously created, opened SPK file
				handle,
				// Target body ID
				id,
				// Observing body ID
				cb_id,
				// Reference frame
				frame,
				// t0
				epochs_j2000[0],
				// tfinal
				epochs_j2000[epochs_j2000.len() - 1],
				// Segment identifier
				segid,
				// Degree of polynomial to be used for lagrange interpolation. Currently somewhat arbitrary.
				7,
				// Number of states/epochs
				states_to_save.len() as i32,
				// Pointer to beginning of state matrix
				states_matrix_km.as_mut_ptr().cast(),
				// Pointer to beginning of epoch vec
				epochs_j2000.as_mut_ptr(),
			)
		}
	}

	// Close previously created and populated SPK file
	unsafe { spice::c::spkcls_c(handle) };
}
