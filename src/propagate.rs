use crate::ivp_utils;
use crate::nbs::NBodySystemData;

// Calculate gravitational acceleration based on vector to attracting body and mu
fn a_grav(
	r: &ndarray::Array1<f64>,
	mu1: f64,
	mu2: f64,
) -> (ndarray::Array1<f64>, ndarray::Array1<f64>) {
	let r_norm_cubed = r.dot(r).sqrt().powf(3.0);
	(mu2 / r_norm_cubed * r, mu1 / r_norm_cubed * -r)
}

// Calculate value of derivative for given state
fn calc_derivative(state: &ndarray::Array2<f64>, mus: &[f64]) -> ndarray::Array2<f64> {
	let n = mus.len();

	let mut accels = ndarray::Array2::<f64>::zeros((n, 3));

	// For every pair of bodies
	for b1 in 0..n {
		for b2 in (b1 + 1)..n {
			// Vector from b1 to b2
			let r = &state.slice(ndarray::s![b2, ..3]) - &state.slice(ndarray::s![b1, ..3]);

			// Calc accelerations
			let (a1, a2) = a_grav(&r, mus[b1], mus[b2]);
			// Update total acceleration for each body
			let mut a1_slice = accels.slice_mut(ndarray::s![b1, ..]);
			a1_slice += &a1;

			let mut a2_slice = accels.slice_mut(ndarray::s![b2, ..]);
			a2_slice += &a2;
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
