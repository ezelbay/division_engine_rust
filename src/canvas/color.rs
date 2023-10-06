use division_math::Vector4;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Color32 {
    vec: Vector4
}

impl Color32 {
    pub fn from_rgb(r: f32, g: f32, b: f32) -> Color32 {
        Color32 {
            vec: Vector4::new(r, g, b, 1.)
        }
    }

    pub fn white() -> Color32 {
        Self::from_rgb(1., 1., 1.)
    }

    pub fn black() -> Color32 {
        Self::from_rgb(0., 0., 0.)
    }

    pub fn gray() -> Color32 {
        Self::from_rgb(0.5, 0.5, 0.5)
    }

    pub fn red() -> Color32 {
        Self::from_rgb(1., 0., 0.)
    }

    pub fn green() -> Color32 {
        Self::from_rgb(0., 1., 0.)
    }

    pub fn blue() -> Color32 {
        Self::from_rgb(0., 0., 1.)
    }

    pub fn purple() -> Color32 {
        Self::from_rgb(0.5, 0., 0.5)
    }

    pub fn with_alpha(self, alpha: f32) -> Color32 {
        let mut clone = self.clone();
        clone.vec.z = alpha;
        clone
    }
}

impl From<Color32> for Vector4 {
    fn from(value: Color32) -> Self {
        value.vec
    }
}