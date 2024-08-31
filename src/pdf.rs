use std::f64::consts::PI;

use nalgebra::Vector3;

use crate::{
    hit::Hittable,
    onb::Onb,
    util::{random_cosine_direction, random_f64, random_range_f64, random_unit_vector},
};
#[derive(Debug)]
pub enum PDF<'a> {
    Cosine(CosinePdf),
    Sphere(SpherePdf),
    None(NonePDF),
    Hittable(Box<HittablePdf<'a>>),
    Mixture(Box<MixturePdf<'a>>),
}

impl PDF<'_> {
    pub fn value(&self, direction: &Vector3<f64>) -> f64 {
        match self {
            PDF::Cosine(pdf) => pdf.value(*direction),
            PDF::Sphere(pdf) => pdf.value(*direction),
            PDF::None(pdf) => pdf.value(*direction),
            PDF::Hittable(pdf) => pdf.value(direction),
            PDF::Mixture(pdf) => pdf.value(direction),
        }
    }
    pub fn generate(&self) -> Vector3<f64> {
        match self {
            PDF::Cosine(pdf) => pdf.generate(),
            PDF::Sphere(pdf) => pdf.generate(),
            PDF::None(pdf) => pdf.generate(),
            PDF::Hittable(pdf) => pdf.generate(),
            PDF::Mixture(pdf) => pdf.generate(),
        }
    }
}
#[derive(Debug)]

pub struct NonePDF;

impl NonePDF {
    pub fn value(&self, _direction: Vector3<f64>) -> f64 {
        0.0
    }
    pub fn generate(&self) -> Vector3<f64> {
        Vector3::new(1.0, 0.0, 0.0)
    }
}

#[derive(Debug)]

pub struct SpherePdf;
impl SpherePdf {
    pub fn value(&self, _direction: Vector3<f64>) -> f64 {
        1.0 / (4.0 * PI)
    }

    pub fn generate(&self) -> Vector3<f64> {
        random_unit_vector()
    }
}
#[derive(Debug)]

pub struct CosinePdf {
    uvw: Onb,
}
impl CosinePdf {
    pub fn new(w: Vector3<f64>) -> Self {
        Self {
            uvw: Onb::new_from_w(w),
        }
    }
}

impl CosinePdf {
    pub fn value(&self, direction: Vector3<f64>) -> f64 {
        let cosine_theta = (direction.normalize()).dot(&self.uvw.w);
        (cosine_theta / PI).max(0.0)
    }

    pub fn generate(&self) -> Vector3<f64> {
        self.uvw.local_v(random_cosine_direction())
    }
}

#[derive(Debug)]

pub struct HittablePdf<'a> {
    pub objects: &'a Hittable,
    pub origin: Vector3<f64>,
}
impl<'a> HittablePdf<'a> {
    pub fn new(objects: &'a Hittable, origin: Vector3<f64>) -> Self {
        Self { objects, origin }
    }
}

impl HittablePdf<'_> {
    pub fn value(&self, direction: &Vector3<f64>) -> f64 {
        self.objects.pdf_value(&self.origin, direction)
    }

    pub fn generate(&self) -> Vector3<f64> {
        self.objects.random(&self.origin)
    }
}

#[derive(Debug)]

pub struct MixturePdf<'a> {
    pub p: [&'a PDF<'a>; 2],
}

impl<'a> MixturePdf<'a> {
    pub fn new(p0: &'a PDF, p1: &'a PDF) -> Self {
        Self { p: [p0, p1] }
    }
}

impl MixturePdf<'_> {
    fn value(&self, direction: &Vector3<f64>) -> f64 {
        let lhs = self.p[0].value(&direction);
        let rhs = self.p[1].value(&direction);
        (lhs + rhs) / 2.0
    }

    fn generate(&self) -> Vector3<f64> {
        if random_f64() < 0.5 {
            self.p[0].generate()
        } else {
            self.p[1].generate()
        }
    }
}
