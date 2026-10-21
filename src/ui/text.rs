use macroquad::color::Color;

pub struct Text {
    pub text: &'static str,
    pub size: f32,
    pub color: Color,
}

impl Text {
    pub fn new(text: &'static str, size: f32, color: Color) -> Self {
        Self { text, size, color }
    }
}
