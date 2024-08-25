use nalgebra::{Vector2, Vector3};

use crate::{bvh::AABB, material::Material, scene::Scene, util::Interval};

use super::HitRecord;
#[derive(Debug, Clone)]

pub struct Quad {
    pub q: Vector3<f32>,
    pub u: Vector3<f32>,
    pub v: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub d: f32,
    pub w: Vector3<f32>,
    pub material: Material,
    pub aabb: AABB,
}

impl Quad {
    pub fn new(q: Vector3<f32>, u: Vector3<f32>, v: Vector3<f32>, material: Material) -> Self {
        let n = u.cross(&v);
        let normal = n.normalize();

        let d = normal.dot(&q);
        // let w = Vector3::new(-0.0625, 0.0, 0.0);
        let w = n / n.dot(&n);
        // assert_eq!(n / n.dot(&n), w);
        let aabb = AABB::new(q, q + u + v);

        Self {
            q,
            u,
            v,
            material,
            aabb,
            normal,
            d,
            w,
        }
    }

    pub fn hit(&self, ray: &crate::ray::Ray, interval: &Interval) -> Option<HitRecord> {
        let mut record = None;
        let denom = self.normal.dot(&ray.direction);

        if denom.abs() < 1e-8 {
            return record;
        }

        let t = (self.d - self.normal.dot(&ray.origin)) / denom;

        if !interval.contains(t) {
            return record;
        }

        let intersection = ray.at(t);
        let plannar_hitpt_vec = intersection - self.q;
        let alpha = self.w.dot(&plannar_hitpt_vec.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&plannar_hitpt_vec));

        let mut hit = HitRecord {
            t,
            p: intersection,
            normal: self.normal,
            front_face: true,
            uv: Vector2::zeros(),
            material: &self.material,
        };
        hit.set_face_normal(ray, &self.normal);
        if !self.is_interior(alpha, beta, &mut hit) {
            return record;
        }

        record.replace(hit);
        record
    }

    /// Given the hit point in plane coordinates, return false if it is outside the
    /// primitive, otherwise set the hit record UV coordinates and return true.

    pub fn is_interior(&self, a: f32, b: f32, rec: &mut HitRecord) -> bool {
        if !(0.0..=1.0).contains(&a) || !(0.0..=1.0).contains(&b) {
            return false;
        }

        rec.uv = Vector2::new(a, b);
        true
    }
}

pub fn box_scene(a: Vector3<f32>, b: Vector3<f32>, mat: Material) -> Scene {
    let mut sides = Scene::default();

    let min = Vector3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
    let max = Vector3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));

    let dx = Vector3::new(max.x - min.x, 0.0, 0.0);
    let dy = Vector3::new(0.0, max.y - min.y, 0.0);
    let dz = Vector3::new(0.0, 0.0, max.z - min.z);

    sides.add(crate::hit::Hittable::Quad(Quad::new(
        Vector3::new(min.x, min.y, max.z),
        dx,
        dy,
        mat.clone(),
    )));

    sides.add(crate::hit::Hittable::Quad(Quad::new(
        Vector3::new(max.x, min.y, max.z),
        -dz,
        dy,
        mat.clone(),
    )));
    sides.add(crate::hit::Hittable::Quad(Quad::new(
        Vector3::new(max.x, min.y, min.z),
        -dx,
        dy,
        mat.clone(),
    )));

    sides.add(crate::hit::Hittable::Quad(Quad::new(
        Vector3::new(min.x, min.y, min.z),
        dz,
        dy,
        mat.clone(),
    )));
    sides.add(crate::hit::Hittable::Quad(Quad::new(
        Vector3::new(min.x, max.y, max.z),
        dx,
        -dz,
        mat.clone(),
    )));
    sides.add(crate::hit::Hittable::Quad(Quad::new(
        Vector3::new(min.x, min.y, min.z),
        dx,
        dz,
        mat.clone(),
    )));

    sides
}
