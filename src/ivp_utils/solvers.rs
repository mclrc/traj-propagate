use ndarray::{ArrayBase, Dimension, OwnedRepr};

type Arr<D> = ArrayBase<OwnedRepr<f64>, D>;

pub fn rk4<D: Dimension>(
	f: &(impl Fn(&Arr<D>) -> Arr<D> + ?Sized),
	ys: &[Arr<D>],
	dx: f32,
) -> (Arr<D>, f32) {
	let y = &ys[ys.len() - 1];

	let dx_f64 = dx as f64;
	let k1 = f(y);
	let k2 = f(&(y + &k1 * dx_f64 * 0.5));
	let k3 = f(&(y + &k2 * dx_f64 * 0.5));
	let k4 = f(&(y + &k3 * dx_f64 * 1.0));

	let next_y = y + (k1 + k2 * 2.0 + k3 * 2.0 + k4) * dx_f64 / 6.0;

	(next_y, dx)
}
