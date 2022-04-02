use ndarray::{arr1, concatenate, s, Array1, ArrayView1, Axis};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Parse body names/id strings to i32s
pub fn naif_ids(bodies: &[impl AsRef<str>]) -> Vec<i32> {
	bodies
		.iter()
		.map(|b| b.as_ref())
		.map(|b| {
			b.parse::<i32>().unwrap_or_else(|_| match spice::bodn2c(b) {
				(id, true) => id,
				(_, false) => panic!("Body '{}' not found in kernel pool", b),
			})
		})
		.collect::<Vec<i32>>()
}

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
pub fn state_at_instant(body: i32, cb_id: i32, et: f64) -> [f64; 6] {
	spice::core::raw::spkezr(&body.to_string(), et, "J2000", "NONE", &cb_id.to_string()).0
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

/// Retrieve state vectors of specified bodies at et
pub fn states_at_instant(bodies: &[i32], cb_id: i32, et: f64) -> Array1<f64> {
	// TODO: There must be a cleaner way to do this
	let state: Vec<Array1<f64>> = bodies
		.iter()
		.map(|&id| arr1(&state_at_instant(id, cb_id, et)) * 1e3)
		.collect();

	let views: Vec<ArrayView1<f64>> = state.iter().map(|s| s.view()).collect();

	concatenate(Axis(0), &views[..]).unwrap()
}

/// Write data contained in system to SPK file
/// 'fraction_to_save' is the fraction of steps to save, e. g. 0.5 will save every 2nd step
pub fn write_to_spk(
	fname: &str,
	bodies: &[i32],
	states: &[Array1<f64>],
	ets: &[f64],
	cb_id: i32,
	fraction_to_save: f32,
) {
	println!("Writing to SPK...");
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

	let mut ets = ets
		.iter()
		.step_by(steps_to_skip)
		.cloned()
		.collect::<Vec<f64>>();
	let states = states
		.iter()
		.step_by(steps_to_skip)
		.collect::<Vec<&Array1<f64>>>();

	// Extract index of central observing body that is used across NBSD fields
	let cb_idx = bodies.iter().position(|&id| id == cb_id);

	// Create state matrix for central body to subtract from target body state matrices
	// to yield state relative to observing body
	let cb_states = cb_idx.map(|idx| {
		states
			.iter()
			.map(|&s| s.slice(s![(idx * 6)..(idx * 6 + 6)]).to_owned())
			.collect::<Vec<_>>()
	});
	let cb_states_matrix_km = cb_states.map(|cb_states| {
		concatenate(
			Axis(0),
			&cb_states
				.iter()
				.map(|s| s.view())
				.collect::<Vec<ArrayView1<f64>>>(),
		)
		.unwrap() / 1000f64
	});

	for (idx, &id) in bodies.iter().enumerate() {
		// Skip observing body
		if id == cb_id {
			continue;
		}

		// Create state matrix for current target body with states in km and km/s
		let body_states = states
			.iter()
			.map(|&s| s.slice(s![(idx * 6)..(idx * 6 + 6)]))
			.collect::<Vec<ArrayView1<f64>>>();

		let mut states_matrix_km = (concatenate(Axis(0), &body_states[..]).unwrap()) / 1000f64;

		if let Some(ref cb_states_matrix_km) = cb_states_matrix_km {
			states_matrix_km -= cb_states_matrix_km;
		}

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
				ets[0],
				// tfinal
				ets[ets.len() - 1],
				// Segment identifier
				segid,
				// Degree of polynomial to be used for lagrange interpolation. Currently somewhat arbitrary.
				7,
				// Number of states/epochs
				body_states.len() as i32,
				// Pointer to beginning of state matrix
				states_matrix_km.as_mut_ptr().cast(),
				// Pointer to beginning of epoch vec
				ets.as_mut_ptr(),
			)
		}
	}

	// Close previously created and populated SPK file
	unsafe { spice::c::spkcls_c(handle) };
}
