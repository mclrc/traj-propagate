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
	// let body_name = spice::cstr!(body.to_ascii_uppercase());
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
