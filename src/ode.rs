pub struct NBodyODE {
	pub mus: Vec<f64>,
}

fn a_grav(r: &ndarray::Array1<f64>, mu: f64) -> ndarray::Array1<f64> {
	(mu / (r.dot(r).sqrt().powf(3.0))) * r
}

impl NBodyODE {
	pub fn f(&self, state: &ndarray::Array2<f64>) -> ndarray::Array2<f64> {
		let n = self.mus.len();

		let mut accels = ndarray::Array2::<f64>::zeros((n, 3));

		for b1 in 0..n {
			for b2 in (b1 + 1)..n {
				let r = &state.slice(ndarray::s![b2, ..3]) - &state.slice(ndarray::s![b1, ..3]);

				let mut a1 = accels.slice_mut(ndarray::s![b1, ..]);
				a1 += &a_grav(&r, self.mus[b2]);

				let mut a2 = accels.slice_mut(ndarray::s![b2, ..]);
				a2 += &a_grav(&-r, self.mus[b1]);
			}
		}
		let vels = state.slice(ndarray::s![.., 3..]);

		ndarray::concatenate(ndarray::Axis(1), &[vels.view(), accels.view()]).unwrap()
	}
}
