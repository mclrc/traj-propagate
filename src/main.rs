#![feature(trait_alias)]
mod cli;
mod ivp_utils;
mod ode;
mod propagate;
mod spice_utils;

use clap::Parser;
use cli::Args;

fn main() -> Result<(), &'static str> {
	let args = Args::parse();

	let bodies = args
		.bodies
		.iter()
		.map(|body| {
			body.parse::<i32>()
				.unwrap_or_else(|_| match spice::bodn2c(body) {
					(id, true) => id,
					(_, false) => panic!("Body '{}' not found in kernel pool", body),
				})
		})
		.collect::<Vec<i32>>();

	let (states, ets) = propagate::propagate(&args.mk, &args.t0, args.dt, args.h, &bodies);

	spice_utils::write_to_spk(
		&args.output_file,
		&bodies,
		&states,
		&ets,
		args.cb_id.unwrap_or_else(|| bodies[bodies.len() - 1]),
		args.fts.unwrap_or(1.0),
	);

	Ok(())
}
