use nalgebra::Vector3;
use rayon::iter::Once;

use crate::{
    hit::{HitRecord, Hittable},
    scene::Scene,
    util::{random_int, Interval, EMPTY_INTERVAL, UNIVERSE_INTERVAL},
};
#[derive(Default, Debug, Clone)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub fn new(a: Vector3<f32>, b: Vector3<f32>) -> Self {
        let x = if a.x < b.x {
            Interval::new(a.x, b.x)
        } else {
            Interval::new(b.x, a.x)
        };
        let y = if a.y < b.y {
            Interval::new(a.y, b.y)
        } else {
            Interval::new(b.y, a.y)
        };
        let z = if a.z < b.z {
            Interval::new(a.z, b.z)
        } else {
            Interval::new(b.z, a.z)
        };
        Self { x, y, z }
    }
    pub fn axis_interval(&self, axis: usize) -> &Interval {
        if axis == 1 {
            &self.y
        } else if axis == 2 {
            &self.z
        } else {
            &self.x
        }
    }
    pub fn merge(lhs: &Self, rhs: &Self) -> Self {
        let x = Interval::merge(&lhs.x, &rhs.x);
        let y = Interval::merge(&lhs.y, &rhs.y);
        let z = Interval::merge(&lhs.z, &rhs.z);
        Self { x, y, z }
    }
    pub fn hit(&self, ray: &crate::ray::Ray, interval: &Interval) -> bool {
        let mut interval = interval.clone();
        let ray_orig = ray.origin;
        let ray_dir = ray.direction;
        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_dir.index(axis);

            let t0 = (ax.min - ray_orig.index(axis)) * adinv;
            let t1 = (ax.max - ray_orig.index(axis)) * adinv;
            if t0 < t1 {
                if t0 > interval.min {
                    interval.min = t0;
                }
                if t1 < interval.max {
                    interval.max = t1;
                }
            } else {
                if t1 > interval.min {
                    interval.min = t1;
                }
                if t0 < interval.max {
                    interval.max = t0;
                }
            }
            if interval.max <= interval.min {
                return false;
            }
        }
        true
    }

    pub fn longest_axis(&self)->i32{
        if self.x.size()>self.y.size(){
            if self.x.size()>self.z.size(){
                0
            }else{
                2
            }
    }else{
        if self.y.size()>self.z.size(){
            1
        }else{
            2
        }
    }
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

#[derive(Debug, Clone)]
pub struct BVHNode {
    pub bbox: AABB,
    pub left: Box<Hittable>,
    pub right: Box<Hittable>,
}

impl BVHNode {
    pub fn new_with_scene(scene: &Scene) -> Self {
        // println!("{}", scene.objects.len());
        Self::new(&mut scene.objects.clone(), 0, scene.objects.len())
    }
    pub fn new(objects: &mut Vec<Hittable>, start: usize, end: usize) -> Self {
        let object_span = end - start;
        let left;
        let right;
        let mut bbox = AABB_EMPTY.clone();
        for obj in objects.iter() {
            bbox = AABB::merge(&bbox, obj.bbox());
        }

        let axis=bbox.longest_axis(); 
        let box_compare = |a: &Hittable, b: &Hittable, axis_index: usize| {
            let a_axis_interval = a.bbox().axis_interval(axis_index);
            let b_axis_interval = b.bbox().axis_interval(axis_index);
            a_axis_interval.min.partial_cmp(&b_axis_interval.min)
        };
        let box_x_compare = |a: &Hittable, b: &Hittable| box_compare(a, b, 0);
        let box_y_compare = |a: &Hittable, b: &Hittable| box_compare(a, b, 1);
        let box_z_compare = |a: &Hittable, b: &Hittable| box_compare(a, b, 2);
        if object_span == 1 {
            left = Box::new(objects[start].clone());
            right = Box::new(objects[start].clone());
        } else if object_span == 2 {
            left = Box::new(objects[start].clone());
            right = Box::new(objects[start + 1].clone());
        } else {
            match axis {
                0 => {
                    objects.sort_by(|arg0: &Hittable, arg1: &Hittable| {
                        box_x_compare(arg0, arg1).unwrap()
                    });
                }
                1 => {
                    objects.sort_by(|arg0: &Hittable, arg1: &Hittable| {
                        box_y_compare(arg0, arg1).unwrap()
                    });
                }
                2 => {
                    objects.sort_by(|arg0: &Hittable, arg1: &Hittable| {
                        box_z_compare(arg0, arg1).unwrap()
                    });
                }
                _ => unreachable!(),
            }
            let mid = start + object_span / 2;
            left = Box::new(Hittable::BVHNode(BVHNode::new(objects, start, mid)));
            right = Box::new(Hittable::BVHNode(BVHNode::new(objects, mid, end)));
        }
        let bbox = AABB::merge(&left.bbox(), &right.bbox());
        Self { bbox, left, right }
    }

    pub fn hit(&self, ray: &crate::ray::Ray, interval: &Interval) -> Option<HitRecord> {
        if !self.bbox.hit(ray, interval) {
            return None;
        }
        let mut interval = interval.clone();
        let mut hit_record = None;
        if let Some(record) = self.left.hit(ray, &interval) {
            interval.max = record.t;
            hit_record.replace(record);
        }
        if let Some(record) = self.right.hit(ray, &interval) {
            hit_record.replace(record);
        }

        hit_record
    }
}
