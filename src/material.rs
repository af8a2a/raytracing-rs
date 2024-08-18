use nalgebra::Vector3;

use crate::{
    hit::{self, HitRecord},
    ray::Ray,
    util::{near_zero, random_f32, random_in_unit_sphere, reflect, reflectance, refract},
};
#[derive(Debug, Clone, Copy)]
pub enum Material {
    Diffuse(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Material {
    pub fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vector3<f32>)> {
        match self {
            Material::Diffuse(lambert) => lambert.scatter(ray, hit_record),
            Material::Metal(metal) => metal.scatter(ray, hit_record),
            Material::Dielectric(dielectric) => dielectric.scatter(ray, hit_record),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
    albedo: Vector3<f32>,
}

impl Lambertian {
    pub fn new(albedo: Vector3<f32>) -> Self {
        Self { albedo }
    }

    pub fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vector3<f32>)> {
        let mut scatter_direction = hit_record.normal + random_in_unit_sphere();
        if near_zero(&scatter_direction) {
            scatter_direction = hit_record.normal;
        }

        let scattered = Ray::new(hit_record.p, scatter_direction);
        let attenuation = self.albedo.clone();
        Some((scattered, attenuation))
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Metal {
    albedo: Vector3<f32>,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vector3<f32>, fuzz: f32) -> Self {
        Self { albedo, fuzz }
    }
    pub fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vector3<f32>)> {
        let reflected =
            random_in_unit_sphere() * self.fuzz + reflect(&ray.direction, &hit_record.normal).normalize();
        let scattered = Ray::new(hit_record.p, reflected);
        let attenuation = self.albedo.clone();
        if scattered.direction.dot(&hit_record.normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
    refraction_index: f32,
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Self {
        Self { refraction_index }
    }

    pub fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vector3<f32>)> {
        let attenuation = Vector3::new(1.0, 1.0, 1.0);
        let ri = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_dir = ray.direction.normalize();
        let cos_theta = (-unit_dir).dot(&hit_record.normal).abs().min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = ri * sin_theta > 1.0;
        let direction;
        if cannot_refract || reflectance(cos_theta, ri) > random_f32() {
            direction = reflect(&unit_dir, &hit_record.normal);
        } else {
            direction = refract(&unit_dir, &hit_record.normal, ri);
        };

        let scattered = Ray::new(hit_record.p, direction);
        Some((scattered, attenuation))
    }
}
