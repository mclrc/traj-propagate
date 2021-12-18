#![feature(trait_alias)]
#[allow(dead_code)]
mod cli;
mod ivp_utils;
mod nbs;
mod propagate;
mod spice_utils;

use clap::Parser;

use cli::Args;
use propagate::propagate;

fn main() {
	// Load included kernels
	spice::furnsh("spice/included.tm");

	// Parse CLI arguments
	let args = Args::parse();

	// Parse supplied body list string to Vec<i32>
	let bodies = args.parse_body_list();
	// Load initial state from SPICE
	let system = nbs::NBodySystemData::instant_from_spice(&args.mk, &bodies, &args.t0);

	println!(
		"Propagating interactions of {} bodies over {} days (dt={}min)",
		system.bodies.len(),
		args.time,
		args.dt
	);

	// Perform propagation
	let propagated = propagate(
		&system,
		&ivp_utils::solvers::rk4,
		args.time * 3600 * 24,
		args.dt * 60,
	);

	// Save results to SPK
	let cb_id = args.cb_id.unwrap_or(system.bodies[0]);
	spice_utils::write_to_spk(&propagated, &args.output_file, cb_id, 1.0);

	spice::unload("spice/included.tm");
}
