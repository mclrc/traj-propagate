use crate::ivp_utils;
use crate::nbs::NBodySystemData;

// Calculate gravitational acceleration based on vector to attracting body and mu
fn a_grav(r: &ndarray::Array1<f64>, mu: f64) -> ndarray::Array1<f64> {
	(mu / (r.dot(r).sqrt().powf(3.0))) * r
}

// Calculate value of derivative for given state
fn calc_derivative(state: &ndarray::Array2<f64>, mus: &[f64]) -> ndarray::Array2<f64> {
	let n = mus.len();

	let mut accels = ndarray::Array2::<f64>::zeros((n, 3));

	// For every body index...
	for b1 in 0..n {
		// Interactions with other bodies earlier in the slice have already been evaluated,
		// as the inner loop of those bodies' iterations of the outer loop already passed over this index.
		// For every body index greater than this one...
		for b2 in (b1 + 1)..n {
			// Vector from b1 to b2
			let r = &state.slice(ndarray::s![b2, ..3]) - &state.slice(ndarray::s![b1, ..3]);

			// Calc acceleration of b1 due to b2
			let mut a1 = accels.slice_mut(ndarray::s![b1, ..]);
			a1 += &a_grav(&r, mus[b2]);

			// Calc acceleration of b2 due to b1
			let mut a2 = accels.slice_mut(ndarray::s![b2, ..]);
			a2 += &a_grav(&-r, mus[b1]);
		}
	}
	// Derivative also includes velocities (1st derivative of position, acceleration is 2nd)
	// Extract them from the state vectors
	let vels = state.slice(ndarray::s![.., 3..]);

	// Return derivative state vectors in the form [vx, vy, vz, ax, ay, az]
	ndarray::concatenate(ndarray::Axis(1), &[vels.view(), accels.view()]).unwrap()
}

// Propagate N-body system
pub fn propagate(
	system: &NBodySystemData,
	solver: &ivp_utils::Solver<ndarray::Array2<f64>>,
	t: u64,
	dt: u32,
) -> NBodySystemData {
	// Clone mus (so the closure can take ownership)
	let mus = system.mus.clone();
	// Derivative closure
	let f = move |state: &ndarray::Array2<f64>| calc_derivative(state, &mus);

	let mut states = system.states.clone();
	let mut dts = system.dts.clone();

	// Compute new states
	let (new_states, new_dts) = ivp_utils::solve_ivp(
		&system.states[system.states.len() - 1],
		&f,
		solver,
		t,
		dt as f32,
	);

	// Combine new and old data
	states.extend_from_slice(&new_states[1..]);
	dts.extend_from_slice(&new_dts[1..]);

	NBodySystemData {
		t0: system.t0.clone(),
		bodies: system.bodies.clone(),
		mus: system.mus.clone(),
		states,
		dts,
	}
}
