#![feature(trait_alias)]
mod cli;
mod ivp_utils;
mod nbs;
mod propagate;
mod spice_utils;

use clap::Parser;

use cli::Args;
use nbs::NBodySystemData;
use propagate::propagate;

#[allow(clippy::too_many_arguments)]
fn propagate_to_spk(
	mk: &str,
	t0: &str,
	time: u64,
	dt: u32,
	bodies: &[i32],
	output_file: &str,
	fraction_to_save: f32,
	cb_id: i32,
) {
	println!(
		"Propagating interactions of {} bodies over {} days (dt={}min)",
		bodies.len(),
		time,
		dt
	);

	// Load included kernels
	spice::furnsh("spice/included.tm");

	let system = NBodySystemData::instant_from_spice(mk, &bodies, t0);

	let propagated = propagate(
		&system,
		&ivp_utils::solvers::rk4,
		time * 3600 * 24, // Days
		dt * 60,          // Minutes
	);

	spice_utils::write_to_spk(&propagated, output_file, cb_id, fraction_to_save);

	spice::unload("spice/included.tm")
}

fn get_naif_ids(bodies: &[&str]) -> Vec<i32> {
	bodies
		.iter()
		.map(|body| {
			body.parse::<i32>()
				.unwrap_or_else(|_| match spice::bodn2c(body) {
					(id, true) => id,
					(_, false) => panic!("Body '{}' not found in kernel pool", body),
				})
		})
		.collect()
}

fn main() -> Result<(), &'static str> {
	let args = std::env::args().collect::<Vec<String>>();
	if args.len() == 1 && args[0] != "--help" {
		// TODO: Parse propagation .toml
		return Err("Cannot yet read .toml propagation files");
	}
	let args = Args::parse();

	let ids = get_naif_ids(&args.bodies.split(", ").collect::<Vec<&str>>());

	propagate_to_spk(
		&args.mk,
		&args.t0,
		args.time,
		args.dt,
		&ids,
		&args.output_file,
		args.fts.unwrap_or(1.0),
		args.cb_id.unwrap_or(ids[0]),
	);

	Ok(())
}
