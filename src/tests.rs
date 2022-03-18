#![cfg(test)]
use super::*;
use ::serial_test::serial;

fn get_temp_filepath(fname: &str) -> String {
	let mut filepath = std::env::temp_dir()
		.into_os_string()
		.into_string()
		.expect("Failed to get temporary directory path");
	filepath.push_str(fname);
	filepath
}

fn delete_if_exists(file: &std::path::Path) {
	if file.exists() {
		std::fs::remove_file(&file).unwrap();
	}
}

fn run_test_scenario(
	mk: &'static str,
	t0: &'static str,
	bodies: &[&'static str],
	tfinal: &str,
	h: f64,
	method: &str,
) {
	let bodies = bodies
		.iter()
		.map(|&b| spice::bodn2c(b).0)
		.collect::<Vec<i32>>();

	let filepath = get_temp_filepath("/testspk.bsp");
	let file = std::path::Path::new(&filepath);
	delete_if_exists(file);

	let (states, ets) = propagate::propagate(mk, &bodies, t0, tfinal, h, method);

	spice_utils::write_to_spk(
		&filepath,
		&bodies,
		&states,
		&ets,
		bodies[bodies.len() - 1],
		0.1,
	);

	assert!(file.exists());
}

#[test]
#[serial]
fn maven_cruise_rk4() {
	run_test_scenario(
		"spice/maven_cruise.bsp",
		"2013-NOV-20",
		&["Sun", "Earth", "Jupiter Barycenter", "Mars", "Maven"],
		"2014-SEP-21",
		1000.0,
		"rk4",
	)
}

#[test]
#[serial]
fn voyager2_flyby_dopri45() {
	run_test_scenario(
		"spice/voyager2_flyby.bsp",
		"1978-JAN-23",
		&["Sun", "Earth", "Jupiter Barycenter", "Mars", "Voyager 2"],
		"1979-SEP-30",
		1000.0,
		"dopri45",
	)
}
