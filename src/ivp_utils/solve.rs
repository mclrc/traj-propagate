use super::progress_bar::update_progress_bar;
use ndarray::{ArrayBase, Dimension, OwnedRepr};

// Type aliases for cleaner function signatures
pub type Derivative<T> = dyn Fn(&T) -> T;
pub type Solver<T> = dyn Fn(&Derivative<T>, &[T], f32) -> (T, f32);
type Arr<D> = ArrayBase<OwnedRepr<f64>, D>;

// Solve initial value problem
pub fn solve_ivp<D: Dimension>(
	y0: &Arr<D>,
	f: &Derivative<Arr<D>>,
	solver: &Solver<Arr<D>>,
	t: u64,
	dt: f32,
) -> (Vec<Arr<D>>, Vec<f32>) {
	let t = t as f64;
	// Approximate number of required steps
	let steps = (t / dt as f64 + 1.0) as usize;

	// Vec for integral values
	let mut ys = Vec::<Arr<D>>::with_capacity(steps);
	ys.push(y0.clone());

	// Vec for time step sizes
	let mut dts = Vec::<f32>::with_capacity(steps);
	dts.push(0.0);

	// Print initally empty progress bar to stdout
	update_progress_bar(steps, 0, 50, "Progress:", "complete");

	let mut t_solved = 0.0;
	let mut i = 1;

	while t_solved < t {
		// Compute next integral value
		let (next_y, dt) = solver(f, &ys[0..i], dt);
		ys.push(next_y);
		dts.push(dt);

		i += 1;
		t_solved += dt as f64;

		// Every 50 steps, update progress bar
		if i % 50 == 0 {
			update_progress_bar(steps, i, 50, "Progress: ", "complete");
		}
	}

	// Update progress bar to show 100% completion
	update_progress_bar(steps, steps, 50, "Progress: ", "complete");

	// Return computed integral values and corresponding step sizes
	(ys, dts)
}
