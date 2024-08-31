use nalgebra::{Vector2, Vector3};

use crate::{
    aabb::AABB,
    material::Material,
    ray::Ray,
    scene::Scene,
    util::{random_f32, Interval},
};

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
    pub area: f32,
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
            area: n.norm(),
        }
    }

    pub fn hit(&self, ray: &crate::ray::Ray, interval: &Interval) -> Option<HitRecord> {
        let denom = self.normal.dot(&ray.direction);

        if denom.abs() < 1e-8 {
            return None;
        }

        let t = (self.d - self.normal.dot(&ray.origin)) / denom;

        if !interval.contains(t) {
            return None;
        }

        let intersection = ray.at(t);
        let plannar_hitpt_vec = intersection - self.q;
        let alpha = self.w.dot(&plannar_hitpt_vec.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&plannar_hitpt_vec));
        let p = intersection;

        match self.is_interior(alpha, beta) {
            Some(uv) => {
                let mut rec = HitRecord {
                    p,
                    normal: Vector3::zeros(),
                    material: &self.material,
                    t,
                    uv,
                    front_face: true,
                };
                rec.set_face_normal(ray, &self.normal);
                Some(rec)
            }
            None => None,
        }
    }

    /// Given the hit point in plane coordinates, return false if it is outside the
    /// primitive, otherwise set the hit record UV coordinates and return true.

    pub fn is_interior(&self, u: f32, v: f32) -> Option<Vector2<f32>> {
        // 给定平面坐标中的击中点，如果它在基元之外，则返回false，否则设置击中记录的UV坐标并返回true。
        if !(0.0..=1.0).contains(&u) || !(0.0..=1.0).contains(&v) {
            return None;
        }

        Some(Vector2::new(u, v))
    }
    pub fn pdf_value(&self, origin: &Vector3<f32>, direction: &Vector3<f32>) -> f32 {
        match self.hit(
            &Ray::new(origin.clone(), direction.clone()),
            &Interval::new(0.00001, f32::INFINITY),
        ) {
            Some(rec) => {
                let distance_squared = rec.t * rec.t * direction.norm_squared();
                let cosine = ((direction.dot(&rec.normal)) / direction.norm()).abs();
                // println!("self.area: {:?}", self.area);
                let res = distance_squared / (cosine * self.area);
                res
            }
            None => 0.0,
        }
    }
    pub fn random(&self, origin: &Vector3<f32>) -> Vector3<f32> {
        let p = self.q + (random_f32() * self.u) + (random_f32() * self.v);
        p - origin
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
