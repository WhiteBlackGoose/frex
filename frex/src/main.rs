use arr_macro::arr;
use colors::ColorizerHue;
use fractals::{Fractal, Mandelbrot};
use num::complex::Complex32;
use rand::prelude::*;
use raylib::{ffi::DisableCursor, prelude::*};

mod colors;
mod fractals;

const WINDOW_WIDTH: i32 = 600;
const WINDOW_HEIGHT: i32 = 600;

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

    let (mut rw, mut rh, mut rx, mut ry) = (2.2, 2.2, -1.5, -1.1);
    while !rl.window_should_close() {
        let zc = 1.2;
        let zcc = (1.0 - 1.0 / zc) / 2.0;
        let mc = 0.1;
        (rx, ry, rw, rh) = match rl.get_key_pressed() {
            Some(KeyboardKey::KEY_EQUAL) => (rx + rw * zcc, ry + rh * zcc, rw / zc, rh / zc),
            Some(KeyboardKey::KEY_MINUS) => (rx - rw * zcc, ry - rh * zcc, rw * zc, rh * zc),
            Some(KeyboardKey::KEY_H) => (rx - rw * mc, ry, rw, rh),
            Some(KeyboardKey::KEY_J) => (rx, ry + rh * mc, rw, rh),
            Some(KeyboardKey::KEY_K) => (rx, ry - rh * mc, rw, rh),
            Some(KeyboardKey::KEY_L) => (rx + rw * mc, ry, rw, rh),
            _ => (rx, ry, rw, rh),
        };

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        {
            let mut d2 = d.begin_mode2D(camera);
            let width = d2.get_screen_width();
            let height = d2.get_screen_height();
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
