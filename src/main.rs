#![feature(trait_alias)]
#[allow(dead_code)]
mod cli;
mod file_utils;
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
	let args = Args::parse();

	let system = if let Some(mk_name) = args.mk {
		if args.system.is_some() {
			println!("[!] Warning: Both a meta-kernel and trajectory data was provided; Trajectory data will be ignored.")
		}

		let bodies = naif_ids_from_list_string(
			&args
				.bodies
				.expect("When propagating based on SPICE, 'bodies' is required"),
		);

		nbs::NBodySystemData::instant_from_mk(
			&mk_name,
			&bodies,
			&args
				.t0
				.expect("When propagating based on SPICE, 't0' is required"),
		)
	} else if let Some(nbs_name) = args.system {
		file_utils::deserialize_from_pickle(nbs_name)
	} else {
		panic!("Neither a meta-kernel file nor a trajectory data file was provided");
	};

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

	let steps_to_skip = args.sts.unwrap_or(29);
	propagated.serialize_to_pickle(&args.output_file, steps_to_skip + 1);
}
