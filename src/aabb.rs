use std::ops::Add;

use nalgebra::Vector3;

use crate::util::{Interval, EMPTY_INTERVAL, UNIVERSE_INTERVAL};

#[derive(Default, Debug, Clone)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub fn new(a: Vector3<f64>, b: Vector3<f64>) -> Self {
        let x = Interval::new(a.x.min(b.x), a.x.max(b.x));
        let y = Interval::new(a.y.min(b.y), a.y.max(b.y));
        let z = Interval::new(a.z.min(b.z), a.z.max(b.z));

        let mut tmp = Self { x, y, z };
        tmp.pad_to_minimums();
        tmp
    }
    pub fn axis(&self, axis: usize) -> &Interval {
        match axis {
            0 => &self.x,
            1 => &self.y,
            _ => &self.z,
        }
    }
    pub fn merge(lhs: &Self, rhs: &Self) -> Self {
        let x = Interval::merge(&lhs.x, &rhs.x);
        let y = Interval::merge(&lhs.y, &rhs.y);
        let z = Interval::merge(&lhs.z, &rhs.z);
        let mut tmp = Self { x, y, z };
        tmp.pad_to_minimums();
        tmp
    }
    pub fn hit(&self, ray: &crate::ray::Ray, interval: &mut Interval) -> bool {
        for axis in 0..3 {
            let inv0 = 1.0 / ray.direction.index(axis);
            let orig = ray.origin.index(axis);

            let mut t0 = (self.axis(axis).min - orig) * inv0;
            let mut t1 = (self.axis(axis).max - orig) * inv0;

            if inv0 < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            if t0 > interval.min {
                interval.min = t0;
            }
            if t1 < interval.max {
                interval.max = t1;
              }
        
            if interval.max <= interval.min {
                return false;
            }
        }
        true
    }

    pub fn longest_axis(&self) -> i32 {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() {
                0
            } else {
                2
            }
        } else {
            if self.y.size() > self.z.size() {
                1
            } else {
                2
            }
        }
    }

    fn pad_to_minimums(&mut self) {
        let delta = 0.0001;
        if self.x.size() < delta {
            self.x = self.x.expand(delta);
        }
        if self.y.size() < delta {
            self.y = self.y.expand(delta);
        }
        if self.z.size() < delta {
            self.z = self.z.expand(delta);
        }
    }

    pub fn add_scalar(&self, scalar: f64) -> Self {
        let x = self.x.add_scalar(scalar);
        let y = self.y.add_scalar(scalar);
        let z = self.z.add_scalar(scalar);
        Self { x, y, z }
    }
    pub fn add_vec(&self, vec: Vector3<f64>) -> Self {
        let x = self.x.add_scalar(vec.x);
        let y = self.y.add_scalar(vec.y);
        let z = self.z.add_scalar(vec.z);
        Self { x, y, z }
    }
}

pub const AABB_EMPTY: AABB = AABB {
    x: EMPTY_INTERVAL,
    y: EMPTY_INTERVAL,
    z: EMPTY_INTERVAL,
};

pub const AABB_UNIVERSE: AABB = AABB {
    x: UNIVERSE_INTERVAL,
    y: UNIVERSE_INTERVAL,
    z: UNIVERSE_INTERVAL,
};
