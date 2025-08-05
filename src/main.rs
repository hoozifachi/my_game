use macroquad::prelude::*;

#[derive(Debug)]
struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
}

impl Shape {
    fn collides_with(&self, other: &Self) -> bool {
        self.circle().overlaps_rect(&other.rect())
    }

    fn circle(&self) -> Circle {
        Circle {
            x: self.x,
            y: self.y,
            r: self.size / 2.0,
        }
    }

    fn rect(&self) -> Rect {
        Rect {
            x: self.x,
            y: self.y,
            w: self.size / 2.0,
            h: self.size / 2.0,
        }
    }
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

    let mut game_over = false;

    loop {
        let delta_time = get_frame_time();

        clear_background(DARKPURPLE);

        // Reset game if space is pressed
        if game_over && is_key_pressed(KeyCode::Space) {
            squares.clear();
            circle.x = screen_width() / 2.0;
            circle.y = screen_height() / 2.0;
            game_over = false;
        }

        // Display game over
        if game_over {
            let text = "GAME OVER!";
            let text_dimensions = measure_text(text, None, 50, 1.0);
            draw_text(
                text,
                screen_width() / 2.0 - text_dimensions.width / 2.0,
                screen_height() / 2.0,
                50.0,
                RED,
            );
        }

        if !game_over {
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
        }

        if squares.iter().any(|square| circle.collides_with(square)) {
            game_over = true;
        }

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
