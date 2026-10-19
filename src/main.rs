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

    let mut col = Column::new();
    let mut row = Row::new();

    let rect = Rectangle::new(20.0, 20.0, None);
    let bigger_rect = Rectangle::new(40.0, 40.0, None);

    row.child(Element::Rect(rect.clone()));

    col.child(Element::Rect(rect))
       .gap(10.0)
       .child(Element::Rect(bigger_rect))
       .child(Element::Rect(rect.clone()))
       .align(Alignment::Center);

    row.child(Element::Column(col));

    loop {
        clear_background(WHITE);


        row.draw(screen_width()/2.0, screen_height()/2.0);
        next_frame().await;
    }
}
