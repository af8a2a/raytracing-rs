pub mod sphere;
use nalgebra::Vector3;

use crate::ray::Ray;
#[derive(Debug,Clone,Default)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub front_face: bool,
}
impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vector3<f32>) {
        self.front_face = ray.direction.dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
pub enum Hittable {
    Sphere(sphere::Sphere),
}

impl Hittable {
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self {
            Hittable::Sphere(sphere) => sphere.hit(ray, t_min, t_max),
        }
    }
}
