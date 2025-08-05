use macroquad::prelude::*;

#[macroquad::main("MyGame")]
async fn main() {
    const MOVEMENT_SPEED: f32 = 200.0;
    const RADIUS: f32 = 16.0;

    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;

    loop {
        let delta_time = get_frame_time();

        clear_background(DARKPURPLE);

        if is_key_down(KeyCode::Right) {
            x += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Left) {
            x -= MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Down) {
            y += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Up) {
            y -= MOVEMENT_SPEED * delta_time;
        }

        draw_circle(x, y, RADIUS, YELLOW);

        x = clamp(x, 0.0 + RADIUS, screen_width() - RADIUS);
        y = clamp(y, 0.0 + RADIUS, screen_height() - RADIUS);

        next_frame().await
    }
}
