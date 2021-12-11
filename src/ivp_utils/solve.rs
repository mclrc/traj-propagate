use super::progress_bar::update_progress_bar;
use ndarray::{ArrayBase, Dimension, OwnedRepr};

pub type Derivative<T> = dyn Fn(&T) -> T;
pub type Solver<T> = dyn Fn(&Derivative<T>, &[T], f64) -> T;
type Arr<D> = ArrayBase<OwnedRepr<f64>, D>;

pub fn solve_ivp<D: Dimension>(
	y0: &Arr<D>,
	f: &Derivative<Arr<D>>,
	solver: &Solver<Arr<D>>,
	t: u64,
	dt: f64,
) -> Vec<Arr<D>> {
	let t = t as f64;
	let steps = (t / dt + 1.0) as usize;

	let mut ys = Vec::<Arr<D>>::with_capacity(steps);
	ys.push(y0.clone());

	update_progress_bar(steps, 0, 50, "Progress:", "complete");

	let mut t_solved = 0.0;
	let mut i = 1;

	while t_solved < t {
		let next_y = solver(f, &ys[0..i], dt);
		ys.push(next_y);

		i += 1;
		t_solved += dt;

		if i % 50 == 0 {
			update_progress_bar(steps, i, 50, "Progress: ", "complete");
		}
	}

	update_progress_bar(steps, steps, 50, "Progress: ", "complete");

	ys
}
