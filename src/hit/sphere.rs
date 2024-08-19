use nalgebra::Vector3;

use crate::{bvh::AABB, material::Material, util::Interval};

use super::HitRecord;
#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Vector3<f32>,
    pub radius: f32,
    pub material: Material,

    pub motion: Option<Vector3<f32>>,
    pub bbox: AABB,
}

impl Sphere {
    pub fn new(center: Vector3<f32>, radius: f32, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
            motion: None,
            bbox: AABB::new(
                center - Vector3::new(radius, radius, radius),
                center + Vector3::new(radius, radius, radius),
            ),
        }
    }
    pub fn new_with_motion(center: Vector3<f32>,motion_center:Vector3<f32>, radius: f32, material: Material) -> Self {
        let rvec=Vector3::new(radius,radius,radius);
        let box1=AABB::new(center-rvec,center+rvec);
        let box2=AABB::new(motion_center-rvec,motion_center+rvec);
        let bbox=AABB::merge(&box1, &box2);

        Self {
            center,
            radius,
            material,
            motion: None,
            bbox,
        }
    }

    pub fn hit(&self, ray: &crate::ray::Ray, interval: &Interval) -> Option<HitRecord> {
        // if !self.bbox.hit(ray, interval){
        //     return None;
        // }
        let center = match self.motion {
            Some(_) => self.sphere_center(ray.time),
            None => self.center,
        };
        let oc = center - ray.origin;
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
        let normal = (p - center) / self.radius;
        let outward_normal = (p - center) / self.radius;
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

    pub fn sphere_center(&self, time: f32) -> Vector3<f32> {
        self.center + self.motion.unwrap_or(Vector3::zeros()) * time
    }
}
