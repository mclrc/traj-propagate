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
	pub bodies: Option<String>,

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

impl Args {
	// Parse list string of body names/NAIF-IDs to Vec<i32> (containing NAIF-IDs)
	pub fn parse_body_list(&self) -> Vec<i32> {
		self
			.bodies
			.as_ref()
			.expect("Please provide a list of bodies")
			.split(", ")
			.map(|item| {
				item
					// Try parsing i32-NAIF-ID from string
					.parse::<i32>()
					// If parse fails, item is likely a string body name. Query SPICE for ID
					.unwrap_or_else(|_| match spice::bodn2c(item) {
						// ID successfully retrieved
						(id, true) => id,
						// ID was not found. Panic
						(_, false) => panic!("No body with name or id '{}' could be found", item),
					})
			})
			.collect()
	}
}
