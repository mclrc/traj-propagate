use std::io::{self, Write};

pub fn update_progress_bar(n: usize, i: usize, length: usize, prefix: &str, suffix: &str) {
	let percent = ((i as f64 / n as f64) * 100.0).floor();
	let filled = length * i / n;
	let bar = "#".repeat(filled) + &("-".repeat(length - filled));

	print!(
		"\r{} [{}] {}% {}{}",
		prefix,
		bar,
		percent,
		suffix,
		" ".repeat(10)
	);
	io::stdout().flush().unwrap();

	if i == n {
		println!();
	}
}
