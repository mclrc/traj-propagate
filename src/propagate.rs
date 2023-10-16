use crate::ode;
use crate::solvers;
use crate::spice_utils;
use ndarray::Array1;

pub enum SolverConfig {
	Rk4 { h: f64 },
	Euler { h: f64 },
	Dopri45 { h: f64, atol: f64, rtol: f64 },
}

/// Propagate trajectories
pub fn propagate(
	bodies: &[i32],
	small_bodies: &[i32],
	attractors: &[i32],
	cb_id: i32,
	t0: &str,
	tfinal: &str,
	solver: SolverConfig,
) -> Result<(Vec<Array1<f64>>, Vec<f64>), String> {
	println!(
		"Propagating trajectories of {} bodies from {} to {}",
		bodies.len(),
		t0,
		tfinal,
	);

	let et0 = spice::str2et(t0);
	let etfinal = spice::str2et(tfinal);
	if et0 >= etfinal {
		return Err("Start time is greater than end time".to_string());
	}

	// Initial conditions - retrieve state vectors from SPICE
	let y0 = spice_utils::states_at_instant(
		&bodies
			.iter()
			.cloned()
			.chain(small_bodies.iter().cloned())
			.collect::<Vec<_>>(),
		cb_id,
		et0,
	)?;

	// Retrieve standard gravitational parameters from SPICE
	let mut mus = vec![0f64; bodies.len() + small_bodies.len()];
	for (idx, &b) in bodies.iter().enumerate() {
		mus[idx] = spice_utils::mu(b)?;
	}

	// Bundle attractor mus and ids in tuples
	let mut attractors_with_mus = Vec::with_capacity(attractors.len());
	for &id in attractors {
		attractors_with_mus.push((id, spice_utils::mu(id)?));
	}

	// The actual derivative being integrated. Returns rate of change of system state
	let f =
		move |et: f64, y: &Array1<f64>| ode::n_body_ode(et, y, &mus, &attractors_with_mus, cb_id);

	// Create solver object based on config on the heap (since exact type is unknown)
	let mut solver: Box<dyn solvers::Solver> = match solver {
		SolverConfig::Rk4 { h } => Box::new(solvers::Rk4::new(f, h, et0, &y0, etfinal)),
		SolverConfig::Euler { h } => Box::new(solvers::Euler::new(f, h, et0, &y0, etfinal)),
		SolverConfig::Dopri45 { h, atol, rtol } => {
			Box::new(solvers::Dopri45::new(f, h, et0, &y0, etfinal, atol, rtol))
		}
	};

	// TODO: Approximate number of required steps to avoid reallocations
	let mut ets = Vec::new();
	let mut states = Vec::new();

	// Collect integral points
	while let Some((et, state)) = solver.next_state()? {
		ets.push(et);
		states.push(state);
	}

	Ok((states, ets))
}
