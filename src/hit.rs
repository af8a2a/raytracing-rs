pub mod sphere;
use nalgebra::Vector3;

use crate::{bvh::{BVHNode, AABB}, material::Material, ray::Ray, util::Interval};
#[derive(Debug, Clone)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub front_face: bool,
    pub material: Material,
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
#[derive(Debug, Clone)]
pub enum Hittable {
    Sphere(sphere::Sphere),
    BVHNode(BVHNode),
}

impl Hittable {
    pub fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        match self {
            Hittable::Sphere(sphere) => sphere.hit(ray, interval),
            Hittable::BVHNode(node) => node.hit(ray, interval),
        }
    }
    pub fn bbox(&self)->&AABB{
        match self {
            Hittable::Sphere(sphere) => &sphere.bbox,
            Hittable::BVHNode(node) => &node.bbox,
        }
    }
}
