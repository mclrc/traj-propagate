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

#[allow(clippy::too_many_arguments)]
fn run_test_scenario(
	mk: &'static str,
	t0: &'static str,
	bodies: Option<&[&'static str]>,
	small_bodies: Option<&[&'static str]>,
	attractors: Option<&[&'static str]>,
	tfinal: &str,
	h: f64,
	method: &str,
	cb: Option<&str>,
) {
	let filepath = get_temp_filepath("/testspk.bsp");
	let file = std::path::Path::new(&filepath);
	delete_if_exists(file);

	run::run(cli::Args {
		mk: mk.to_string(),
		bodies: bodies.map(|bs| bs.iter().map(<_>::to_string).collect()),
		small_bodies: small_bodies.map(|bs| bs.iter().map(<_>::to_string).collect()),
		attractors: attractors.map(|bs| bs.iter().map(<_>::to_string).collect()),
		t0: t0.to_owned(),
		tfinal: tfinal.to_owned(),
		atol: Some(50000.0),
		h,
		method: Some(method.to_owned()),
		cb_id: cb.map(|b| spice::bodn2c(b).0),
		fts: Some(0.1),
		output_file: filepath.clone(),
	})
	.unwrap();

	assert!(file.exists());
}

#[test]
#[serial]
fn maven_cruise_rk4() {
	run_test_scenario(
		"spice/maven_cruise.bsp",
		"2013-NOV-20",
		Some(&["Sun", "Earth", "Jupiter Barycenter", "Mars"]),
		Some(&["Maven"]),
		None,
		"2014-SEP-21",
		1000.0,
		"rk4",
		None,
	)
}

#[test]
#[serial]
fn voyager2_flyby_dopri45() {
	run_test_scenario(
		"spice/voyager2_flyby.bsp",
		"1978-JAN-23",
		Some(&["Sun", "Earth", "Jupiter Barycenter", "Mars"]),
		Some(&["Voyager 2"]),
		None,
		"1979-SEP-30",
		1000.0,
		"dopri45",
		None,
	)
}

#[test]
#[serial]
fn spkattractors() {
	run_test_scenario(
		"spice/voyager2_flyby.bsp",
		"1978-JAN-23",
		None,
		Some(&["Voyager 2"]),
		Some(&["Sun", "Earth", "Jupiter Barycenter", "Mars"]),
		"1979-SEP-01",
		1000.0,
		"dopri45",
		Some("Sun"),
	)
}

#[test]
#[serial]
fn insuffdata() {
	let filepath = get_temp_filepath("/testspk.bsp");
	let file = std::path::Path::new(&filepath);
	delete_if_exists(file);

	match run::run(cli::Args {
		mk: "spice/voyager2_flyby.bsp".to_string(),
		atol: None,
		fts: None,
		output_file: filepath,
		t0: "2000-JAN-23".to_string(),
		bodies: None,
		small_bodies: Some(["Voyager 2"].iter().map(<_>::to_string).collect()),
		attractors: Some(
			["Sun", "Earth", "Jupiter Barycenter", "Mars"]
				.iter()
				.map(<_>::to_string)
				.collect(),
		),
		tfinal: "2005-SEP-01".to_string(),
		h: 1000.0,
		method: Some("dopri45".to_string()),
		cb_id: Some(spice::bodn2c("Sun").0),
	}) {
		Err(msg) => println!("{msg}"),
		Ok(_) => panic!("This should have failed"),
	}
	unsafe { spice::c::reset_c() }
}
