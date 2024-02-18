use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
pub struct HitRecord {
    pub(crate) p: Vec3,
    pub(crate) normal: Vec3,
    pub(crate) t: f32,
    pub(crate) material: Box<dyn Material>,
}
impl Clone for HitRecord {
    fn clone(&self) -> Self {
        Self {
            p: self.p,
            normal: self.normal,
            t: self.t,
            material: self.material.clone_box(),
        }
    }
}
pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}

