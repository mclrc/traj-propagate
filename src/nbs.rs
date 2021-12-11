use crate::file_utils;
use crate::spice_utils;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NBodySystemData {
	pub bodies: Vec<i32>,
	pub mus: Vec<f64>,
	pub states: Vec<ndarray::Array2<f64>>,
}

impl NBodySystemData {
	pub fn instant_from_mk(mk_name: &str, bodies: &[i32], t: &str) -> Self {
		spice::furnsh(mk_name);

		let states = vec![Self::state_at_instant(bodies, spice::str2et(t))];
		let mus = bodies.iter().map(|id| spice_utils::get_gm(*id)).collect();

		spice::unload(mk_name);

		NBodySystemData {
			bodies: bodies.to_vec(),
			mus,
			states,
		}
	}

	pub fn trajectories_from_mk(mk_name: &str, bodies: &[i32], t0: &str, t: i32, dt: u32) -> Self {
		spice::furnsh(mk_name);

		let mus = bodies.iter().map(|id| spice_utils::get_gm(*id)).collect();
		let mut states = Vec::new();

		let et_j2000 = spice::str2et(t0);

		let mut t_elapsed = 0.0;
		while t_elapsed < t as f64 {
			states.push(Self::state_at_instant(bodies, et_j2000 + t_elapsed));
			t_elapsed += dt as f64;
		}

		spice::unload(mk_name);

		NBodySystemData {
			bodies: bodies.to_vec(),
			mus,
			states,
		}
	}

	fn state_at_instant(bodies: &[i32], t: f64) -> ndarray::Array2<f64> {
		let cb_id = bodies[0];

		let state: Vec<ndarray::Array1<f64>> = bodies
			.iter()
			.map(|id| ndarray::arr1(&spice_utils::get_state(*id, cb_id, t)) * 1e3)
			.collect();

		let views: Vec<ndarray::ArrayView1<f64>> = state.iter().map(|s| s.view()).collect();

		ndarray::stack(ndarray::Axis(0), &views[..]).unwrap()
	}

	pub fn serialize_to_pickle(&self, fname: &str, nth_steps: usize) {
		let states: Vec<ndarray::Array2<f64>> =
			self.states.iter().cloned().step_by(nth_steps).collect();

		file_utils::serialize_to_pickle(
			&NBodySystemData {
				bodies: self.bodies.clone(),
				mus: self.mus.clone(),
				states,
			},
			fname.to_string(),
		);
	}
}
