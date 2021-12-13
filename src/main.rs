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

fn naif_ids_from_list_string(string: &str) -> Vec<i32> {
	string
		.split(", ")
		.map(|item| {
			item
				.parse::<i32>()
				.unwrap_or_else(|_| spice_utils::id_for_label(item))
		})
		.collect()
}

fn main() {
	spice::furnsh("spice/included.mk");

	let args = Args::parse();

	let bodies = naif_ids_from_list_string(
		&args
			.bodies
			.expect("When propagating based on SPICE, 'bodies' is required"),
	);

	let system = nbs::NBodySystemData::instant_from_mk(
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

	let propagated = propagate(
		&system,
		&ivp_utils::solvers::rk4,
		args.time * 3600 * 24,
		args.dt * 60,
	);

	let cb_id = args.cb_id.unwrap_or(system.bodies[0]);
	spice_utils::write_to_spk(&propagated, &args.output_file, cb_id, 1.0);

	spice::furnsh("spice/included.mk");
}
