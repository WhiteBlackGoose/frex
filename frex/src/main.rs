use arr_macro::arr;
use rand::prelude::*;
use raylib::{ffi::DisableCursor, prelude::*};

mod colors;
mod fractals;

const WINDOW_WIDTH: i32 = 1280 * 2;
const WINDOW_HEIGHT: i32 = 720 * 2;

fn main() {
    let (mut rl, thread) = raylib::init()
        // .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        // .resizable()
        .fullscreen()
        .title("Hello, world!")
        .build();

    let mut camera = Camera2D::default();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::GREEN);
        {
            let mut d2 = d.begin_mode2D(camera);
            for x in 0..d2.get_screen_width() {
                for y in 0..d2.get_screen_height() {
                    d2.draw_pixel(x, y, color);
                }
            }
        }
    }
}
