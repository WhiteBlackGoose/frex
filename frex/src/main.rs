use raylib::{ffi::DisableCursor, prelude::*};

const WINDOW_WIDTH: i32 = 900;
const WINDOW_HEIGHT: i32 = 900;

// https://stackoverflow.com/questions/16500656/which-color-gradient-is-used-to-color-mandelbrot-in-wikipedia
const WK_PALETTE: [Vector4; 5] = [
    Vector4::new(0f32, 7f32, 100f32, 255f32),     //
    Vector4::new(32f32, 107f32, 203f32, 255f32),  //
    Vector4::new(237f32, 255f32, 255f32, 255f32), //
    Vector4::new(255f32, 170f32, 0f32, 255f32),   //
    Vector4::new(0f32, 2f32, 0f32, 255f32),       //
];

const WK_POSITIONS: [f32; 5] = [0.16, 0.42, 0.6425, 0.8575, 1.0];

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Frex!")
        .build();

    rl.set_target_fps(120);

    let mut shader = rl.load_shader(&thread, None, Some("./julia.fs")).unwrap();
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
    let julia_param_loc = shader.get_shader_location("julia_param");
    let wk_colors_loc = shader.get_shader_location("wk_colors");
    let wk_pos_loc = shader.get_shader_location("wk_pos");
    shader.set_shader_value_v(wk_colors_loc, &WK_PALETTE);
    shader.set_shader_value_v(wk_pos_loc, &WK_POSITIONS);

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
        shader.set_shader_value(
            julia_param_loc,
            [
                -0.5251993f32 + rl.get_mouse_x() as f32 / rl.get_screen_width() as f32 / 50.0,
                -0.5251993f32 + rl.get_mouse_y() as f32 / rl.get_screen_height() as f32 / 50.0,
            ],
        );

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
