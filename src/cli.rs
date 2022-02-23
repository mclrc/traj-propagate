use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Args {
	#[clap(long, value_name = "FILE", help = "Meta-kernel file name")]
	pub mk: String,

	#[clap(
		long,
		value_name = "UTC_TIMESTAMP",
		help = "Time at which to begin propagation"
	)]
	pub t0: String,

	#[clap(
		long,
		value_name = "NUM_DAYS",
		help = "Time period over which to integrate"
	)]
	pub time: u64,

	#[clap(
		long,
		value_name = "NUM_MINUTES",
		help = "Timestep size for numerical integration"
	)]
	pub dt: u32,

	#[clap(
		long,
		value_name = "BODY_LIST",
		help = "String containing comma-separated NAIF-IDs or body names"
	)]
	pub bodies: String,

	#[clap(short, long, value_name = "FILE", help = "File to write results to")]
	pub output_file: String,

	#[clap(
		long,
		value_name = "FRACTION",
		help = "Fraction of steps to save to SPK file. 1 saves every step, 0.5 every 2nd etc. Defaults to 1"
	)]
	pub fts: Option<f32>,

	#[clap(
		long,
		value_name = "NAIF_ID",
		help = "Observing body for SPK segments. Defaults to first body in list"
	)]
	pub cb_id: Option<i32>,
}
