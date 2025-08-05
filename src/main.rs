use macroquad::prelude::*;

#[macroquad::main("MyGame")]
async fn main() {
    loop {
        clear_background(DARKPURPLE);

        next_frame().await
    }
}
