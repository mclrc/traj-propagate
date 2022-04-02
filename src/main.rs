mod cli;
mod ode;
mod propagate;
mod run;
mod solvers;
mod spice_utils;
mod tests;

use clap::Parser;

fn main() -> Result<(), &'static str> {
	run::run(cli::Args::parse())
}
