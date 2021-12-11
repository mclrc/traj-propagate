use ndarray::{ArrayBase, Dimension, OwnedRepr};

type Arr<D> = ArrayBase<OwnedRepr<f64>, D>;

pub fn rk4<D: Dimension>(
	f: &(impl Fn(&Arr<D>) -> Arr<D> + ?Sized),
	ys: &[Arr<D>],
	dx: f64,
) -> Arr<D> {
	let y = &ys[ys.len() - 1];

	let k1 = f(y);
	let k2 = f(&(y + &k1 * dx * 0.5));
	let k3 = f(&(y + &k2 * dx * 0.5));
	let k4 = f(&(y + &k3 * dx * 1.0));

	y + (k1 + k2 * 2.0 + k3 * 2.0 + k4) * dx / 6.0
}
