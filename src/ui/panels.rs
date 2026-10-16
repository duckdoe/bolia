use std::fmt::Debug;

use macroquad::color::Color;
use macroquad::math::Rect;
use macroquad::shapes::{draw_circle, draw_rectangle};
use macroquad::window::{screen_height, screen_width};

#[allow(unused)]
pub enum Alignment {
    TopRight,
    TopLeft,
    Center(f32),
    BottomRight,
    BottomLeft,
}

pub struct Panel {
    /* Panel, contains elements that it draws itself */
    /* Panel also can only be used relative to the screen */
    align: Alignment,
    width: f32,
    height: f32,
    color: Color,
}

impl Panel {
    pub fn new(align: Alignment, width: f32, height: f32, color: Color) -> Self {
        Self {
            align,
            width,
            height,
            color,
        }
    }

    pub fn draw_panel(&self) {
        match self.align {
            Alignment::TopLeft => {
                draw_rectangle(0.0, 0.0, self.width, self.height, self.color);
            }
            Alignment::TopRight => {
                let offset = screen_width() - self.width;
                draw_rectangle(offset, 0.0, self.width, self.height, self.color);
            }
            Alignment::BottomLeft => {
                draw_rectangle(0.0, screen_height(), self.width, self.height, self.color);
            }
            Alignment::BottomRight => {
                let width_offset = screen_width() - self.width;
                let height_offset = screen_height() - self.height;

                draw_rectangle(
                    width_offset,
                    height_offset,
                    self.width,
                    self.height,
                    self.color,
                );
            }
            Alignment::Center(y) => {
                /* Align center from the width and stuff */
                let width_offset = self.width / 2.0;
                let x_center = screen_width() / 2.0;
                let x = x_center - width_offset;

                draw_rounded_rectangle(x, y, self.width, self.height, self.color);
            }
        }
    }
}

fn draw_rounded_rectangle(x: f32, y: f32, w: f32, h: f32, color: Color) {
    let x_end = x + w;
    draw_rectangle(x, y, w, h, color);
    draw_circle(x, y + h / 2.0, h / 2.0, color);
    draw_circle(x_end, y + h / 2.0, h / 2.0, color);
}
