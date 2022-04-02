use crate::ode;
use crate::solvers;
use crate::spice_utils;
use ndarray::Array1;

/// Propagate trajectories
#[allow(clippy::too_many_arguments)]
pub fn propagate(
	bodies: &[i32],
	small_bodies: &[i32],
	attractors: &[i32],
	cb_id: i32,
	t0: &str,
	tfinal: &str,
	h: f64,
	method: &str,
) -> (Vec<Array1<f64>>, Vec<f64>) {
	println!(
		"Propagating trajectories of {} bodies from {} to {} (dt={}min)",
		bodies.len(),
		t0,
		tfinal,
		h
	);

	let et0 = spice::str2et(t0);
	let etfinal = spice::str2et(tfinal);
	assert!(etfinal > et0);

	// Initial conditions
	let y0 = spice_utils::states_at_instant(
		&bodies
			.iter()
			.cloned()
			.chain(small_bodies.iter().cloned())
			.collect::<Vec<i32>>(),
		cb_id,
		et0,
	);

	// Retrieve standard gravitational parameters
	let mus = bodies
		.iter()
		.map(|&b| spice_utils::get_gm(b))
		.chain(std::iter::repeat(0.0).take(small_bodies.len()))
		.collect::<Vec<f64>>();

	let attractors_with_mus = attractors
		.iter()
		.map(|&b| (b, spice_utils::get_gm(b)))
		.collect::<Vec<_>>();

	let f =
		move |et: f64, y: &Array1<f64>| ode::n_body_ode(et, y, &mus, &attractors_with_mus, cb_id);

	let points: Vec<(f64, Array1<f64>)> = match method {
		"rk4" => solvers::Rk4::new(f, h * 60.0, et0, &y0, etfinal).collect(),
		"dopri45" => solvers::Dopri45::new(f, h * 60.0, et0, &y0, etfinal, 50000.0, 0.0).collect(),
		_ => unimplemented!("Unknown method"),
	};

	let n_states = points.len();
	let (ets, states) = points.into_iter().fold(
		(Vec::with_capacity(n_states), Vec::with_capacity(n_states)),
		|(mut ets, mut states), (et, state)| {
			ets.push(et);
			states.push(state);
			(ets, states)
		},
	);

	(states, ets)
}
