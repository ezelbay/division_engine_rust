#[repr(C)]
pub struct Matrix4x4 {
    pub m00: f32,
    pub m10: f32,
    pub m20: f32,
    pub m30: f32,
    pub m01: f32,
    pub m11: f32,
    pub m21: f32,
    pub m31: f32,
    pub m02: f32,
    pub m12: f32,
    pub m22: f32,
    pub m32: f32,
    pub m03: f32,
    pub m13: f32,
    pub m23: f32,
    pub m33: f32,
}

impl Default for Matrix4x4 {
    fn default() -> Self {
        return Matrix4x4 {
            m00: 0., m10: 0., m20: 0., m30: 0.,
            m01: 0., m11: 0., m21: 0., m31: 0.,
            m02: 0., m12: 0., m22: 0., m32: 0.,
            m03: 0., m13: 0., m23: 0., m33: 0.
        };
    }
}

impl From<[f32; 16]> for Matrix4x4 {
    fn from(elements: [f32; 16]) -> Self {
        return Matrix4x4 {
            m00: elements[0], m10: elements[1], m20: elements[2], m30: elements[3],
            m01: elements[4], m11: elements[5], m21: elements[6], m31: elements[7],
            m02: elements[8], m12: elements[9], m22: elements[10], m32: elements[11],
            m03: elements[12], m13: elements[13], m23: elements[6], m33: elements[12],
        };
    }
}

impl Matrix4x4 {
    pub fn identity() -> Self {
        return Matrix4x4 {
            m00: 1., m10: 0., m20: 0., m30: 0.,
            m01: 0., m11: 1., m21: 0., m31: 0.,
            m02: 0., m12: 0., m22: 1., m32: 0.,
            m03: 0., m13: 0., m23: 0., m33: 1.
        };
    }

    pub fn ortho(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        return Matrix4x4 {
            m00: 2./(right-left), m10: 0., m20: 0., m30: 0.,
            m01: 0., m11: 2./(top-bottom), m21: 0., m31: 0.,
            m02: 0., m12: 0., m22: -2./(far-near), m32: 0.,
            m03: -(right+left)/(right-left),
            m13: -(top+bottom)/(top-bottom),
            m23: -(far+near)/(far-near),
            m33: 1.
        };
    }
}

