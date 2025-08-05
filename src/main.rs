use macroquad::prelude::*;

#[macroquad::main("MyGame")]
async fn main() {
    const SPEED: f32 = 5.0;

    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;

    loop {
        clear_background(DARKPURPLE);

        if is_key_down(KeyCode::Right) {
            x += SPEED;
        }
        if is_key_down(KeyCode::Left) {
            x -= SPEED;
        }
        if is_key_down(KeyCode::Down) {
            y += SPEED;
        }
        if is_key_down(KeyCode::Up) {
            y -= SPEED;
        }

        draw_circle(x, y, 16.0, YELLOW);

        next_frame().await
    }
}
