use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

pub fn serialize_to_pickle<T: Serialize>(object: &T, path: String) {
	let mut f = File::create(path).expect("Unable to create file");
	let serialized =
		serde_pickle::ser::to_vec(object, Default::default()).expect("Unable to serialize");

	f.write_all(&serialized).expect("Unable to write to file");
}

pub fn deserialize_from_pickle<'de, T: Deserialize<'de>>(path: String) -> T {
	let f = File::open(path).expect("Unable to open file");

	serde_pickle::de::from_reader(f, Default::default()).unwrap()
}
