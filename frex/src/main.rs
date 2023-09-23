use arr_macro::arr;
use rand::prelude::*;
use raylib::{ffi::DisableCursor, prelude::*};

const WINDOW_WIDTH: i32 = 1280 * 2;
const WINDOW_HEIGHT: i32 = 720 * 2;

fn main() {
    let (mut rl, thread) = raylib::init()
        // .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        // .resizable()
        .fullscreen()
        .title("Hello, world!")
        .build();

    let mut camera = Camera3D::perspective(
        Vector3::new(0.0, 3.0, 0.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        60.0,
    );

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        rl.update_camera(&mut camera, 3);

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::GREEN);
        {
            let mut d2 = d.begin_mode3D(camera);
            d2.draw_triangle3D(
                Vector3::new(-5.0, 0.0, -5.0),
                Vector3::new(5.0, 0.0, -5.0),
                Vector3::new(0.0, 0.0, 10.0),
                Color::PINK,
            );
        }
    }
}
