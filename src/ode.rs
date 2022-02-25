/// Calculate gravitational acceleration two bodies exert on one another
///   `r`: Vec from first to second body
///   `mu1`: Standard gravitational parameter of first body
///   `mu2`: Standard gravitational parameter of second body
/// Returns tuple `(a1, a2)` where
///   `a1`: Acceleration of the first body
///   `a2`: Acceleration of the second body
fn a_grav(
	r: &ndarray::Array1<f64>,
	mu1: f64,
	mu2: f64,
) -> (ndarray::Array1<f64>, ndarray::Array1<f64>) {
	let r_norm_cubed = r.dot(r).sqrt().powf(3.0);
	(mu2 / r_norm_cubed * r, mu1 / r_norm_cubed * -r)
}

/// Calculate value of derivative for given state
///   `state`: Current state of the system; ndarray of the bodies state vectors
///   `mus`: Standard gravitational parameters of the bodies
/// Returns ndarray of derivative state vectors
pub fn n_body_ode(state: &ndarray::Array2<f64>, mus: &[f64]) -> ndarray::Array2<f64> {
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
