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
	let filepath = get_temp_filepath("/traj-propagate-test.bsp");
	let file = std::path::Path::new(&filepath);

	run::run(cli::Args {
		mk: mk.to_string(),
		bodies: bodies.map(|bs| bs.iter().map(<_>::to_string).collect()),
		small_bodies: small_bodies.map(|bs| bs.iter().map(<_>::to_string).collect()),
		attractors: attractors.map(|bs| bs.iter().map(<_>::to_string).collect()),
		t0: t0.to_string(),
		tfinal: tfinal.to_string(),
		atol: Some(50000f64),
		h,
		method: Some(method.to_string()),
		cb_id: cb.map(|b| spice::bodn2c(b).0),
		fts: Some(0.1f32),
		output_file: filepath.clone(),
	})
	.unwrap();

	assert!(file.exists());

	unsafe {
		spice::c::reset_c();
	}
	println!();
}

#[test]
#[serial]
fn maven_cruise_euler() {
	println!("starting euler test");
	run_test_scenario(
		"spice/maven_cruise.bsp",
		"2013-NOV-20",
		Some(&["Sun", "Earth", "Jupiter Barycenter", "Mars"]),
		Some(&["Maven"]),
		None,
		"2014-SEP-21",
		100f64,
		"euler",
		None,
	)
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
		1000f64,
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
		1000f64,
		"dopri45",
		None,
	)
}

#[test]
#[serial]
fn spk_attractors() {
	run_test_scenario(
		"spice/voyager2_flyby.bsp",
		"1978-JAN-23",
		None,
		Some(&["Voyager 2"]),
		Some(&["Sun", "Earth", "Jupiter Barycenter", "Mars"]),
		"1979-SEP-01",
		1000f64,
		"dopri45",
		Some("Sun"),
	)
}

#[test]
#[serial]
fn spice_errors() {
	assert!(spice_utils::state_at_instant(-202, 10, 0f64).is_err());
	assert!(spice_utils::mu(-202).is_err());
	assert!(spice_utils::naif_ids(&["doesnotexist"]).is_err());
}
