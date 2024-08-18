use nalgebra::Vector3;

use super::HitRecord;

pub struct Sphere {
    pub center: Vector3<f32>,
    pub radius: f32,
}

impl Sphere {
    pub fn hit(&self, ray: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
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
        if root <= t_min || t_max <= root {
            root = (h + sqrt_discriminant) / a;
            if root <= t_min || t_max <= root {
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
        };
        hit_record.set_face_normal(ray, outward_normal);
        Some(hit_record)
    }
}
