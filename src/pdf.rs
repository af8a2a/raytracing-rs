use std::f32::consts::PI;

use nalgebra::Vector3;

use crate::{hit::Hittable, onb::Onb, util::{random_cosine_direction, random_range_f32, random_unit_vector}};

pub enum PDF<'a> {
    Cosine(CosinePdf),
    Sphere(SpherePdf),
    None(NonePDF),
    Hittable(Box<HittablePdf<'a>>),
    Mixture(Box<MixturePdf<'a>>),
}

impl PDF<'_> {
    pub fn value(&self, direction: &Vector3<f32>) -> f32 {
        1.0
    }
    pub fn generate(&self) -> Vector3<f32> {
        Vector3::zeros()
    }
}

pub struct NonePDF;

impl NonePDF {
    pub fn value(&self, _direction: Vector3<f32>) -> f32 {
        0.0
    }
    pub fn generate(&self) -> Vector3<f32> {
        Vector3::new(1.0, 0.0, 0.0)
    }
}


pub struct SpherePdf;
impl  SpherePdf {
    pub fn value(&self, _direction: Vector3<f32>) -> f32 {
        1.0 / (4.0 * PI)
    }

    pub fn generate(&self) -> Vector3<f32> {
        random_unit_vector()
    }
}

pub struct CosinePdf {
    uvw: Onb,
}
impl CosinePdf {
    pub fn new(w: Vector3<f32>) -> Self {
        Self {
            uvw: Onb::new_from_w(w),
        }
    }
}


impl  CosinePdf {
    pub fn value(&self, direction: Vector3<f32>) -> f32 {
        let cosine_theta = direction.normalize().dot(&self.uvw.w);
        (cosine_theta / PI).max(0.0)
    }

    pub fn generate(&self) -> Vector3<f32> {
        self.uvw.local_v(random_cosine_direction())
    }
}


pub struct HittablePdf<'a> {
    pub objects: &'a Hittable,
    pub origin: Vector3<f32>,
}
impl<'a> HittablePdf<'a> {
    pub fn new(objects: &'a Hittable, origin: Vector3<f32>) -> Self {
        Self { objects, origin }
    }
}


impl  HittablePdf<'_> {
    pub fn value(&self, direction: Vector3<f32>) -> f32 {
        self.objects.pdf_value(self.origin, direction)
    }

    pub fn generate(&self) -> Vector3<f32> {
        self.objects.random(self.origin)
    }
}

    
pub struct MixturePdf<'a> {
    pub p: [&'a PDF<'a>; 2],
}

impl<'a> MixturePdf<'a> {
    pub fn new(p0: &'a PDF, p1: &'a PDF) -> Self {
        Self { p: [p0, p1] }
    }
}

impl  MixturePdf<'_> {
    fn value(&self, direction: Vector3<f32>) -> f32 {
        0.5 * self.p[0].value(&direction) + 0.5 * self.p[1].value(&direction)
    }

    fn generate(&self) -> Vector3<f32> {
        if random_range_f32(0.0, 1.0) < 0.5 {
            self.p[0].generate()
        } else {
            self.p[1].generate()
        }
    }
}
