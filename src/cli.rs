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
		value_name = "UTC_TIMESTAMP",
		help = "J2000 time to propagate up to"
	)]
	pub tfinal: String,

	#[clap(
		long,
		value_name = "NUM_MINUTES",
		help = "Timestep size for integration"
	)]
	pub h: f64,

	#[clap(
		long,
		required = true,
		value_delimiter = ',',
		require_value_delimiter = true,
		min_values = 2,
		help = "Comma-separated NAIF-IDs or body names"
	)]
	pub bodies: Vec<String>,

	#[clap(
		long,
		value_delimiter = ',',
		require_value_delimiter = true,
		min_values = 1,
		help = "Bodies to include whose gravitational pull/mass can be ignored (e. g. spacecraft)"
	)]
	pub small_bodies: Option<Vec<String>>,

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

	#[clap(long, value_name = "rk4|dopri45", help = "Integration method")]
	pub method: Option<String>,
}
