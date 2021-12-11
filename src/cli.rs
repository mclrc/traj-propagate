use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Args {
	#[clap(required = true, long, value_name = "FILE", help = "Meta-kernel file")]
	pub mk: String,

	#[clap(
		required = true,
		long,
		required = true,
		value_name = "UTC_TIMESTAMP",
		help = "Time at which to begin propagation"
	)]
	pub t0: String,

	#[clap(
		required = true,
		short,
		value_name = "NUM_DAYS",
		help = "Time period over which to integrate"
	)]
	pub t: u64,

	#[clap(
		required = true,
		long,
		value_name = "NUM_MINUTES",
		help = "Timestep size for numerical integration"
	)]
	pub dt: u32,

	#[clap(
		required = true,
		long,
		value_name = "BODY_LIST",
		help = "String containing comma-separated NAIF-IDs or body names"
	)]
	pub bodies: String,

	#[clap(short, long, value_name = "FILE", help = "File to write results to")]
	pub output_file: Option<String>,

	#[clap(
		long,
		value_name = "NUM_STEPS",
		help = "Number of steps to skip between each saved one to reduce output file size"
	)]
	pub sts: Option<usize>,
}
