use crate::vec3::Vec3;

pub struct Ray {
    a: Vec3,
    b: Vec3,
}

impl Ray {
    pub(crate) fn origin(&self) -> Vec3 {
        self.a
    }

    pub(crate) fn direction(&self) -> Vec3 {
        self.b
    }

    pub(crate) fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.a + self.b * t
    }

    pub(crate) fn new(a: Vec3, b: Vec3) -> Ray {
        Ray { a: a, b: b }
    }

}