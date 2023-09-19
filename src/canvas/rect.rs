use division_math::Vector2;

pub struct Rect {
    pub center: Vector2,
    pub extents: Vector2, // half size
}

impl Rect {
    pub fn from_center_and_size(center: Vector2, size: Vector2) -> Rect {
        Rect {
            center,
            extents: size * 0.5,
        }
    }

    pub fn size(&self) -> Vector2 {
        self.extents * 2.
    }

    pub fn area(&self) -> f32 {
        self.extents.x * 2. * self.extents.y * 2.
    }

    pub fn left_bottom(&self) -> Vector2 {
        self.center - self.extents
    }

    pub fn right_top(&self) -> Vector2 {
        self.center + self.extents
    }

    pub fn left_top(&self) -> Vector2 {
        Vector2::new(
            self.center.x - self.extents.x,
            self.center.y + self.extents.y,
        )
    }

    pub fn right_bottom(&self) -> Vector2 {
        Vector2::new(
            self.center.x + self.extents.x,
            self.center.y - self.extents.y,
        )
    }
}
