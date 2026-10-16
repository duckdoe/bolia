use macroquad::prelude::*;

#[macroquad::main("egui with macroquad")]
async fn main() {
    loop {
        clear_background(WHITE);
        next_frame().await;
    }
}
