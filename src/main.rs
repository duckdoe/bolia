mod ui;

use ui::panels::Panel;
use macroquad::prelude::*;

use crate::ui::panels::Alignment;

fn window_conf() -> Conf {
    Conf {
        window_title: "Interface".to_owned(),
        sample_count: 6,
        ..Default::default()
    }
}

fn build_ui () {
    let left_panel = Panel::new(Alignment::TopLeft, 250.0, screen_height(), BLACK);
    let right_panel = Panel::new(Alignment::TopRight, 250.0, screen_height(), BLACK);
    let center_panel = Panel::new(Alignment::Center(screen_height() - 80.0), 100.0, 50.0, BLACK);

    left_panel.draw_panel();
    right_panel.draw_panel();
    center_panel.draw_panel();
}
#[macroquad::main(window_conf)]
async fn main() {

    loop {
        clear_background(WHITE);

        build_ui();
        next_frame().await;
    }
}
