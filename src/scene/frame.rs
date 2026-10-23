use macroquad::texture::Texture2D;

use crate::viewport::object::CanvasObject;

pub struct Frame<'a> {
    objects: &'a dyn CanvasObject,
    texture: Texture2D,
}
