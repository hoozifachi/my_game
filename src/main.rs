use macroquad::prelude::*;

#[macroquad::main("MyGame")]
async fn main() {
    loop {
        clear_background(DARKBLUE);

        next_frame().await
    }
}
