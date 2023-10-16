use ndarray::Array1;

#[rustfmt::skip]
pub mod step_fns {
	use ndarray::Array1;
	pub fn rk4(
		f: impl Fn(f64, &Array1<f64>) -> Result<Array1<f64>, String>,
		x: f64,
		y: &Array1<f64>,
		h: f64,
	) -> Result<(f64, Array1<f64>), String> {
		let k1 = f(x, y)?;
		let k2 = f(x + 0.5 * h, &(y + h * &k1*0.5))?;
		let k3 = f(x + 0.5 * h, &(y + h * &k2*0.5))?;
		let k4 = f(x + 1.0 * h, &(y + h * &k3*1.0))?;

		let next_y = y + (k1 + k2 * 2.0 + k3 * 2.0 + k4) * h / 6.0;

		Ok((x + h, next_y))
	}
	pub fn euler(
		f: impl Fn(f64, &Array1<f64>) -> Result<Array1<f64>, String>,
		x: f64,
		y: &Array1<f64>,
		h: f64,
	) -> Result<(f64, Array1<f64>), String> {
		let dy = f(x, y)?;
		let next_y = y + dy * h;
		Ok((x + h, next_y))
	}
	#[allow(clippy::too_many_arguments)]
	pub fn dopri(
		f: impl Fn(f64, &Array1<f64>) -> Result<Array1<f64>, String>,
		x: f64,
		y: &Array1<f64>,
		h: f64,
		atol: f64,
		rtol: f64,
		safety_fac: f64,
		max_relative_dh: f64,
	) -> Result<(f64, Array1<f64>, f64), String> {
		let k1 = f(x, y)?;
		let k2 = f(x + h*1.0/5.0,  &(y + h *  &k1*0.5))?;
		let k3 = f(x + h*3.0/10.0, &(y + h * (&k1*3.0/40.0       + &k2*9.0/40.0)))?;
		let k4 = f(x + h*4.0/5.0,  &(y + h * (&k1*44.0/45.0      + &k2*-56.0/15.0      + &k3*32.0/9.0)))?;
		let k5 = f(x + h*8.0/9.0,  &(y + h * (&k1*19372.0/6561.0 + &k2*-25360.0/2187.0 + &k3*64448.0/6561.0 + &k4*-212.0/792.0)))?;
		let k6 = f(x + h*1.0,      &(y + h * (&k1*9017.0/3168.0  + &k2*355.0/33.0      + &k3*46732.0/5247.0 + &k4*49.0/176.0     + &k5*-5103.0/18656.0)))?;
		let k7 = f(x + h*1.0,      &(y + h * (&k1*35.0/384.0     + &k3*500.0/1113.0    + &k4*125.0/192.0    + &k5*-2187.0/6784.0 + &k6*11.0/84.0)))?;

		let hiord_y = y + h * &(&k1*35.0/384.0     + &k3*500.0/1113.0   + &k4*125.0/192.0 + &k5*-2187.0/6784.0    + &k6 * 11.0/84.0);
		let loord_y = y + h * &(&k1*5179.0/57600.0 + &k3*7571.0/16695.0 + &k4*393.0/640.0 + &k5*-92097.0/339200.0 + &k6 * 187.0/2100.0 + &k7 * 1.0/40.0);
		let err = &hiord_y - &loord_y;

		let max_y_norm = hiord_y.dot(&hiord_y).sqrt().max(y.dot(y).sqrt());

		let tol_n = atol + rtol * max_y_norm;
		let err_normalized = err / tol_n;
		let err_norm = err_normalized.dot(&err_normalized).sqrt();

		let new_h_fac = ((1.0 / err_norm).powf(0.2) * safety_fac).max(1.0 / max_relative_dh).min(max_relative_dh);
		let h_new = new_h_fac * h;

		Ok((x + h, hiord_y, h_new))
	}
}

pub trait Solver {
	fn next_state(&mut self) -> Result<Option<(f64, Array1<f64>)>, String>;
}

pub struct Rk4<F> {
	f: F,
	h: f64,
	x: f64,
	y: Array1<f64>,
	xmax: f64,
}
impl<F> Rk4<F> {
	pub fn new(f: F, h: f64, x0: f64, y0: &Array1<f64>, xmax: f64) -> Self {
		Self {
			f,
			h,
			x: x0,
			y: y0.clone(),
			xmax,
		}
	}
}
impl<F> Solver for Rk4<F>
where
	F: Fn(f64, &Array1<f64>) -> Result<Array1<f64>, String>,
{
	fn next_state(&mut self) -> Result<Option<(f64, Array1<f64>)>, String> {
		if self.x >= self.xmax {
			return Ok(None);
		} else if self.x + self.h > self.xmax {
			self.h = self.xmax - self.x;
		}

		let p = step_fns::rk4(|x, y| (self.f)(x, y), self.x, &self.y, self.h)?;
		self.x = p.0;
		self.y = p.1.clone();

		Ok(Some(p))
	}
}

pub struct Euler<F> {
	f: F,
	x: f64,
	y: Array1<f64>,
	h: f64,
	xmax: f64,
}

impl<F> Euler<F> {
	pub fn new(f: F, h: f64, x0: f64, y0: &Array1<f64>, xmax: f64) -> Self {
		Self {
			f,
			x: x0,
			y: y0.clone(),
			h,
			xmax,
		}
	}
}

impl<F> Solver for Euler<F>
where
	F: Fn(f64, &Array1<f64>) -> Result<Array1<f64>, String>,
{
	fn next_state(&mut self) -> Result<Option<(f64, Array1<f64>)>, String> {
		if self.x >= self.xmax {
			return Ok(None);
		} else if self.x + self.h > self.xmax {
			self.h = self.xmax - self.x;
		}

		let p = step_fns::euler(|x, y| (self.f)(x, y), self.x, &self.y, self.h)?;
		self.x = p.0;
		self.y = p.1.clone();

		Ok(Some(p.clone()))
	}
}

pub struct Dopri45<F> {
	f: F,
	h: f64,
	x: f64,
	y: Array1<f64>,
	xmax: f64,
	atol: f64,
	rtol: f64,
	max_relative_dh: f64,
	safety_factor: f64,
}
impl<F> Dopri45<F> {
	#[allow(dead_code)]
	pub fn new(f: F, h: f64, x0: f64, y0: &Array1<f64>, xmax: f64, atol: f64, rtol: f64) -> Self {
		Self {
			f,
			h,
			x: x0,
			y: y0.clone(),
			xmax,
			atol,
			rtol,
			max_relative_dh: 1.2,
			safety_factor: 0.85,
		}
	}
}

impl<F> Solver for Dopri45<F>
where
	F: Fn(f64, &Array1<f64>) -> Result<Array1<f64>, String>,
{
	fn next_state(&mut self) -> Result<Option<(f64, Array1<f64>)>, String> {
		if self.x >= self.xmax {
			return Ok(None);
		} else if self.x + self.h > self.xmax {
			self.h = self.xmax - self.x;
		}

		let p = step_fns::dopri(
			|x, y| (self.f)(x, y),
			self.x,
			&self.y,
			self.h,
			self.atol,
			self.rtol,
			self.safety_factor,
			self.max_relative_dh,
		)?;
		self.x = p.0;
		self.y = p.1.clone();
		self.h = p.2;
		Ok(Some((p.0, p.1)))
	}
}
