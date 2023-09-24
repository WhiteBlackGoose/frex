use arr_macro::arr;
use colors::ColorizerHue;
use fractals::{Fractal, Mandelbrot};
use num::complex::Complex32;
use rand::prelude::*;
use raylib::{ffi::DisableCursor, prelude::*};

mod colors;
mod fractals;

const WINDOW_WIDTH: i32 = 2000;
const WINDOW_HEIGHT: i32 = 1500;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Hello, world!")
        //.fullscreen()
        .build();

    let mut camera = Camera2D {
        target: Vector2::new(0.0, 0.0),
        offset: Vector2::new(0.0, 0.0),
        rotation: 0.0,
        zoom: 1.0,
    };

    rl.set_target_fps(260);

    let mandel = Mandelbrot::new(ColorizerHue {});
    let mut shader = rl
        .load_shader(&thread, None, Some("./mandelbrot.fs"))
        .unwrap();
    let mut text = rl
        .load_render_texture(
            &thread,
            rl.get_screen_width() as u32,
            rl.get_screen_height() as u32,
        )
        .unwrap();

    let resolution_loc = shader.get_shader_location("resolution");
    shader.set_shader_value_v(
        resolution_loc,
        &[rl.get_screen_width() as f32, rl.get_screen_height() as f32],
    );
    let pos_loc = shader.get_shader_location("pos");
    let size_loc = shader.get_shader_location("size");

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
        shader.set_shader_value(pos_loc, [rx as f32, ry as f32]);
        shader.set_shader_value(size_loc, [rw as f32, rh as f32]);

        let mut d = rl.begin_drawing(&thread);
        {
            let mut d2 = d.begin_texture_mode(&thread, &mut text);
            d2.draw_rectangle(
                0,
                0,
                d2.get_screen_width(),
                d2.get_screen_height(),
                Color::GRAY,
            );
        }
        d.clear_background(Color::BLACK);
        {
            let mut sh = d.begin_shader_mode(&shader);
            sh.draw_texture(&mut text, 0, 0, Color::GRAY);
        }
        d.draw_fps(0, 0);
    }
}
