use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
#[derive(Clone)]
pub struct Dielectric {
    ref_idx: f32,
}

impl Dielectric {
    pub(crate) fn new(ref_idx: f32) -> Dielectric {
        Dielectric { ref_idx }
    }

    pub fn schlick(cosine: f32, ref_idx: f32) -> f32 {
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let outward_normal: Vec3;
        let reflected = Vec3::reflect(&ray_in.direction(), &rec.normal);
        let ni_over_nt: f32;

        *attenuation = Vec3::new(1.0, 1.0, 0.0);
        let mut refracted: Vec3 = Vec3::new(0.0, 0.0, 0.0);

        let reflect_prob: f32;
        let cosine: f32;
        if (ray_in.direction().dot(&rec.normal)) > 0.0 {
            outward_normal = -rec.normal;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * ray_in.direction().dot(&rec.normal) / ray_in.direction().length();
        } else {
            outward_normal = rec.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -ray_in.direction().dot(&rec.normal) / ray_in.direction().length();
        }
        if Vec3::refract(&ray_in.direction(), &outward_normal, ni_over_nt, &mut refracted) {
            reflect_prob = Dielectric::schlick(cosine, self.ref_idx);
        } else {
            *scattered = Ray::new(rec.p, reflected);
            reflect_prob = 1.0;
        }

        if rand::random::<f32>() < reflect_prob {
            *scattered = Ray::new(rec.p, reflected);
        } else {
            *scattered = Ray::new(rec.p, refracted);
        }
        true
    }

    fn clone_box(&self) -> Box<dyn Material> {
        Box::new((*self).clone())
    }
}