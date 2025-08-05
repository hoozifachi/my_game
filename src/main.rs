use macroquad::prelude::*;

#[macroquad::main("MyGame")]
async fn main() {
    loop {
        clear_background(LIME);

        next_frame().await
    }
}
