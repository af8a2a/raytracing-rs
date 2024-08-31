use std::{
    backtrace::Backtrace,
    f32::{consts::PI, INFINITY},
};

use nalgebra::{Vector2, Vector3};

use crate::{
    aabb::AABB,
    material::Material,
    onb::Onb,
    ray::Ray,
    util::{random_f32, Interval},
};

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
    pub fn new_with_motion(
        center: Vector3<f32>,
        motion_center: Vector3<f32>,
        radius: f32,
        material: Material,
    ) -> Self {
        let rvec = Vector3::new(radius, radius, radius);
        let box1 = AABB::new(center - rvec, center + rvec);
        let box2 = AABB::new(motion_center - rvec, motion_center + rvec);
        let bbox = AABB::merge(&box1, &box2);

        Self {
            center,
            radius,
            material,
            motion: None,
            bbox,
        }
    }
    fn get_sphere_uv(p: &Vector3<f32>) -> Vector2<f32> {
        let theta = f32::acos(-p.y);
        let phi = f32::atan2(-p.z, p.x) + PI;
        let u = phi / (2.0 * PI);
        let v = theta / PI;
        Vector2::new(u, v)
    }
    pub fn hit(&self, ray: &crate::ray::Ray, interval: &Interval) -> Option<HitRecord> {
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
        // assert_eq!((p-self.center).norm()-self.radius<0.0001, true);
        let outward_normal = (p - self.center) / self.radius;
        let uv = Self::get_sphere_uv(&outward_normal);
        let mut hit_record = HitRecord {
            t,
            p,
            normal: Vector3::default(),
            front_face: false,
            material: &self.material,
            uv,
        };
        hit_record.set_face_normal(ray, &outward_normal);

        if (p-self.center).norm() < self.radius {
            println!("hit sphere at {:?} with normal {:?}", p, outward_normal);
        }

        Some(hit_record)
    }

    pub fn sphere_center(&self, time: f32) -> Vector3<f32> {
        self.center + self.motion.unwrap_or(Vector3::zeros()) * time
    }

    pub fn pdf_value(&self, origin: &Vector3<f32>, direction: &Vector3<f32>) -> f32 {
        // 这个方法只适用于静止的球体。

        if self
            .hit(
                &Ray::new(origin.clone(), direction.clone()),
                &Interval::new(0.001, INFINITY),
            )
            .is_none()
        {
            return 0.0;
        }

        let cos_theta_max =
            (1.0 - self.radius * self.radius / (self.center - origin).norm_squared()).sqrt();
        let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);
        let res = 1.0 / solid_angle;
        // if res.is_nan() {
        //     println!("self.radius:{}", self.radius);
        //     println!("self.origin:{}", origin);
        //     println!("self.radius * self.radius:{}", self.radius * self.radius);
        //     println!(
        //         "(self.center - origin).norm_squared():{}",
        //         (self.center - origin).norm_squared()
        //     );
        //     println!(
        //         "self.radius * self.radius / (self.center - origin).norm_squared():{}",
        //         self.radius * self.radius / (self.center - origin).norm_squared()
        //     );
        //     // let backtrace=Backtrace::force_capture();
        //     // println!("{:#?}",backtrace);
        //     panic!()
        // }
        1.0 / solid_angle
    }
    pub fn random(&self, origin: &Vector3<f32>) -> Vector3<f32> {
        let direction = self.center - origin;
        let distance_squared = direction.norm_squared();
        let uvw = Onb::new_from_w(direction);
        uvw.local_v(Self::random_to_sphere(self.radius, distance_squared))
    }

    fn random_to_sphere(radius: f32, distance_squared: f32) -> Vector3<f32> {
        let r1 = random_f32();
        let r2 = random_f32();
        let z = 1.0 + r2 * ((1.0 - radius * radius / distance_squared).sqrt() - 1.0);

        let phi = 2.0 * PI * r1;
        let x = phi.cos() * (1.0 - z * z).sqrt();
        let y = phi.sin() * (1.0 - z * z).sqrt();

        Vector3::new(x, y, z)
    }
}
