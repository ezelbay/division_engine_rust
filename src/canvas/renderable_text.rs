use division_math::Vector2;

use super::color::Color32;

pub struct RenderableText {
    pub position: Vector2,
    pub color: Color32,
    pub text: String,
    pub font_size: f32
}