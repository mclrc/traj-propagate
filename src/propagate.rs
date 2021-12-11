use crate::ivp_utils;
use crate::nbs::NBodySystemData;
use crate::ode;

pub fn propagate(
	system: &NBodySystemData,
	solver: &ivp_utils::Solver<ndarray::Array2<f64>>, //Fn( &ivp_utils::solve::Derivative<ndarray::Array2<f64>>, &[ ndarray::Array2<f64> ], f64 ) -> ndarray::Array2<f64>,
	t: u64,
	dt: u32,
) -> NBodySystemData {
	let ode = ode::NBodyODE {
		mus: system.mus.clone(),
	};
	let mut states = system.states.clone();

	let mut propagated = ivp_utils::solve_ivp(
		&system.states[system.states.len() - 1],
		&move |s| ode.f(s),
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
