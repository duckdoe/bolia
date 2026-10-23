use macroquad::{color::BLACK, shapes::draw_rectangle_lines};

use crate::scene::camera::Camera;

pub struct Pos {
    pub x: f32,
    pub y: f32,
}

pub trait CanvasObject {
    fn get_pos(&self) -> Pos;
    fn draw(&self, x: f32, y: f32);
}

impl CanvasObject for Camera {
    fn get_pos(&self) -> Pos {
        Pos {
            x: self.x,
            y: self.y,
        }
    }

    fn draw(&self, x: f32, y: f32) {
        let x = x - (self.w / 2.0);
        let y = y - (self.h / 2.0);

        draw_rectangle_lines(x, y, self.w, self.h, 1.0, BLACK);
    }
}
