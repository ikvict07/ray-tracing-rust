use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub(crate) struct Sphere {
    center: Vec3,
    radius: f32,
    material: Box<dyn Material>,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = ray.origin() - self.center;
        let a = ray.direction().squared_length();
        let half_b = oc.dot(&ray.direction());
        let c = oc.squared_length() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let mut temp = (-half_b - discriminant.sqrt()) / a;
        if !(temp < t_max && temp > t_min) {
            temp = (-half_b + discriminant.sqrt()) / a;
        }
        if temp < t_max && temp > t_min {
            rec.t = temp;
            rec.p = ray.point_at_parameter(rec.t);
            rec.normal = (rec.p - self.center) / self.radius;
            rec.material = self.material.clone_box();
            return true;
        }
        false
    }
}

impl Sphere {
    pub(crate) fn new(center: Vec3, radius: f32, material: Box<dyn Material>) -> Sphere {
        Sphere { center, radius, material}
    }
}