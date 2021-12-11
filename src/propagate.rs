use crate::ivp_utils;
use crate::nbs::NBodySystemData;

fn a_grav(r: &ndarray::Array1<f64>, mu: f64) -> ndarray::Array1<f64> {
	(mu / (r.dot(r).sqrt().powf(3.0))) * r
}

fn calc_derivative(state: &ndarray::Array2<f64>, mus: &[f64]) -> ndarray::Array2<f64> {
	let n = mus.len();

	let mut accels = ndarray::Array2::<f64>::zeros((n, 3));

	for b1 in 0..n {
		for b2 in (b1 + 1)..n {
			let r = &state.slice(ndarray::s![b2, ..3]) - &state.slice(ndarray::s![b1, ..3]);

			let mut a1 = accels.slice_mut(ndarray::s![b1, ..]);
			a1 += &a_grav(&r, mus[b2]);

			let mut a2 = accels.slice_mut(ndarray::s![b2, ..]);
			a2 += &a_grav(&-r, mus[b1]);
		}
	}
	let vels = state.slice(ndarray::s![.., 3..]);

	ndarray::concatenate(ndarray::Axis(1), &[vels.view(), accels.view()]).unwrap()
}

pub fn propagate(
	system: &NBodySystemData,
	solver: &ivp_utils::Solver<ndarray::Array2<f64>>,
	t: u64,
	dt: u32,
) -> NBodySystemData {
	let mus = system.mus.clone();
	let f = move |state: &ndarray::Array2<f64>| calc_derivative(state, &mus);

	let mut states = system.states.clone();

	let mut propagated = ivp_utils::solve_ivp(
		&system.states[system.states.len() - 1],
		&f,
		solver,
		t,
		dt as f64,
	);

	states.append(&mut propagated);

	NBodySystemData {
		labels: system.labels.clone(),
		mus: system.mus.clone(),
		states,
	}
}
