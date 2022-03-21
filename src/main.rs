mod cli;
mod ode;
mod propagate;
mod solvers;
mod spice_utils;
mod tests;

use clap::Parser;
use cli::Args;

fn main() -> Result<(), &'static str> {
	let args = Args::parse();

	fn get_naif_ids(bodies: &[String]) -> Vec<i32> {
		bodies
			.iter()
			.map(|b| {
				b.parse::<i32>().unwrap_or_else(|_| match spice::bodn2c(b) {
					(id, true) => id,
					(_, false) => panic!("Body '{}' not found in kernel pool", b),
				})
			})
			.collect::<Vec<i32>>()
	}

	// Replace names with corresponding NAIF-IDs
	let bodies = get_naif_ids(&args.bodies);
	let small_bodies = args
		.small_bodies
		.map(|sc| get_naif_ids(&sc))
		.unwrap_or_else(Vec::new);

	let (states, ets) = propagate::propagate(
		&args.mk,
		&bodies,
		&small_bodies,
		&args.t0,
		&args.tfinal,
		args.h,
		&args.method.unwrap_or_else(|| "rk4".into()),
	);

	spice_utils::write_to_spk(
		&args.output_file,
		&bodies
			.iter()
			.cloned()
			.chain(small_bodies.iter().cloned())
			.collect::<Vec<i32>>(),
		&states,
		&ets,
		args.cb_id.unwrap_or(bodies[0]),
		args.fts.unwrap_or(1.0),
	);

	Ok(())
}
