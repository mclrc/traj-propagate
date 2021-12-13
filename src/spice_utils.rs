use crate::nbs;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

pub fn label_for_id(id: i32) -> String {
	spice::bodc2n(id).0
}

pub fn id_for_label(label: &str) -> i32 {
	spice::bodn2c(label).0
}

pub fn to_labels(ids: &[i32]) -> Vec<String> {
	ids.iter().map(|id| label_for_id(*id)).collect()
}

pub fn to_ids(labels: &[&str]) -> Vec<i32> {
	labels.iter().map(|label| id_for_label(label)).collect()
}

pub fn get_gm(body: i32) -> f64 {
	let body_name = spice::cstr!(body.to_string());
	let value_name = spice::cstr!("GM");
	let mut dim: i32 = 0;
	let mut value: f64 = 0.0;

	unsafe {
		spice::c::bodvrd_c(body_name, value_name, 1, &mut dim, &mut value);
	}

	value * 1e9
}

pub fn get_mass(body: i32) -> f64 {
	get_gm(body) / 6.67408e-11
}

pub fn get_state(body: i32, cb_id: i32, t: f64) -> [f64; 6] {
	spice::core::raw::spkezr(&body.to_string(), t, "J2000", "NONE", &cb_id.to_string()).0
}

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

pub fn write_to_spk(system: &nbs::NBodySystemData, fname: &str, cb_id: i32, accuracy: f32) {
	if accuracy < 0.0 || accuracy > 1.0 {
		panic!("Please supply an accuracy value between 0 and 1")
	}
	let fname = spice::cstr!(fname);
	let ifname = spice::cstr!("SPK_File");
	let ncomch = 50;
	let mut handle = 0;
	unsafe { spice::c::spkopn_c(fname, ifname, ncomch, &mut handle) };

	let steps_to_skip = (1.0 / accuracy) as usize;
	let states_to_save: Vec<&ndarray::Array2<f64>> =
		system.states.iter().step_by(steps_to_skip).collect();

	let dts_to_save: Vec<f32> = system
		.dts
		.chunks(steps_to_skip)
		.map(|c| c.iter().sum())
		.collect();

	let mut epochs_j2000 = Vec::<f64>::with_capacity(dts_to_save.len());
	let t0_j2000 = spice::str2et(&system.t0);
	epochs_j2000.push(t0_j2000);

	for (idx, &dt) in dts_to_save.iter().skip(1).enumerate() {
		epochs_j2000.push(epochs_j2000[idx] + f64::from(dt));
	}

	let cb_idx = system
		.bodies
		.iter()
		.position(|&id| id == cb_id)
		.expect("Dataset does not contain specified observing body");

	let cb_states: Vec<ndarray::ArrayView1<f64>> = states_to_save
		.iter()
		.map(|&s| s.slice(ndarray::s![cb_idx, ..]))
		.collect();
	let cb_states_matrix = ndarray::concatenate(ndarray::Axis(0), &cb_states[..]).unwrap();

	for (idx, &id) in system.bodies.iter().enumerate() {
		if id == cb_id {
			continue;
		}

		let states = states_to_save
			.iter()
			.map(|&s| s.slice(ndarray::s![idx, ..]))
			.collect::<Vec<ndarray::ArrayView1<f64>>>();

		let mut states_matrix_km =
			(ndarray::concatenate(ndarray::Axis(0), &states[..]).unwrap() - &cb_states_matrix) / 1000.0;

		let segid = spice::cstr!(format!("Position of {} relative to {}", id, cb_id));
		let frame = spice::cstr!("J2000");
		unsafe {
			spice::c::spkw09_c(
				handle,
				id,
				cb_id,
				frame,
				epochs_j2000[0],
				epochs_j2000[epochs_j2000.len() - 1],
				segid,
				11,
				states_to_save.len() as i32,
				states_matrix_km.as_mut_ptr().cast(),
				epochs_j2000.as_mut_ptr(),
			)
		}
	}

	unsafe { spice::c::spkcls_c(handle) };
}
