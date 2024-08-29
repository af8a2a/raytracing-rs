use nalgebra::Vector3;
use rayon::iter::Once;

use crate::{
    aabb::{AABB, AABB_EMPTY},
    hit::{HitRecord, Hittable},
    scene::Scene,
    util::{random_int, Interval, EMPTY_INTERVAL, UNIVERSE_INTERVAL},
};

#[derive(Debug, Clone)]
pub struct BVHNode {
    pub bbox: AABB,
    pub left: Box<Hittable>,
    pub right: Box<Hittable>,
}

impl BVHNode {
    pub fn new(scene: &mut Scene) -> Self {
        let len = scene.objects.len();

        Self::new_with_scene(&mut scene.objects, 0, len)
    }
    pub fn new_with_scene(objects: &mut Vec<Hittable>, start: usize, end: usize) -> Self {
        let mut bbox = AABB_EMPTY;
        objects[start..end].iter().for_each(|obj| {
            bbox = AABB::merge(&bbox, obj.bbox());
        });
        let axis = bbox.longest_axis();

        let comparator = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            _ => Self::box_z_compare,
        };

        let object_span = end - start;

        if object_span == 1 {
            Self {
                left: Box::new(objects[start].clone()),
                right: Box::new(objects[start].clone()),
                bbox,
            }
        } else if object_span == 2 {
            if comparator(&objects[start], &objects[start + 1]) == std::cmp::Ordering::Less {
                Self {
                    left: Box::new(objects[start].clone()),
                    right: Box::new(objects[start + 1].clone()),
                    bbox,
                }
            } else {
                Self {
                    left: Box::new(objects[start + 1].clone()),
                    right: Box::new(objects[start].clone()),
                    bbox,
                }
            }
        } else {
            objects[start..end].sort_by(comparator);

            let mid = start + object_span / 2;
            let left = Box::new(Hittable::BVH(Self::new_with_scene(objects, start, mid)));
            let right = Box::new(Hittable::BVH(Self::new_with_scene(objects, mid, end)));
            Self { left, right, bbox }
        }
    }

    pub fn hit(&self, ray: &crate::ray::Ray, interval: &Interval) -> Option<HitRecord> {
        let mut interval = interval.clone();
        if !self.bbox.hit(ray, &mut interval) {
            return None;
        }
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
    fn box_compare(a: &Hittable, b: &Hittable, axis_index: usize) -> std::cmp::Ordering {
        a.bbox()
            .axis(axis_index)
            .min
            .partial_cmp(&b.bbox().axis(axis_index).min)
            .unwrap()
    }

    fn box_x_compare(a: &Hittable, b: &Hittable) -> std::cmp::Ordering {
        Self::box_compare(a, b, 0)
    }

    fn box_y_compare(a: &Hittable, b: &Hittable) -> std::cmp::Ordering {
        Self::box_compare(a, b, 1)
    }

    fn box_z_compare(a: &Hittable, b: &Hittable) -> std::cmp::Ordering {
        Self::box_compare(a, b, 2)
    }
}
