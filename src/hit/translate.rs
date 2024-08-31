use nalgebra::Vector3;

use crate::{aabb::AABB, ray::Ray, util::Interval};

use super::{HitRecord, Hittable};
#[derive(Debug, Clone)]

pub struct Translate {
    pub offset: Vector3<f64>,
    pub object: Box<Hittable>,
    pub bbox: AABB,
}

impl Translate {
    pub fn new(object: Hittable, offset: Vector3<f64>) -> Self {
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
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub bbox: AABB,
    pub object: Box<Hittable>,
}

impl RotateY {
    pub fn new(object: Hittable, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bbox();

        let mut min = Vector3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Vector3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x.max + (1 - i) as f64 * bbox.x.min;
                    let y = j as f64 * bbox.y.max + (1 - j) as f64 * bbox.y.min;
                    let z = k as f64 * bbox.z.max + (1 - k) as f64 * bbox.z.min;

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

                let mut p = rec.p;
                p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
                p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];

                // 将法线从对象空间变换到世界空间
                let mut normal = rec.normal;
                normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
                normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];


                rec.p = p;
                rec.normal = normal;
                Some(rec)
            }
            None => None,
        }
    }
    fn bbox(&self) -> &AABB {
        &self.bbox
    }
}
