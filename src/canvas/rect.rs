use division_math::Vector2;

pub struct Rect {
    pub center: Vector2,
    pub extents: Vector2, // half size
}

impl Rect {
    pub fn from_center(center: Vector2, size: Vector2) -> Rect {
        Rect {
            center,
            extents: size * 0.5,
        }
    }

    pub fn from_top_right(top_right: Vector2, size: Vector2) -> Rect {
        let extents = size * 0.5;

        Rect {
            center: top_right - extents,
            extents,
        }
    }

    pub fn from_top_left(top_left: Vector2, size: Vector2) -> Rect {
        let extents = size * 0.5;

        Rect {
            center: Vector2::new(top_left.x + extents.x, top_left.y - extents.y),
            extents,
        }
    }

    pub fn from_bottom_right(bottom_right: Vector2, size: Vector2) -> Rect {
        let extents = size * 0.5;

        Rect {
            center: Vector2::new(bottom_right.x - extents.x, bottom_right.y + extents.y),
            extents,
        }
    }

    pub fn from_bottom_left(bottom_left: Vector2, size: Vector2) -> Rect {
        let extents = size * 0.5;

        Rect {
            center: bottom_left + extents,
            extents,
        }
    }

    pub fn size(&self) -> Vector2 {
        self.extents * 2.
    }

    pub fn area(&self) -> f32 {
        self.extents.x * 2. * self.extents.y * 2.
    }

    pub fn bottom_left(&self) -> Vector2 {
        self.center - self.extents
    }

    pub fn top_right(&self) -> Vector2 {
        self.center + self.extents
    }

    pub fn top_left(&self) -> Vector2 {
        Vector2::new(
            self.center.x - self.extents.x,
            self.center.y + self.extents.y,
        )
    }

    pub fn bottom_right(&self) -> Vector2 {
        Vector2::new(
            self.center.x + self.extents.x,
            self.center.y - self.extents.y,
        )
    }
}
