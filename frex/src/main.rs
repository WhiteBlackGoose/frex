use arr_macro::arr;
use colors::ColorizerHue;
use fractals::{Fractal, Mandelbrot};
use num::complex::Complex32;
use rand::prelude::*;
use raylib::{ffi::DisableCursor, prelude::*};

mod colors;
mod fractals;

const WINDOW_WIDTH: i32 = 200;
const WINDOW_HEIGHT: i32 = 200;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Hello, world!")
        .build();

    let mut camera = Camera2D {
        target: Vector2::new(0.0, 0.0),
        offset: Vector2::new(0.0, 0.0),
        rotation: 0.0,
        zoom: 1.0,
    };

    rl.set_target_fps(60);

    let mandel = Mandelbrot::new(ColorizerHue {});

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        {
            let mut d2 = d.begin_mode2D(camera);
            let width = d2.get_screen_width();
            let height = d2.get_screen_height();
            let (rw, rh, rx, ry) = (2.2, 2.2, -1.5, -1.1);
            for x in 0..width {
                for y in 0..height {
                    let fx = x as f32 / width as f32 * rw + rx;
                    let fy = y as f32 / height as f32 * rh + ry;
                    d2.draw_pixel(x, y, mandel.calc(Complex32::new(fx, fy)));
                }
            }
            d2.draw_fps(0, 0);
        }
    }
}
