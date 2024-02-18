use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of range"),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of range"),
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self[0] + other[0],
            y: self[1] + other[1],
            z: self[2] + other[2],
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self[0] - other[0],
            y: self[1] - other[1],
            z: self[2] - other[2],
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self[0] * other[0],
            y: self[1] * other[1],
            z: self[2] * other[2],
        }
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self {
            x: self[0] / other[0],
            y: self[1] / other[1],
            z: self[2] / other[2],
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, other: f32) -> Self::Output {
        Self {
            x: self[0] * other,
            y: self[1] * other,
            z: self[2] * other,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, other: f32) -> Self::Output {
        Self {
            x: self[0] / other,
            y: self[1] / other,
            z: self[2] / other,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        Vec3 {
            x: self * other[0],
            y: self * other[1],
            z: self * other[2],
        }
    }
}


impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self[0] + other[0],
            y: self[1] + other[1],
            z: self[2] + other[2],
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self[0] - other[0],
            y: self[1] - other[1],
            z: self[2] - other[2],
        }
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x: self[0] / other[0],
            y: self[1] / other[1],
            z: self[2] / other[2],
        }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self[0] * other[0],
            y: self[1] * other[1],
            z: self[2] * other[2],
        }
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        *self = Self {
            x: self[0] * other,
            y: self[1] * other,
            z: self[2] * other,
        }
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
        *self = Self {
            x: self[0] / other,
            y: self[1] / other,
            z: self[2] / other,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self[0],
            y: -self[1],
            z: -self[2],
        }
    }
}

impl Vec3 {
    pub(crate) fn dot(&self, other: &Vec3) -> f32 {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2]
    }
    pub(crate) fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self[1] * other[2] - self[2] * other[1],
            y: self[2] * other[0] - self[0] * other[2],
            z: self[0] * other[1] - self[1] * other[0],
        }
    }
    pub(crate) fn length(&self) -> f32 {
        self.dot(self).sqrt()
    }
    pub(crate) fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    pub(crate) fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }
    pub(crate) fn squared_length(&self) -> f32 {
        self.dot(self)
    }

    pub(crate) fn x(&self) -> f32 {
        self.x
    }
    pub(crate) fn y(&self) -> f32 {
        self.y
    }
    pub(crate) fn z(&self) -> f32 {
        self.z
    }

    pub fn zero() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        *v - *n * 2.0 * v.dot(n)
    }
    pub fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32, refracted: &mut Vec3) -> bool {
        let uv = Vec3::unit_vector(v);
        let dt = uv.dot(n);
        let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
        if discriminant > 0.0 {
            *refracted = (uv - *n * dt) * ni_over_nt - *n * discriminant.sqrt();
            return true;
        }
        return false;
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::new(rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>()) * 2.0 - Vec3::new(1.0, 1.0, 1.0);
            if p.squared_length() < 1.0 {
                return p;
            }
        }
    }
}