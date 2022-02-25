use crate::ivp_utils;
use crate::ode;
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
	dt: f64,
	h: f64,
) -> (Vec<ndarray::Array2<f64>>, Vec<f64>) {
	println!(
		"Propagating trajectories of {} bodies over {} days (dt={}min)",
		bodies.len(),
		dt,
		h
	);

	// Load included kernels
	spice::furnsh("spice/included.tm");
	// Load user-provided kernels
	spice::furnsh(mk);

	let et0 = spice::str2et(t0);
	// Initial conditions
	let y0 = spice_utils::states_at_instant(bodies, et0);

	// Retrieve standard gravitational parameters
	let mus = bodies
		.iter()
		.map(|&b| spice_utils::get_gm(b))
		.collect::<Vec<f64>>();

	let (states, ets) = ivp_utils::solve_ivp(
		et0,
		&y0,
		move |_, y| ode::n_body_ode(y, &mus),
		|f_, xs_, ys_, dt_| ivp_utils::solvers::rk4(f_, xs_, ys_, dt_),
		dt * 3600.0 * 24.0,
		h * 60.0,
	);

	// Unload kernels
	spice::unload("spice/included.tm");
	spice::unload(mk);

	(states, ets)
}
