use macroquad::prelude::*;

#[macroquad::main("MyGame")]
async fn main() {
    loop {
        clear_background(DARKGRAY);

        next_frame().await
    }
}
