use macroquad::color::{Color, DARKGRAY};

pub struct Button {
    padding: f32,
    color: Color,
    text: &'static str,
}

impl Button {
    pub fn new(text: &'static str, padding: f32) -> Self {
        Self {
            text,
            padding,
            color: DARKGRAY,
        }
    }
}
