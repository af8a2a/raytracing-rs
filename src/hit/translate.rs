use nalgebra::Vector3;

use crate::{aabb::AABB, ray::Ray, util::Interval};

use super::{HitRecord, Hittable};
#[derive(Debug, Clone)]

pub struct Translate {
    pub offset: Vector3<f32>,
    pub object: Box<Hittable>,
    pub bbox: AABB,
}

impl Translate {
    pub fn new(object: Hittable, offset: Vector3<f32>) -> Self {
        let bbox = object.bbox().clone().add_vec(offset);
        Self {
            offset,
            object: Box::new(object),
            bbox,
        }
    }
    pub fn hit(&self, ray: &crate::ray::Ray, interval: &Interval) -> Option<HitRecord> {
        let offset_r = Ray::new_with_time(ray.origin - self.offset, ray.direction, ray.time);

        match self.object.hit(&offset_r, interval) {
            Some(mut rec) => {
                rec.p += self.offset;
                Some(rec)
            }
            None => None,
        }
    }

    pub fn bbox(&self) -> &AABB {
        &self.bbox
    }
}
#[derive(Debug, Clone)]

pub struct RotateY {
    pub sin_theta: f32,
    pub cos_theta: f32,
    pub bbox: AABB,
    pub object: Box<Hittable>,
}

impl RotateY {
    pub fn new(object: Hittable, angle: f32) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bbox().clone();

        let mut min = Vector3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
        let mut max = Vector3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f32 * bbox.x.max + (1 - i) as f32 * bbox.x.min;
                    let y = j as f32 * bbox.y.max + (1 - j) as f32 * bbox.y.min;
                    let z = k as f32 * bbox.z.max + (1 - k) as f32 * bbox.z.min;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;
                    let tester = Vector3::new(newx, y, newz);
                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }
        let bbox = AABB::new(min, max);
        Self {
            sin_theta,
            cos_theta,
            bbox,
            object: Box::new(object),
        }
    }

    pub fn hit(&self, ray: &crate::ray::Ray, interval: &Interval) -> Option<HitRecord> {
        let mut origin = ray.origin;
        let mut direction = ray.direction;

        origin.x = self.cos_theta * ray.origin.x - self.sin_theta * ray.origin.z;
        origin.z = self.sin_theta * ray.origin.x + self.cos_theta * ray.origin.z;

        direction.x = self.cos_theta * ray.direction.x - self.sin_theta * ray.direction.z;
        direction.z = self.sin_theta * ray.direction.x + self.cos_theta * ray.direction.z;

        let rotated_r = Ray::new_with_time(origin, direction, ray.time);

        match self.object.hit(&rotated_r, interval) {
            Some(mut rec) => {
                rec.p.x = self.cos_theta * rec.p.x + self.sin_theta * rec.p.z;
                rec.p.z = -self.sin_theta * rec.p.x + self.cos_theta * rec.p.z;

                rec.normal.x = self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z;
                rec.normal.z = -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z;
                Some(rec)
            }
            None => None,
        }
    }
    fn bbox(&self) -> &AABB {
        &self.bbox
    }
}
