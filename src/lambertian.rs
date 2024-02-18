use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
#[derive(Clone)]
pub struct Lambertian {
    albedo: Vec3,
}
impl Material for Lambertian {
    fn scatter(&self, _: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let target = rec.p + rec.normal + Vec3::random_in_unit_sphere();
        *scattered = Ray::new(rec.p, target - rec.p);
        *attenuation = self.albedo;
        true
    }

    fn clone_box(&self) -> Box<dyn Material> {
        Box::new((*self).clone())
    }
}

impl Lambertian {
    pub(crate) fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}