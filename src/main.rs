mod ui;
mod scene;
mod viewport;

use macroquad::prelude::*;

use crate::{scene::camera::Camera, ui::element::Element, viewport::canvas::Canvas};

fn window_conf() -> Conf {
    Conf {
        window_title: "Interface".to_owned(),
        sample_count: 6,
        window_width: 1920,
        window_height: 1080,
        ..Default::default()
    }
}


#[macroquad::main(window_conf)]
async fn main() {
    let font = load_ttf_font("/home/fortune/programming/duckdoe/bolia/assets/font/Coolvetica Rg.otf")
        .await
        .unwrap();

    set_default_font(font);
    let w = screen_width();
    let h = screen_height();

    println!("width: {w}");
    println!("height: {h}");

    let mut canvas = Canvas::new(w, h);
    let camera = Camera::new(0.0, 0.0, 400.0, 400.0);
    canvas.context.add_object(&camera);
    loop {
        clear_background(WHITE);

        canvas.draw(0.0, 0.0);
        next_frame().await;
    }
}
