use nalgebra::Vector3;

use crate::{material::Material, util::Interval};

use super::HitRecord;

pub struct Sphere {
    pub center: Vector3<f32>,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn hit(&self, ray: &crate::ray::Ray, interval: &Interval) -> Option<HitRecord> {
        let oc = self.center - ray.origin;
        let a = ray.direction.norm_squared();
        let h = ray.direction.dot(&oc);
        let c = oc.norm_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrt_discriminant = discriminant.sqrt();
        let mut root = (h - sqrt_discriminant) / a;
        if !interval.surrounds(root) {
            root = (h + sqrt_discriminant) / a;
            if !interval.surrounds(root) {
                return None;
            }
        }
        let t = root;
        let p = ray.at(t);
        let normal = (p - self.center) / self.radius;
        let outward_normal = (p - self.center) / self.radius;
        let mut hit_record = HitRecord {
            t,
            p,
            normal,
            front_face: false,
            material: self.material.clone(),
        };
        hit_record.set_face_normal(ray, outward_normal);
        Some(hit_record)
    }
}
