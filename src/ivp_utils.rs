type Arr<D> = ndarray::ArrayBase<ndarray::OwnedRepr<f64>, D>;

pub mod solvers {
	use super::Arr;
	pub fn rk4<D: ndarray::Dimension>(
		f: impl Fn(f64, &Arr<D>) -> Arr<D>,
		xs: &[f64],
		ys: &[Arr<D>],
		h: f64,
	) -> (Arr<D>, f64) {
		let x = xs[xs.len() - 1];
		let y = &ys[ys.len() - 1];

		let k1 = f(x, y);
		let k2 = f(x + 0.5 * h, &(y + &k1 * h * 0.5));
		let k3 = f(x + 0.5 * h, &(y + &k2 * h * 0.5));
		let k4 = f(x + 1.0 * h, &(y + &k3 * h * 1.0));

		let next_y = y + (k1 + k2 * 2.0 + k3 * 2.0 + k4) * h / 6.0;

		(next_y, x + h)
	}
}

pub fn update_progress_bar(n: usize, i: usize, length: usize, prefix: &str, suffix: &str) {
	use std::io::{self, Write};
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

/// Solve initial value problem
pub fn solve_ivp<D: ndarray::Dimension, F, S>(
	x0: f64,
	y0: &Arr<D>,
	f: F,
	solver: S,
	dx: f64,
	h: f64,
) -> (Vec<Arr<D>>, Vec<f64>)
where
	F: Fn(f64, &Arr<D>) -> Arr<D>,
	S: Fn(&F, &[f64], &[Arr<D>], f64) -> (Arr<D>, f64),
{
	// Approximate number of required steps
	let steps = (dx as f64 / h + 1.0) as usize;

	// Vec for integral values
	let mut ys = Vec::<Arr<D>>::with_capacity(steps);
	ys.push(y0.clone());

	// Vec for x-values
	let mut xs = Vec::<f64>::with_capacity(steps);
	xs.push(x0);

	// Print initally empty progress bar to stdout
	update_progress_bar(steps, 0, 50, "Progress:", "complete");

	while xs[xs.len() - 1] < x0 + dx {
		// Compute next integral value
		let (next_y, next_x) = solver(&f, &xs[..], &ys[..], h);
		ys.push(next_y);
		xs.push(next_x);

		// Every 50 steps, update progress bar
		if xs.len() % 50 == 0 {
			update_progress_bar(steps, xs.len(), 50, "Progress: ", "complete");
		}
	}

	// Update progress bar to show 100% completion
	update_progress_bar(steps, steps, 50, "Progress: ", "complete");

	// Return computed integral values and corresponding x-values
	(ys, xs)
}
