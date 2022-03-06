#![cfg(test)]

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

fn run_test_scenario(mk: &'static str, t0: &'static str, bodies: &[&'static str], dt: f64, h: f64) {
	let bodies = bodies
		.iter()
		.map(|&b| spice::bodn2c(b).0)
		.collect::<Vec<i32>>();

	let filepath = get_temp_filepath("/testspk.bsp");
	let file = std::path::Path::new(&filepath);
	delete_if_exists(file);

	let (states, ets) = crate::propagate::propagate(mk, &bodies, t0, dt, h);

	crate::spice_utils::write_to_spk(
		&filepath,
		&bodies,
		&states,
		&ets,
		bodies[bodies.len() - 1],
		0.05,
	);

	assert!(file.exists());
}

#[test]
fn maven_cruise() {
	run_test_scenario(
		"spice/maven_cruise.bsp",
		"2013-NOV-19 00:00:00",
		&["Sun", "Earth", "Jupiter Barycenter", "Mars", "Maven"],
		308.0,
		10.0,
	)
}
