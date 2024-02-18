use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
#[derive(Clone)]
pub(crate) struct Metal {
    albedo: Vec3,
    fuzz: f32,
}
impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = Vec3::reflect(&ray_in.direction().unit_vector(), &rec.normal);
        *scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());
        *attenuation = self.albedo;
        scattered.direction().dot(&rec.normal) > 0.0
    }

    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}
impl Metal {
    pub(crate) fn new(albedo: Vec3, fuzz: f32) -> Metal {
        if fuzz < 1.0 {
            Metal { albedo, fuzz }
        } else {
            Metal { albedo, fuzz: 1.0 }
        }
    }
}