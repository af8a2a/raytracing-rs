use crate::{
    bvh::AABB,
    hit::{HitRecord, Hittable},
    util::Interval,
};
#[derive(Debug, Default)]
pub struct Scene {
    pub objects: Vec<Hittable>,
    // bbox: AABB,
}

impl Scene {
    pub fn hit(&self, ray: &crate::ray::Ray, interval: &Interval) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_so_far = interval.max;
        for obj in &self.objects {
            if let Some(record) = obj.hit(ray, &Interval::new(interval.min, closest_so_far)) {
                closest_so_far = record.t;
                hit_record.replace(record);
            }
        }
        hit_record
    }

    pub fn add(&mut self, obj: Hittable) {
        // self.bbox = AABB::merge(&self.bbox, obj.bbox());
        self.objects.push(obj);
    }
    pub fn new(objects: Vec<Hittable>) -> Self {
        Self { objects }
    }
    pub fn new_with_bvh(bvh_node: Hittable) -> Self {
        let mut scene = Scene::new(vec![]);
        scene.add(bvh_node);
        scene
    }
}
