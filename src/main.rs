mod ui;

use macroquad::prelude::*;

use crate::ui::layout::{Alignment, Column, Element, Rectangle, Row};

fn window_conf() -> Conf {
    Conf {
        window_title: "Interface".to_owned(),
        sample_count: 6,
        ..Default::default()
    }
}


#[macroquad::main(window_conf)]
async fn main() {
    let mut row = Row::new();
    let rect = Rectangle::new(20.0, 20.0, Some(RED));
    let bigger_rect = Rectangle::new(20.0, 60.0, Some(RED));
    row.child(&rect).child(&bigger_rect).align(Alignment::Center);
    loop {
        clear_background(WHITE);

        row.draw(200.0, 200.0);
        next_frame().await;
    }
}
