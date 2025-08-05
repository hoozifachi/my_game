use macroquad::{prelude::*, rand::rand};

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
}

#[macroquad::main("MyGame")]
async fn main() {
    const MOVEMENT_SPEED: f32 = 200.0;

    rand::srand(miniquad::date::now() as u64);

    let mut squares = vec![];
    let mut circle = Shape {
        size: 32.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
    };

    loop {
        let delta_time = get_frame_time();

        clear_background(DARKPURPLE);

        // Handle keyboard input
        if is_key_down(KeyCode::Right) {
            circle.x += circle.speed * delta_time;
        }
        if is_key_down(KeyCode::Left) {
            circle.x -= circle.speed * delta_time;
        }
        if is_key_down(KeyCode::Down) {
            circle.y += circle.speed * delta_time;
        }
        if is_key_down(KeyCode::Up) {
            circle.y -= circle.speed * delta_time;
        }

        // Keep circle on the screen
        circle.x = clamp(
            circle.x,
            0.0 + (circle.size / 2.0),
            screen_width() - (circle.size / 2.0),
        );
        circle.y = clamp(
            circle.y,
            0.0 + (circle.size / 2.0),
            screen_height() - (circle.size / 2.0),
        );

        // Generate a new square
        if rand::gen_range(0, 99) >= 95 {
            let size = rand::gen_range(16.0, 64.0);
            squares.push(Shape {
                size,
                speed: rand::gen_range(50.0, 150.0),
                x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                y: -size,
            });
        }

        // Move squares
        for square in &mut squares {
            square.y += square.speed * delta_time;
        }

        // Remove squares below the bottom of the screen
        squares.retain(|square| square.y < screen_height() + square.size);

        // Draw everything
        draw_circle(circle.x, circle.y, circle.size / 2.0, YELLOW);
        for square in &squares {
            draw_rectangle(
                square.x - square.size / 2.0,
                square.y - square.size / 2.0,
                square.size,
                square.size,
                GREEN,
            );
        }

        next_frame().await
    }
}
