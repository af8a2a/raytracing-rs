use core::f64;

use nalgebra::Vector3;

use crate::{
    material::{Lambertian, Material},
    util::{random_f64, Interval, UNIVERSE_INTERVAL},
};

use super::Hittable;
#[derive(Debug, Clone)]

pub struct ConstMedium {
    pub boundary: Box<Hittable>,
    pub neg_density: f64,
    pub phase_function: Box<Material>,
}

impl ConstMedium {
    pub fn new(boundary: Hittable, density: f64, phase_function: Box<Material>) -> Self {
        Self {
            boundary: Box::new(boundary),
            neg_density: -1.0 / density,
            phase_function,
        }
    }
    pub fn new_with_color(boundary: Hittable, density: f64, color: Vector3<f64>) -> Self {
        Self {
            boundary: Box::new(boundary),
            neg_density: -1.0 / density,
            phase_function: Box::new(Material::Diffuse(Lambertian::new_with_color(color))),
        }
    }
    pub fn hit(&self, ray: &crate::ray::Ray, interval: &Interval) -> Option<crate::hit::HitRecord> {
        let mut rec1;
        match self.boundary.hit(ray, &UNIVERSE_INTERVAL) {
            Some(rec) => rec1 = rec,
            None => return None,
        }

        let mut rec2;
        match self
            .boundary
            .hit(ray, &Interval::new(rec1.t + 0.0001, f64::INFINITY))
        {
            Some(rec) => rec2 = rec,
            None => return None,
        }
        if rec1.t < interval.min {
            rec1.t = interval.min;
        }
        if rec2.t > interval.max {
            rec2.t = interval.max;
        }

        if rec1.t >= rec2.t {
            return None;
        }
        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = ray.direction.norm();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_density * random_f64().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let mut rec = rec1;
        rec.t = rec.t + hit_distance / ray_length;
        rec.p = ray.at(rec.t);
        rec.normal = Vector3::new(1.0, 0.0, 0.0);
        rec.front_face = true;
        rec.material = &self.phase_function;

        Some(rec)
    }
}
