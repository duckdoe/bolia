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
    let font = load_ttf_font("/home/fortune/programming/duckdoe/bolia/assets/font/Coolvetica Rg.otf")
        .await
        .unwrap();

    set_default_font(font);

    loop {
        clear_background(WHITE);
        draw_text("Hello coolvetica", 500.0, 500.0, 14.0, BLACK);
        next_frame().await;
    }
}
