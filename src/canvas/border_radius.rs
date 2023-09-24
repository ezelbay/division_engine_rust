use division_math::Vector4;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct BorderRadius {
    tr_br_tl_bl: Vector4,
}

impl BorderRadius {
    pub fn none() -> BorderRadius {
        BorderRadius { tr_br_tl_bl: Vector4::zero() }
    }

    pub fn top_rigt_bottom_right_top_left_bottom_left(
        top_right: f32,
        bottom_right: f32,
        top_left: f32,
        bottom_left: f32,
    ) -> BorderRadius {
        BorderRadius {
            tr_br_tl_bl: Vector4 {
                x: top_right,
                y: bottom_right,
                z: top_left,
                w: bottom_left,
            },
        }
    }

    pub fn all(value: f32) -> BorderRadius {
        BorderRadius {
            tr_br_tl_bl: Vector4::all(value),
        }
    }

    pub fn top_bottom(top: f32, bottom: f32) -> BorderRadius {
        BorderRadius {
            tr_br_tl_bl: Vector4 {
                x: top,
                y: bottom,
                z: top,
                w: bottom,
            },
        }
    }

    pub fn left_right(left: f32, right: f32) -> BorderRadius {
        BorderRadius {
            tr_br_tl_bl: Vector4 {
                x: right,
                y: right,
                z: left,
                w: left,
            },
        }
    }
}

impl From<BorderRadius> for Vector4 {
    fn from(value: BorderRadius) -> Self {
        value.tr_br_tl_bl
    }
}