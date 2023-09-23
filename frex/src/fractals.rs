use num::complex::ComplexFloat;

use crate::colors::Colorizer;

pub trait Fractal<C: ComplexFloat> {
    fn calc(&self, coords: C) -> raylib::color::Color;
}

pub struct Mandelbrot<R: Colorizer> {
    col: R,
}

impl<R: Colorizer> Mandelbrot<R> {
    pub fn new(col: R) -> Self {
        Mandelbrot { col }
    }
}

impl<R, C> Fractal<C> for Mandelbrot<R>
where
    R: Colorizer,
    C: ComplexFloat,
    C::Real: Into<f32>,
{
    fn calc(&self, coords: C) -> raylib::color::Color {
        let (max, iter) = count_iters(|z| z * z + coords, C::zero());
        self.col.from_iter(iter, max)
    }
}

pub struct Julia<R: Colorizer, P: ComplexFloat> {
    col: R,
    param: P,
}

impl<R, C> Fractal<C> for Julia<R, C>
where
    R: Colorizer,
    C: ComplexFloat,
    C::Real: Into<f32>,
{
    fn calc(&self, coords: C) -> raylib::color::Color {
        let (max, iter) = count_iters(|z| z * z + self.param, coords);
        self.col.from_iter(iter, max)
    }
}

fn count_iters<F, C: Copy + ComplexFloat>(step: F, init: C) -> (u32, u32)
where
    F: Fn(C) -> C,
    C::Real: Into<f32>,
{
    let max_prec = 150;
    let z = (0..max_prec).try_fold(init, |x, i| {
        let next = step(x);
        if next.abs().into() < 2.0f32 {
            Ok(next)
        } else {
            Err(i)
        }
    });
    match z {
        Ok(_) => (max_prec, max_prec),
        Err(i) => (max_prec, i),
    }
}
