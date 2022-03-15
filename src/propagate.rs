use crate::ode;
use crate::solvers;
use crate::spice_utils;

/// Propagate trajectories
///   `mk`: Meta-kernel file to load initial conditions
///   `bodies`: NAIF-IDs of the bodies to include
///   `t0`: J2000 epoch to begin propagation from
///   `dt`: Time period over which to propagate, in days
///   `h`: Time step size for integrator, in minutes
/// Returns tuple `(states, ts)` where
///   `states`: Calculated system states
///   `ts`: Corresponding J2000 epochs
pub fn propagate(
	mk: &str,
	bodies: &[i32],
	t0: &str,
	tfinal: &str,
	h: f64,
) -> (Vec<ndarray::Array1<f64>>, Vec<f64>) {
	println!(
		"Propagating trajectories of {} bodies from {} to {} (dt={}min)",
		bodies.len(),
		t0,
		tfinal,
		h
	);

	// Load included kernels
	spice::furnsh("spice/included.tm");
	// Load user-provided kernels
	spice::furnsh(mk);

	let et0 = spice::str2et(t0);
	let etfinal = spice::str2et(tfinal);
	assert!(etfinal > et0);

	// Initial conditions
	let y0 = spice_utils::states_at_instant(bodies, et0);

	// Retrieve standard gravitational parameters
	let mus = bodies
		.iter()
		.map(|&b| spice_utils::get_gm(b))
		.collect::<Vec<f64>>();

	let (ets, states) = solvers::Rk4::new(
		move |_: f64, y: &ndarray::Array1<f64>| ode::n_body_ode(y, &mus),
		h * 60.0,
		et0,
		&y0,
		etfinal,
	)
	.fold(
		(Vec::new(), Vec::new()),
		|(mut ets, mut states), (et, state)| {
			ets.push(et);
			states.push(state);
			(ets, states)
		},
	);

	// Unload kernels
	spice::unload("spice/included.tm");
	spice::unload(mk);

	(states, ets)
}
