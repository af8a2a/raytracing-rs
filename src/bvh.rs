use nalgebra::Vector3;
use rayon::iter::Once;

use crate::{
    aabb::{AABB, AABB_EMPTY}, hit::{HitRecord, Hittable}, scene::Scene, util::{random_int, Interval, EMPTY_INTERVAL, UNIVERSE_INTERVAL}
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
