use crate::hittable::{HitRecord, Hittable};
use crate::lambertian::Lambertian;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HittableList {
    list: Vec<Box<dyn Hittable>>,
    list_size: usize,
}

impl Hittable for HittableList{
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord {
            p: Vec3::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            material: Box::new(Lambertian::new(Vec3::zero())),
        };
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for i in 0..self.list_size {
            if self.list[i].hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }
}

impl HittableList {
    pub(crate) fn new(list: Vec<Box<dyn Hittable>>) -> HittableList {
        let list_size = list.len();
        HittableList { list, list_size }
    }
}