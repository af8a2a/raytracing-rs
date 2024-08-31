use std::ops::{Index, IndexMut};

use nalgebra::Vector3;

#[derive(Default,Debug)]
pub struct Onb {
    pub u: Vector3<f64>,
    pub v: Vector3<f64>,
    pub w: Vector3<f64>,
}

impl Onb {
    pub fn local(&self, a: f64, b: f64, c: f64) -> Vector3<f64> {
        a * self.u + b * self.v + c * self.w
    }
    pub fn local_v(&self, a: Vector3<f64>) -> Vector3<f64> {
        a.x * self.u + a.y * self.v + a.z * self.w
    }

    pub fn new_from_w(w: Vector3<f64>) -> Self {
        let unit_w = w.normalize();
        let a = if unit_w.x.abs() > 0.9 {
            Vector3::new(0.0, 1.0, 0.0)
        } else {
            Vector3::new(1.0, 0.0, 0.0)
        };
        let v = unit_w.cross(&a).normalize();
        let u = unit_w.cross(&v);
        Self { u, v, w }
    }
}

impl Index<usize> for Onb {
    type Output = Vector3<f64>;

    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.u,
            1 => &self.v,
            _ => &self.w,
        }
    }
}

impl IndexMut<usize> for Onb {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        match i {
            0 => &mut self.u,
            1 => &mut self.v,
            _ => &mut self.w,
        }
    }
}
