use super::{color::Color32, border_radius::BorderRadius};

#[derive(Clone, Copy)]
pub struct Decoration {
    pub color: Color32,
    pub border_radius: BorderRadius
}