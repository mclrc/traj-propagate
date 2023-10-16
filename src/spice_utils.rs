use ndarray::{arr1, concatenate, s, Array1, Axis};
use std::ffi::CStr;
use std::os::raw::c_char;
use std::path::Path;

// Sets SPICE error handling action, message length and error device
pub fn set_error_handling(action: &str, len: &str, dev: &str) {
	unsafe {
		spice::c::errprt_c(spice::cstr!("set"), 0, spice::cstr!(len));
		spice::c::erract_c(spice::cstr!("set"), 20, spice::cstr!(action));
		spice::c::errdev_c(spice::cstr!("set"), 0, spice::cstr!(dev));
	}
}

/// Ok if SPICE has not signaled an error, Err containing "short" error message if it has
/// If Err, SPICE error status will also be reset to enable subsequent use
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
	set_error_handling("return", "short", "NULL");

	let mut dim = 0;
	let mut value = 0f64;

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
	set_error_handling("return", "short", "NULL");

	let (pos, _) =
		spice::core::raw::spkezr(&body.to_string(), et, "J2000", "NONE", &cb_id.to_string());

	get_spice_result_and_reset().map_err(|msg| {
		format!("Could not retrieve state of {body} relative to {cb_id} at {et}: {msg}")
	})?;
	Ok(arr1(&pos))
}

/// Retrieve state vectors of specified bodies at et
pub fn states_at_instant(bodies: &[i32], cb_id: i32, et: f64) -> Result<Array1<f64>, String> {
	let mut state = ndarray::Array1::zeros(bodies.len() * 6);

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
	set_error_handling("return", "short", "NULL");

	if !(0f32..=1f32).contains(&fraction_to_save) {
		return Err("Please supply a fraction_to_save value between 0 and 1".to_string());
	}

	// Open SPK file for writing
	let mut handle = 0;
	let kernel_exists = Path::new(fname).exists();
	if kernel_exists {
		unsafe {
			spice::c::spkopa_c(spice::cstr!(fname), &mut handle);
		};
	} else {
		handle = spice::spkopn(
			fname,
			"Propagated",
			256, // Number of characters reserved for comments
		);
	}

	get_spice_result_and_reset()
		.map_err(|msg| format!("Failed to open SPK file for writing: {msg}"))?;

	// Extract states to actually write to the file
	let steps_to_skip = (1f32 / fraction_to_save) as usize;
	let mut ets = ets
		.iter()
		.step_by(steps_to_skip)
		.cloned()
		.collect::<Vec<_>>();
	let states = states.iter().step_by(steps_to_skip).collect::<Vec<_>>();

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
			.collect::<Vec<_>>();

		let mut states_matrix_km = (concatenate(Axis(0), &body_states[..]).unwrap()) / 1000f64;

		if let Some(ref cb_states_matrix_km) = cb_states_matrix_km {
			states_matrix_km -= cb_states_matrix_km;
		}

		spice::spkw09(
			// Handle for previously created, opened SPK file
			handle,
			// Target body ID
			id,
			// Observing body ID
			cb_id,
			// Reference frame
			"J2000",
			// t0
			ets[0],
			// tfinal
			ets[ets.len() - 1],
			// Segment identifier
			&format!("Position of {} relative to {}", id, cb_id),
			// Degree of polynomial to be used for lagrange interpolation. Currently somewhat arbitrary.
			7,
			// Number of states/epochs
			body_states.len() as i32,
			// Pointer to beginning of state matrix
			unsafe {
				core::slice::from_raw_parts_mut(
					states_matrix_km.as_mut_ptr().cast::<[f64; 6]>(),
					body_states.len(),
				)
			},
			// Epoch vec
			&mut ets,
		)
	}

	get_spice_result_and_reset()
		.map_err(|msg| format!("Failed to write segment to SPK file: {msg}"))?;

	// Close previously created and populated SPK file
	spice::spkcls(handle);

	get_spice_result_and_reset().map_err(|msg| format!("Failed to close SPK file: {msg}"))?;

	Ok(())
}
