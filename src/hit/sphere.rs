use std::{
    backtrace::Backtrace,
    f64::{consts::PI, INFINITY},
};

use nalgebra::{Vector2, Vector3};

use crate::{
    aabb::AABB,
    material::Material,
    onb::Onb,
    ray::Ray,
    util::{random_f64, Interval},
};

use super::HitRecord;
#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Vector3<f64>,
    pub radius: f64,
    pub material: Material,
    pub motion: Option<Vector3<f64>>,
    pub bbox: AABB,
}

impl Sphere {
    pub fn new(center: Vector3<f64>, radius: f64, material: Material) -> Self {
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
        center: Vector3<f64>,
        motion_center: Vector3<f64>,
        radius: f64,
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
    fn get_sphere_uv(p: &Vector3<f64>) -> Vector2<f64> {
        let theta = f64::acos(-p.y);
        let phi = f64::atan2(-p.z, p.x) + PI;
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
        let outward_normal = (p - self.center).normalize();
        let uv = Self::get_sphere_uv(&outward_normal);
        assert_eq!((p - self.center).norm() - self.radius < 0.0001, true);

        let mut hit_record = HitRecord {
            t,
            p,
            normal: Vector3::default(),
            front_face: false,
            material: &self.material,
            uv,
            trace: true,
        };
        hit_record.set_face_normal(ray, &outward_normal);

        Some(hit_record)
    }

    pub fn sphere_center(&self, time: f64) -> Vector3<f64> {
        self.center + self.motion.unwrap_or(Vector3::zeros()) * time
    }

    pub fn pdf_value(&self, origin: &Vector3<f64>, direction: &Vector3<f64>) -> f64 {
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
        if res.is_nan() {
            println!("self.radius:{}", self.radius);
            println!("self.origin:{}", origin);
            println!("self.radius * self.radius:{}", self.radius * self.radius);
            println!(
                "(self.center - origin).norm_squared():{}",
                (self.center - origin).norm_squared()
            );
            println!(
                "self.radius * self.radius / (self.center - origin).norm_squared():{}",
                self.radius * self.radius / (self.center - origin).norm_squared()
            );
            // let backtrace=Backtrace::force_capture();
            // println!("{:#?}",backtrace);
            panic!()
        }
        1.0 / solid_angle
    }
    pub fn random(&self, origin: &Vector3<f64>) -> Vector3<f64> {
        let direction = self.center - origin;
        let distance_squared = direction.norm_squared();
        let uvw = Onb::new_from_w(direction);
        uvw.local_v(Self::random_to_sphere(self.radius, distance_squared))
    }

    fn random_to_sphere(radius: f64, distance_squared: f64) -> Vector3<f64> {
        let r1 = random_f64();
        let r2 = random_f64();
        let z = 1.0 + r2 * ((1.0 - radius * radius / distance_squared).sqrt() - 1.0);

        let phi = 2.0 * PI * r1;
        let x = phi.cos() * (1.0 - z * z).sqrt();
        let y = phi.sin() * (1.0 - z * z).sqrt();

        Vector3::new(x, y, z)
    }
}



// #[derive(Clone,Debug)]

// pub struct Sphere {
//     center1: Vector3<f64>,
//     radius: f64,
//     mat: Material,
//     is_moving: bool,
//     center_vec: Vector3<f64>,
//     pub bbox: AABB,
// }

// impl Sphere {
//     pub fn new(center: Vector3<f64>, radius: f64, material: Material) -> Self {
//         let rvec = Vector3::new(radius, radius, radius);
//         Self {
//             center1: center,
//             radius,
//             mat: material,
//             is_moving: false,
//             center_vec: Vector3::default(),
//             bbox: AABB::new((center - rvec), (center + rvec)),
//         }
//     }

//     pub fn new_with_center2(
//         center1: Vector3<f64>,
//         center2: Vector3<f64>,
//         radius: f64,
//         material: Material,
//     ) -> Self {
//         let rvec = Vector3::new(radius, radius, radius);
//         let box1 = AABB::new((center1 - rvec), (center1 + rvec));
//         let box2 = AABB::new((center2 - rvec), (center2 + rvec));
//         Self {
//             center1,
//             radius,
//             mat: material,
//             is_moving: true,
//             center_vec: center2 - center1,
//             bbox: AABB::merge(&box1, &box2),
//         }
//     }

//     fn sphere_center(&self, time: f64) -> Vector3<f64> {
//         self.center1 + self.center_vec * time
//     }

//     fn get_sphere_uv(p: Vector3<f64>) -> (f64, f64) {
//         // p: a given point on the sphere of radius one, centered at the origin.
//         // u: returned value [0,1] of angle around the Y axis from X=-1.
//         // v: returned value [0,1] of angle from Y=-1 to Y=+1.
//         //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
//         //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
//         //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>

//         let theta = (-p.y).acos();
//         let phi = (-p.z).atan2(p.x) + PI;

//         (phi / (2.0 * PI), theta / PI)
//     }

//     fn random_to_sphere(radius: f64, distance_squared: f64) -> Vector3<f64> {
//         let r1 = random_f64();
//         let r2 = random_f64();
//         let z = 1.0 + r2 * ((1.0 - radius * radius / distance_squared).sqrt() - 1.0);

//         let phi = 2.0 * PI * r1;
//         let x = phi.cos() * (1.0 - z * z).sqrt();
//         let y = phi.sin() * (1.0 - z * z).sqrt();

//         Vector3::new(x, y, z)
//     }
// }

// impl Sphere {
//     pub fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
//         let center = if self.is_moving {
//             self.sphere_center(r.time)
//         } else {
//             self.center1
//         };
//         let oc = center - r.origin;
//         let a = r.direction.norm_squared();
//         let h = r.direction.dot(&oc);
//         let c = oc.norm_squared() - self.radius * self.radius;

//         let discriminant = h * h - a * c;
//         if discriminant < 0.0 {
//             return None;
//         }
//         let sqrtd = discriminant.sqrt();

//         // Find the nearest root that lies in the acceptable range.
//         let mut root = (h - sqrtd) / a;
//         if !ray_t.surrounds(root) {
//             root = (h + sqrtd) / a;
//             if !ray_t.surrounds(root) {
//                 return None;
//             }
//         }

//         let p = r.at(root);
//         assert_eq!((p - self.center1).norm() - self.radius < 0.0001, true);

//         let outward_normal = (p - self.center1) / self.radius;
//         let (u, v) = Self::get_sphere_uv(outward_normal);

//         let mut hit_record = HitRecord {
//             p,
//             normal: Vector3::default(),
//             material: &self.mat,
//             t: root,
//             uv: Vector2::new(u, v),
//             front_face: false,
//         };
//         hit_record.set_face_normal(r, &outward_normal);
//         Some(hit_record)
//     }

//     pub fn bounding_box(&self) -> &AABB {
//         &self.bbox
//     }

//     pub fn pdf_value(&self, origin: &Vector3<f64>, direction:& Vector3<f64>) -> f64 {
//         // 这个方法只适用于静止的球体。

//         if self
//             .hit(
//                 &Ray::new(origin.clone(), direction.clone()),
//                 &Interval::new(0.001, INFINITY),
//             )
//             .is_none()
//         {
//             return 0.0;
//         }

//         let cos_theta_max =
//             (1.0 - self.radius * self.radius / (self.center1 - origin).norm_squared()).sqrt();
//         assert_eq!(cos_theta_max.is_nan(), false);
//         let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);
//         // eprintln!("self.center1 - origin.length_squared(): {:?}", (self.center1 - origin).length_squared());
//         1.0 / solid_angle
//     }

//     pub fn random(&self, origin: &Vector3<f64>) -> Vector3<f64> {
//         let direction = self.center1 - origin;
//         let distance_squared = direction.norm_squared();
//         let uvw = Onb::new_from_w(direction);
//         uvw.local_v(Self::random_to_sphere(self.radius, distance_squared))
//     }
// }
