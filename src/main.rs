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
        draw_rectangle(0.0, 0.0, 250.0, screen_height(), BLACK);
        // draw_rectangle(screen_width() - 250.0, 0.0, 250.0, screen_height(), BLACK);
        draw_rectangle(1085.0, 0.0, 250.0, 250.0, BLACK);
        next_frame().await;
    }
}
