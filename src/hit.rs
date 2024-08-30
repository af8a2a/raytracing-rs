pub mod medium;
pub mod quad;
pub mod sphere;
pub mod translate;
use nalgebra::{Vector2, Vector3};

use crate::{aabb::AABB, bvh::BVHNode, material::Material, ray::Ray, scene::Scene, util::Interval};
#[derive(Debug, Clone)]
pub struct HitRecord<'a> {
    pub t: f32,
    pub p: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub front_face: bool,
    pub uv: Vector2<f32>,
    pub material: &'a Material,
}

impl HitRecord<'_> {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vector3<f32>) {
        self.front_face = ray.direction.dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -outward_normal
        };
    }
}
#[derive(Debug, Clone)]
pub enum Hittable {
    Sphere(sphere::Sphere),
    BVH(BVHNode),
    Quad(quad::Quad),
    Translate(translate::Translate),
    Rotate(translate::RotateY),
    PrefabScene(Scene),
    ConstantMedium(medium::ConstMedium),
}

impl Hittable {
    pub fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        match self {
            Hittable::Sphere(sphere) => sphere.hit(ray, interval),
            Hittable::BVH(node) => node.hit(ray, interval),
            Hittable::Quad(quad) => quad.hit(ray, interval),
            Hittable::Translate(t) => t.hit(ray, interval),
            Hittable::Rotate(r) => r.hit(ray, interval),
            Hittable::PrefabScene(scene) => scene.hit(ray, interval),
            Hittable::ConstantMedium(medium) => medium.hit(ray, interval),
        }
    }
    pub fn bbox(&self) -> &AABB {
        match self {
            Hittable::Sphere(sphere) => &sphere.bbox,
            Hittable::BVH(node) => &node.bbox,
            Hittable::Quad(quad) => &quad.aabb,
            Hittable::Translate(t) => &t.bbox,
            Hittable::Rotate(r) => &r.bbox,
            Hittable::PrefabScene(scene) => &scene.bbox,
            Hittable::ConstantMedium(medium) => medium.boundary.bbox(),
        }
    }

    pub fn pdf_value(&self, origin: &Vector3<f32>, direction: &Vector3<f32>) -> f32 {
        match self {
            Hittable::Quad(obj) => obj.pdf_value(&origin, &direction),
            Hittable::Sphere(obj) => obj.pdf_value(&origin, &direction),
            Hittable::BVH(_) => 0.0,
            Hittable::PrefabScene(obj) => obj.pdf_value(&origin, &direction),
            Hittable::Rotate(obj) => obj.object.pdf_value(&origin, &direction),
            Hittable::Translate(obj) => obj.object.pdf_value(&origin, &direction),
            Hittable::ConstantMedium(_) => todo!(),
        }
    }
    pub fn  random(&self, origin: &Vector3<f32>) -> Vector3<f32> {
        match self {
            Hittable::Quad(obj) => obj.random(&origin),
            Hittable::Sphere(obj) => obj.random(&origin),
            Hittable::BVH(_) => Vector3::new(1.0, 0.0, 0.0),
            Hittable::PrefabScene(obj) => obj.random(&origin),
            Hittable::Rotate(obj) => obj.object.random(&origin),
            Hittable::Translate(obj) => obj.object.random(&origin),
            Hittable::ConstantMedium(_) => todo!(),
        }
    }
}
