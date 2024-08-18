use crate::hit::{HitRecord, Hittable};
#[derive(Default)]
pub struct Scene {
    pub objects: Vec<Hittable>,
}

impl Scene {
    pub fn hit(&self, ray: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closet_so_far = t_max;
        let mut hit_record = None;
        for obj in &self.objects {
            if let Some(record) = obj.hit(ray, t_min, closet_so_far) {
                closet_so_far = record.t;
                hit_record.replace(record);
            }
        }
        hit_record
    }
}
