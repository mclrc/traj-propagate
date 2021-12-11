#![feature(trait_alias)]
mod cli;
#[allow(dead_code)]
mod file_utils;
mod ivp_utils;
mod nbs;
mod ode;
mod propagate;
mod spice_utils;

use clap::Parser;

use cli::Args;
use propagate::propagate;

fn naif_ids_from_list_string(string: &str) -> Vec<i32> {
	string
		.split(", ")
		.map(|item| {
			item.parse::<i32>()
				.unwrap_or_else(|_| spice_utils::id_for_label(item))
		})
		.collect()
}

fn main() {
	let args = Args::parse();

	let bodies = naif_ids_from_list_string(&args.bodies);

	let system = nbs::NBodySystemData::instant_from_mk(&args.mk, &bodies, &args.t0);

	let propagated = propagate(
		&system,
		&ivp_utils::solvers::rk4,
		args.t * 3600 * 24,
		args.dt * 60,
	);

	if let Some(output_path) = args.output_file {
		let steps_to_skip = args.sts.unwrap_or(49);
		propagated.serialize_to_pickle(&output_path, steps_to_skip + 1);
	}
}
