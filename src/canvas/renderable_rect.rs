use super::{rect::Rect, decoration::Decoration};

pub struct RenderableRect {
    pub rect: Rect,
    pub decoration: Decoration
}

impl RenderableRect {
    pub fn new(rect: Rect, decoration: Decoration) -> RenderableRect {
        RenderableRect { rect, decoration }
    }
}