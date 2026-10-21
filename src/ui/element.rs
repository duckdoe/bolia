use macroquad::{
    color::{Color, BLACK},
    shapes::draw_rectangle,
    text::{draw_text, get_default_font, measure_text},
};

use crate::ui::{canvas::Canvas, text::Text};

pub struct Size {
    pub w: f32,
    pub h: f32,
}

impl Size {
    pub fn new(w: f32, h: f32) -> Self {
        Self { w, h }
    }
}

pub trait Element {
    fn size(&self) -> Size;
    fn draw(&self, x: f32, y: f32);
}

pub struct Rectangle {
    w: f32,
    h: f32,
    color: Color,
}

impl Element for Rectangle {
    fn size(&self) -> Size {
        Size::new(self.w, self.h)
    }
    fn draw(&self, x: f32, y: f32) {
        draw_rectangle(x, y, self.w, self.h, self.color);
    }
}

impl Element for Text {
    fn size(&self) -> Size {
        let font = get_default_font();
        let size = measure_text(self.text, Some(&font), self.size as u16, 1.0);

        Size::new(size.width, size.height)
    }

    fn draw(&self, x: f32, y: f32) {
        draw_text(self.text, x, y, self.size, self.color);
    }
}

impl Element for Canvas<'_> {
    fn size(&self) -> Size {
        Size::new(self.w, self.h)
    }

    fn draw(&self, _x: f32, y: f32) {
        for object in &self.context.objects {
            object.draw();
        }
    }
}

impl Rectangle {
    pub fn new(w: f32, h: f32, color: Option<Color>) -> Self {
        if let Some(color) = color {
            Self { w, h, color }
        } else {
            Self { w, h, color: BLACK }
        }
    }
}
