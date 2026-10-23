use macroquad::{
    math::Rect,
    texture::{get_screen_data, Texture2D},
};

// A camera in this kind of setting is basically just a
// limit of how many things are drawn/shown on the screen
pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Camera {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    pub fn get_texture(&self) -> Texture2D {
        let rect = Rect::new(self.x, self.y, self.w, self.h);
        let full_screen_image = get_screen_data();
        let thumbnail = full_screen_image.sub_image(rect);

        Texture2D::from_image(&thumbnail)
    }
}
