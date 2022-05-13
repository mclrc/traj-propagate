use ndarray::{arr1, concatenate, s, Array1, ArrayView1, Axis};
use std::ffi::CStr;
use std::os::raw::c_char;

pub fn set_error_handling(action: &str, len: &str) {
	unsafe {
		spice::c::errprt_c(spice::cstr!("set"), 0, spice::cstr!(len));
		spice::c::erract_c(spice::cstr!("set"), 20, spice::cstr!(action));
		spice::c::errdev_c(spice::cstr!("SET"), 0, spice::cstr!("NULL"));
	}
}

pub fn get_spice_result_and_reset() -> Result<(), String> {
	unsafe {
		if spice::c::failed_c() != 0 {
			let mut msgbuf = [0 as c_char; 50];
			spice::c::getmsg_c(spice::cstr!("short"), 50, msgbuf.as_mut_ptr());
			spice::c::reset_c();
			return Err(CStr::from_ptr(msgbuf.as_ptr())
				.to_str()
				.unwrap()
				.to_string());
		}
	}
	Ok(())
}

/// Parse body names/id strings to NAIF-ID i32s
pub fn naif_ids(bodies: &[impl AsRef<str>]) -> Result<Vec<i32>, String> {
	let mut ids = Vec::new();
	for b in bodies {
		let b = b.as_ref();
		match spice::bodn2c(b) {
			(id, true) => ids.push(id),
			(_, false) => return Err(format!("Unknown body: '{b}'")),
		}
	}
	Ok(ids)
}

/// Retrieve standard gravitational parameter for body
pub fn mu(body: i32) -> Result<f64, String> {
	set_error_handling("return", "short");

	let mut dim: i32 = 0;
	let mut value: f64 = 0.0;

	unsafe {
		spice::c::bodvrd_c(
			spice::cstr!(body.to_string()),
			spice::cstr!("GM"),
			1,
			&mut dim,
			&mut value,
		);
	};

	get_spice_result_and_reset()
		.map_err(|msg| format!("Could not retrieve GM for body {body}: {msg}"))?;

	// Unit conversion: km^3/s^2 to m^3/s^2
	Ok(value * 1e9)
}

/// Retrieve state vector for body relative to central body at t
pub fn state_at_instant(body: i32, cb_id: i32, et: f64) -> Result<Array1<f64>, String> {
	set_error_handling("return", "short");

	let (pos, _) =
		spice::core::raw::spkezr(&body.to_string(), et, "J2000", "NONE", &cb_id.to_string());

	get_spice_result_and_reset().map_err(|msg| {
		format!("Could not retrieve state of {body} relative to {cb_id} at {et}: {msg}")
	})?;
	Ok(arr1(&pos))
}

/// Retrieve state vectors of specified bodies at et
pub fn states_at_instant(bodies: &[i32], cb_id: i32, et: f64) -> Result<Array1<f64>, String> {
	let mut state = ndarray::Array1::<f64>::zeros(bodies.len() * 6);

	for (idx, &b) in bodies.iter().enumerate() {
		let mut s = state.slice_mut(s![(idx * 6)..(idx * 6 + 6)]);
		s += &state_at_instant(b, cb_id, et)?;
	}

	Ok(state)
}

/// Write data to SPK file
pub fn write_to_spk(
	fname: &str,
	bodies: &[i32],
	states: &[Array1<f64>],
	ets: &[f64],
	cb_id: i32,
	fraction_to_save: f32,
) -> Result<(), String> {
	set_error_handling("return", "short");

	if !(0.0..=1.0).contains(&fraction_to_save) {
		return Err("Please supply a fraction_to_save value between 0 and 1".to_string());
	}

	// Open a new SPK file.
	let mut handle = 0;
	unsafe {
		spice::c::spkopn_c(
			spice::cstr!(fname),        // File name
			spice::cstr!("Propagated"), // Internal file name
			256,                        // Number of characters reserved for comments
			&mut handle,
		)
	};

	get_spice_result_and_reset()
		.map_err(|msg| format!("Failed to open SPK file for writing: {msg}"))?;

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

	// If the observing bodies trajectory was also propagated, assemble a state matrix for that body
	// that can be substracted from other bodies state matrices to yield state relative to observing body
	let cb_states_matrix_km = bodies.iter().position(|&id| id == cb_id).map(|idx| {
		let cb_states = states
			.iter()
			.map(|&s| s.slice(s![(idx * 6)..(idx * 6 + 6)]))
			.collect::<Vec<_>>();

		concatenate(Axis(0), &cb_states).unwrap() / 1000f64
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

		unsafe {
			spice::c::spkw09_c(
				// Handle for previously created, opened SPK file
				handle,
				// Target body ID
				id,
				// Observing body ID
				cb_id,
				// Reference frame
				spice::cstr!("J2000"),
				// t0
				ets[0],
				// tfinal
				ets[ets.len() - 1],
				// Segment identifier
				spice::cstr!(format!("Position of {} relative to {}", id, cb_id)),
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

	get_spice_result_and_reset()
		.map_err(|msg| format!("Failed to write segment to SPK file: {msg}"))?;

	// Close previously created and populated SPK file
	unsafe { spice::c::spkcls_c(handle) };

	get_spice_result_and_reset().map_err(|msg| format!("Failed to close SPK file: {msg}"))?;

	Ok(())
}
