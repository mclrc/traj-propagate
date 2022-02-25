use crate::ivp_utils;
use crate::ode;
use crate::spice_utils;

pub fn propagate(
	mk: &str,
	t0: &str,
	time: f64,
	h: f64,
	bodies: &[i32],
) -> (Vec<ndarray::Array2<f64>>, Vec<f64>) {
	println!(
		"Propagating interactions of {} bodies over {} days (dt={}min)",
		bodies.len(),
		time,
		h
	);

	// Load included kernels
	spice::furnsh("spice/included.tm");
	spice::furnsh(mk);

	let et0 = spice::str2et(t0);
	let y0 = spice_utils::states_at_instant(bodies, et0);

	let mus = bodies
		.iter()
		.map(|&b| spice_utils::get_gm(b))
		.collect::<Vec<f64>>();

	let (states, ets) = ivp_utils::solve_ivp(
		et0,
		&y0,
		move |_, y| ode::n_body_ode(y, &mus),
		|f_, xs_, ys_, dt_| ivp_utils::solvers::rk4(f_, xs_, ys_, dt_),
		time * 3600.0 * 24.0,
		h * 60.0,
	);

	spice::unload("spice/included.tm");
	spice::unload(mk);

	(states, ets)
}
