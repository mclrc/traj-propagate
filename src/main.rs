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

// Parse list string of body names/NAIF-IDs to Vec<i32> (containing NAIF-IDs)
fn naif_ids_from_list_string(string: &str) -> Vec<i32> {
	string
		.split(", ")
		.map(|item| {
			item
				// Try parsing i32-NAIF-ID from string
				.parse::<i32>()
				// If parse fails, item is likely a string body name. Query SPICE for ID
				.unwrap_or_else(|_| match spice::bodn2c(item) {
					// ID successfully retrieved
					(id, true) => id,
					// ID was not found. Panic
					(_, false) => panic!("No body with name or id '{}' could be found", item),
				})
		})
		.collect()
}

fn main() {
	// Load included kernels
	spice::furnsh("spice/included.tm");

	// Parse CLI arguments
	let args = Args::parse();

	// Parse supplied body list string to Vec<i32>
	let bodies = naif_ids_from_list_string(
		&args
			.bodies
			.expect("When propagating based on SPICE, 'bodies' is required"),
	);

	// Load initial state from SPICE
	let system = nbs::NBodySystemData::instant_from_spice(
		&args.mk,
		&bodies,
		&args
			.t0
			.expect("When propagating based on SPICE, 't0' is required"),
	);

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

	spice::furnsh("spice/included.tm");
}
