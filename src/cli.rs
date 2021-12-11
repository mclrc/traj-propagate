use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Args {
	#[clap(long, value_name = "FILE", help = "Meta-kernel file name")]
	pub mk: Option<String>,

	#[clap(long, value_name = "FILE", help = "Trajectory data file name")]
	pub system: Option<String>,

	#[clap(
		long,
		value_name = "UTC_TIMESTAMP",
		help = "Time at which to begin propagation"
	)]
	pub t0: Option<String>,

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
	pub bodies: Option<String>,

	#[clap(short, long, value_name = "FILE", help = "File to write results to")]
	pub output_file: String,

	#[clap(
		long,
		value_name = "NUM_STEPS",
		help = "Number of steps to skip between each saved one to reduce output file size"
	)]
	pub sts: Option<usize>,
}
