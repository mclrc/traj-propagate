use crate::cli;
use crate::propagate;
use crate::spice_utils;

pub fn run(
	cli::Args {
		mk,
		cb_id,
		bodies,
		small_bodies,
		attractors,
		t0,
		tfinal,
		h,
		method,
		output_file,
		fts,
	}: cli::Args,
) -> Result<(), &'static str> {
	if bodies.is_none() && small_bodies.is_none() {
		return Err("Please provide at least one body");
	} else if bodies.is_some() && attractors.is_some() {
		return Err("'bodies' cannot affect trajectories of 'attractors' - Providing both would result in inconsistencies");
	}
	// Load included kernels
	spice::furnsh("spice/included.tm");
	// Load user-provided kernels
	spice::furnsh(&mk);

	let bodies = bodies
		.map(|bs| spice_utils::naif_ids(&bs))
		.unwrap_or_else(Vec::new);
	let small_bodies = small_bodies
		.map(|bs| spice_utils::naif_ids(&bs))
		.unwrap_or_else(Vec::new);
	let attractors = attractors
		.map(|bs| spice_utils::naif_ids(&bs))
		.unwrap_or_else(Vec::new);

	let cb_id = match attractors.is_empty() {
		false => cb_id.expect("-cb-id is required when using --attractors"),
		true => cb_id.unwrap_or(bodies[0]),
	};

	let (ets, states) = propagate::propagate(
		&bodies,
		&small_bodies,
		&attractors,
		cb_id,
		&t0,
		&tfinal,
		h,
		&method.unwrap_or_else(|| "rk4".into()),
	);

	spice_utils::write_to_spk(
		&output_file,
		&bodies
			.iter()
			.cloned()
			.chain(small_bodies.iter().cloned())
			.collect::<Vec<i32>>(),
		&ets,
		&states,
		cb_id,
		fts.unwrap_or(1.0),
	);

	// Unload kernels
	spice::unload("spice/included.tm");
	spice::unload(&mk);

	Ok(())
}
