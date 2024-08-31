use std::f64::{consts::PI, NAN};

use nalgebra::{Vector2, Vector3};

use crate::{
    hit::{self, HitRecord},
    pdf::{CosinePdf, NonePDF, PDF},
    ray::Ray,
    texture::{SolidColor, Texture},
    util::{
        near_zero, random_f64, random_in_unit_sphere, random_on_hemisphere, random_unit_vector,
        reflect, reflectance, refract,
    },
};
#[derive(Debug, Clone)]
pub enum Material {
    Diffuse(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
    DiffuseLight(DiffuseLight),
}

pub struct ScatterRecord<'a> {
    pub attenuation: Vector3<f64>,
    pub pdf: PDF<'a>,
    pub skip_pdf: bool,
    pub skip_pdf_ray: Ray,
}

impl<'a> Default for ScatterRecord<'a> {
    fn default() -> Self {
        Self {
            attenuation: Vector3::zeros(),
            pdf: PDF::None(NonePDF),
            skip_pdf: false,
            skip_pdf_ray: Ray::default(),
        }
    }
}

impl Material {
    pub fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        match self {
            Material::Diffuse(lambert) => lambert.scatter(ray, rec),
            Material::Metal(metal) => metal.scatter(ray, rec),
            Material::Dielectric(dielectric) => dielectric.scatter(ray, rec),
            Material::DiffuseLight(light) => light.scatter(ray, rec),
        }
    }
    pub fn emitted(&self, uv: &Vector2<f64>, p: &Vector3<f64>, rec: &HitRecord) -> Vector3<f64> {
        match self {
            Material::Diffuse(lambert) => lambert.emitted(uv, p, rec),
            Material::Metal(metal) => metal.emitted(uv, p, rec),
            Material::Dielectric(dielectric) => dielectric.emitted(uv, p, rec),
            Material::DiffuseLight(light) => light.emitted(uv, p, rec),
        }
    }
    pub fn scattering_pdf(&self, ray: &Ray, scattered: &Ray, rec: &HitRecord) -> f64 {
        match self {
            Material::Diffuse(lambert) => lambert.scattering_pdf(ray, scattered, rec),
            // Material::Metal(_) => 0.0,
            // Material::Dielectric(_) => 0.0,
            // Material::DiffuseLight(_) => 0.0,
            _ => 0.0,
        }
    }
}
#[derive(Debug, Clone)]
pub struct Lambertian {
    albedo: Box<Texture>,
}

impl Lambertian {
    pub fn new_with_color(albedo: Vector3<f64>) -> Self {
        Self {
            albedo: Box::new(Texture::Color(SolidColor::new(albedo))),
        }
    }
    pub fn new(tex: Texture) -> Self {
        Self {
            albedo: Box::new(tex),
        }
    }
    pub fn scatter(&self, _ray: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let mut srec: ScatterRecord<'_> = ScatterRecord::default();
        srec.attenuation = self.albedo.value(&rec.uv, &rec.p);
        srec.pdf = PDF::Cosine(CosinePdf::new(rec.normal));
        srec.skip_pdf = false;
        Some(srec)
    }
    pub fn emitted(&self, _uv: &Vector2<f64>, _p: &Vector3<f64>, _rec: &HitRecord) -> Vector3<f64> {
        Vector3::zeros()
    }

    pub fn scattering_pdf(&self, _ray: &Ray, scattered: &Ray, rec: &HitRecord) -> f64 {
        let cos_theta = rec.normal.dot(&scattered.direction.normalize());
        if cos_theta < 0.0 {
            0.0
        } else {
            cos_theta / PI
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Metal {
    albedo: Vector3<f64>,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vector3<f64>, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
    pub fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let mut srec = ScatterRecord::default();
        srec.attenuation = self.albedo;
        srec.skip_pdf = true;
        let reflected = reflect(&ray.direction.normalize(), &rec.normal);
        srec.skip_pdf_ray = Ray::new_with_time(
            rec.p,
            reflected + self.fuzz * random_in_unit_sphere(),
            ray.time,
        );
        Some(srec)
    }
    pub fn emitted(&self, _uv: &Vector2<f64>, _p: &Vector3<f64>, _rec: &HitRecord) -> Vector3<f64> {
        Vector3::zeros()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    pub fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let mut srec = ScatterRecord::default();
        srec.attenuation = Vector3::new(1.0, 1.0, 1.0);
        srec.skip_pdf = true;
        let refraction_ratio = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray.direction.normalize();
        let cos_theta = -unit_direction.dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_f64()
        {
            reflect(&unit_direction, &rec.normal)
        } else {
            refract(&unit_direction, &rec.normal, refraction_ratio)
        };
        // let direction=reflect(&unit_direction, &rec.normal)
        srec.skip_pdf_ray = Ray::new_with_time(rec.p, direction, ray.time);
        Some(srec)
    }
    pub fn emitted(&self, _uv: &Vector2<f64>, _p: &Vector3<f64>, _rec: &HitRecord) -> Vector3<f64> {
        Vector3::zeros()
    }
}
#[derive(Debug, Clone)]

pub struct DiffuseLight {
    pub emit: Box<Texture>,
}

impl DiffuseLight {
    pub fn new_with_color(emit: Vector3<f64>) -> Self {
        Self {
            emit: Box::new(Texture::Color(SolidColor::new(emit))),
        }
    }
    pub fn new(tex: Texture) -> Self {
        Self {
            emit: Box::new(tex),
        }
    }

    pub fn emitted(&self, uv: &Vector2<f64>, p: &Vector3<f64>, rec: &HitRecord) -> Vector3<f64> {
        if rec.front_face {
            self.emit.value(uv, p)
        } else {
            Vector3::default()
        }
    }

    pub fn scatter(&self, _ray: &Ray, _hit_record: &HitRecord) -> Option<ScatterRecord> {
        None
    }
}
