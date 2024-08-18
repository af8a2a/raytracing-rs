use crate::{
    hit::{HitRecord, Hittable},
    util::Interval,
};
#[derive(Default)]
pub struct Scene {
    pub objects: Vec<Hittable>,
}

impl Scene {
    pub fn hit(&self, ray: &crate::ray::Ray, interval: &Interval) -> Option<HitRecord> {
        let mut interval=interval.clone();
        let mut hit_record = None;
        for obj in &self.objects {
            if let Some(record) = obj.hit(ray, &interval) {
                interval.max = record.t;
                hit_record.replace(record);
            }
        }
        hit_record
    }
}
