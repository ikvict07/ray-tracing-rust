mod vec3;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod camera;
mod material;
mod lambertian;
mod metal;
mod dielectric;

use std::fs::File;
use std::io::Write;
use rayon::prelude::IntoParallelIterator;
use vec3::Vec3;
use ray::Ray;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use crate::camera::Camera;
use crate::lambertian::Lambertian;
use crate::metal::Metal;
use crate::dielectric::Dielectric;
use rayon::prelude::ParallelIterator;
use std::sync::Arc;

fn main() {
    generate_gradient();
}

fn generate_gradient() {
    let mut file = File::create("output.ppm").unwrap();
    let nx = 1280;
    let ny = 720;
    let ns = 300;

    file.write_all(b"P3\n").unwrap();
    {
        let content = format!("{} {}\n255\n", nx, ny);
        file.write_all(content.as_bytes()).unwrap();
    }

    let world = Arc::new(random_scene());

    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);


    let cam = Arc::new(Camera::new(lookfrom, lookat, Vec3::new(0.0, 1.0, 0.0), 20.0, nx as f32 / ny as f32));

    let data: Vec<_> = (0..ny).into_par_iter().flat_map(|j| {
        let cam = Arc::clone(&cam);
        let world = Arc::clone(&world); // Clone the world variable
        (0..nx).into_par_iter().map(move |i| {
            let mut col = Vec3::zero();

            for _ in 0..ns {
                let u = (i as f32 + rand::random::<f32>()) / nx as f32;
                let v = ((ny - j) as f32 + rand::random::<f32>()) / ny as f32; // Adjusted calculation of v
                let r = cam.get_ray(u, v);
                col += color(&r, &*world, 0);
            }

            col /= ns as f32;
            col = Vec3::new(col.x().sqrt(), col.y().sqrt(), col.z().sqrt());
            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;

            format!("{} {} {}\n", ir, ig, ib)
        })
    }).collect();

    for line in data {
        file.write_all(line.as_bytes()).unwrap();
    }
}

pub(crate) fn color(ray: &Ray, world: &dyn Hittable, depth: i32) -> Vec3 {
    let mut rec = HitRecord {
        p: Vec3::zero(),
        normal: Vec3::zero(),
        t: 0.0,
        material: Box::new(Lambertian::new(Vec3::zero())),
    };
    if world.hit(ray, 0.001, f32::MAX, &mut rec) {
        let mut scattered = Ray::new(Vec3::zero(), Vec3::zero());
        let mut attenuation = Vec3::zero();
        return if depth < 50 && rec.material.scatter(ray, &rec, &mut attenuation, &mut scattered) {
            attenuation * color(&scattered, world, depth + 1)
        } else {
            Vec3::zero()
        };
    }
    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
}

pub fn random_scene() -> HittableList {
    let mut list: Vec<Box<dyn Hittable>> = Vec::new();
    list.push(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))))));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<f32>();
            let center = Vec3::new(a as f32 + 0.9 * rand::random::<f32>(), 0.2, b as f32 + 0.9 * rand::random::<f32>());
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    list.push(Box::new(Sphere::new(center, 0.2, Box::new(Lambertian::new(Vec3::new(rand::random::<f32>() * rand::random::<f32>(), rand::random::<f32>() * rand::random::<f32>(), rand::random::<f32>() * rand::random::<f32>()))))));
                } else if choose_mat < 0.95 {
                    list.push(Box::new(Sphere::new(center, 0.2, Box::new(Metal::new(Vec3::new(0.5 * (1.0 + rand::random::<f32>()), 0.5 * (1.0 + rand::random::<f32>()), 0.5 * (1.0 + rand::random::<f32>())), 0.5 * rand::random::<f32>())))));
                } else {
                    list.push(Box::new(Sphere::new(center, 0.2, Box::new(Dielectric::new(1.5)))));
                }
            }
        }
    }
    list.push(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Box::new(Dielectric::new(1.5)))));
    list.push(Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, Box::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))))));
    list.push(Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)))));
    return HittableList::new(list);
}
