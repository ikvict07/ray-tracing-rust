use crate::ray::Ray;
use crate::vec3::Vec3;

pub(crate) struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub(crate) fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f32, aspect: f32) -> Camera {
        let (u, v, w): (Vec3, Vec3, Vec3);


        let theta = vfov.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        w = (lookfrom - lookat).unit_vector();
        u = vup.cross(&w).unit_vector();
        v = w.cross(&u);

        Camera {
            origin: lookfrom,
            lower_left_corner: lookfrom - half_width * u - half_height * v  - w ,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin)
    }
}