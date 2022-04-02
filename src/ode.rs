use crate::spice_utils;
use ndarray::{s, Array1};

/// Calculate value of derivative for given state
pub fn n_body_ode(
	et: f64,
	state: &Array1<f64>,
	mus: &[f64],
	attractors: &[(i32, f64)],
	cb_id: i32,
) -> Array1<f64> {
	let n = mus.len();

	let mut derivative = Array1::<f64>::zeros(n * 6);

	// For every pair of bodies
	for b1 in 0..n {
		// Derivative also includes velocities (1st derivative of position, acceleration is 2nd)
		// Extract them from the state vectors
		let mut v_slice = derivative.slice_mut(s![(b1 * 6)..(b1 * 6 + 3)]);
		v_slice += &state.slice(s![(b1 * 6 + 3)..(b1 * 6 + 6)]);

		// Calculate accelerations
		for b2 in (b1 + 1)..n {
			// Vector from b1 to b2
			let r =
				&state.slice(s![(b2 * 6)..(b2 * 6 + 3)]) - &state.slice(s![(b1 * 6)..(b1 * 6 + 3)]);
			// Calc accelerations
			let r_norm_cubed = r.dot(&r).sqrt().powf(3.0);
			let (a1, a2) = (mus[b2] / r_norm_cubed * &r, mus[b1] / r_norm_cubed * -(&r));
			// Update total acceleration for each body
			let mut a1_slice = derivative.slice_mut(s![(b1 * 6 + 3)..(b1 * 6 + 6)]);
			a1_slice += &a1;
			let mut a2_slice = derivative.slice_mut(s![(b2 * 6 + 3)..(b2 * 6 + 6)]);
			a2_slice += &a2;
		}
	}

	// If there are no attractors, there is nothing left to do
	if attractors.is_empty() {
		return derivative;
	}

	let attractor_positions = attractors
		.iter()
		.map(|&(id, _)| spice_utils::states_at_instant(&[id], cb_id, et).slice_move(s![..3]))
		.collect::<Vec<_>>();

	for b1 in 0..n {
		let pos_slice = state.slice(s![(b1 * 6)..(b1 * 6 + 3)]);
		let mut a_slice = derivative.slice_mut(s![(b1 * 6 + 3)..(b1 * 6 + 6)]);
		for (idx, &(_, mu)) in attractors.iter().enumerate() {
			let r = &attractor_positions[idx] - &pos_slice;
			let a = (mu / r.dot(&r).sqrt().powf(3.0)) * &r;
			a_slice += &a;
		}
	}

	derivative
}
