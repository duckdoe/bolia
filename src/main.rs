mod ui;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Interface".to_owned(),
        sample_count: 6,
        ..Default::default()
    }
}


#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(WHITE);
        next_frame().await;
    }
}
