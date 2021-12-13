use crate::spice_utils;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NBodySystemData {
	pub t0: String,
	pub bodies: Vec<i32>,
	pub mus: Vec<f64>,
	pub states: Vec<ndarray::Array2<f64>>,
	pub dts: Vec<f32>,
}

impl NBodySystemData {
	pub fn instant_from_mk(mk_name: &str, bodies: &[i32], t: &str) -> Self {
		spice::furnsh(mk_name);

		let states = vec![Self::state_at_instant(bodies, spice::str2et(t))];
		let mus = bodies.iter().map(|&id| spice_utils::get_gm(id)).collect();

		spice::unload(mk_name);

		NBodySystemData {
			t0: t.to_string(),
			bodies: bodies.to_vec(),
			mus,
			states,
			dts: vec![0.0],
		}
	}

	pub fn trajectories_from_mk(mk_name: &str, bodies: &[i32], t0: &str, t: i32, dt: f32) -> Self {
		spice::furnsh(mk_name);

		let mus = bodies.iter().map(|&id| spice_utils::get_gm(id)).collect();
		let mut states = Vec::new();

		let et_j2000 = spice::str2et(t0);

		let mut t_elapsed = 0.0;
		while t_elapsed < t as f64 {
			states.push(Self::state_at_instant(bodies, et_j2000 + t_elapsed));
			t_elapsed += dt as f64;
		}

		spice::unload(mk_name);

		let steps = states.len();

		NBodySystemData {
			t0: t0.to_string(),
			bodies: bodies.to_vec(),
			mus,
			states,
			dts: vec![dt; steps],
		}
	}

	fn state_at_instant(bodies: &[i32], t: f64) -> ndarray::Array2<f64> {
		let cb_id = bodies[0];

		let state: Vec<ndarray::Array1<f64>> = bodies
			.iter()
			.map(|&id| ndarray::arr1(&spice_utils::get_state(id, cb_id, t)) * 1e3)
			.collect();

		let views: Vec<ndarray::ArrayView1<f64>> = state.iter().map(|s| s.view()).collect();

		ndarray::stack(ndarray::Axis(0), &views[..]).unwrap()
	}
}
