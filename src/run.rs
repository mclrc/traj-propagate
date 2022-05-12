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
		atol,
		tfinal,
		h,
		method,
		output_file,
		fts,
	}: cli::Args,
) -> Result<(), String> {
	if bodies.is_none() && small_bodies.is_none() {
		return Err("Please provide at least one body".to_string());
	} else if bodies.is_some() && attractors.is_some() {
		return Err("'bodies' cannot affect trajectories of 'attractors' - Providing both would result in inconsistencies".to_string());
	} else if attractors.is_some() && cb_id.is_none() {
		return Err("--cb-id is requried when using --attractors".to_string());
	}
	// Load included kernels
	spice::furnsh("spice/included.tm");
	// Load user-provided kernels
	spice::furnsh(&mk);

	// Convert body name/NAIF-ID vectors or None values to NAIF-ID vectors
	let bodies = spice_utils::naif_ids(&bodies.unwrap_or_default())?;
	let small_bodies = spice_utils::naif_ids(&small_bodies.unwrap_or_default())?;
	let attractors = spice_utils::naif_ids(&attractors.unwrap_or_default())?;

	// This unwrap is okay because there are only two possible scenarios:
	// 		1. There is a combination of small_bodies and attractors,
	// 		   in which case 'bodies' is guaranteed to be Some by the guards
	//    2. There no small_bodies or attractors, meaning bodies can't be empty
	let cb_id = cb_id.unwrap_or_else(|| bodies[0]);

	// Create solver config based on CLI args
	let solver = match method.as_deref() {
		Some("rk4") | None => propagate::SolverConfig::Rk4 { h },
		Some("dopri45") => propagate::SolverConfig::Dopri45 {
			h,
			atol: atol.unwrap_or(50000.0),
			rtol: 0.0,
		},
		Some(method) => return Err(format!("Unknown method: {method}")),
	};

	// Propagate trajectories
	let (ets, states) = propagate::propagate(
		&bodies,
		&small_bodies,
		&attractors,
		cb_id,
		&t0,
		&tfinal,
		solver,
	)?;

	// Write propagated trajectories to new SPK kernel
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
	)?;

	// Cleanup - unload kernels
	spice::unload("spice/included.tm");
	spice::unload(&mk);

	Ok(())
}
