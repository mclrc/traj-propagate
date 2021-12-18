use crate::spice_utils;

pub struct NBodySystemData {
	// Initial time as UTC timestamp "YYY-MMM-DD HH:MM:SS"
	pub t0: String,
	// NAIF-IDs of included bodies
	pub bodies: Vec<i32>,
	// Standard gravitational parameters of included bodies
	pub mus: Vec<f64>,
	// Known/calculated states
	pub states: Vec<ndarray::Array2<f64>>,
	// dts in s corresponding to states
	pub dts: Vec<f32>,
}

impl NBodySystemData {
	// Retrieves state at t for specified bodies from specified meta kernel
	pub fn instant_from_spice(mk_name: &str, bodies: &[i32], t: &str) -> Self {
		spice::furnsh(mk_name);

		// Retrieve initial state vectors
		let y0 = spice_utils::states_at_instant(bodies, spice::str2et(t));
		// Retrieve mus
		let mus = bodies.iter().map(|&id| spice_utils::get_gm(id)).collect();

		spice::unload(mk_name);

		NBodySystemData {
			t0: t.to_string(),
			bodies: bodies.to_vec(),
			mus,
			states: vec![y0],
			dts: vec![0.0],
		}
	}
}
