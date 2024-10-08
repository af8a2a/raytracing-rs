use image::RgbImage;
use nalgebra::{Vector2, Vector3};

use crate::{hit::HitRecord, noise::Perlin, ray::Ray, util::random_unit_vector};
#[derive(Debug, Clone)]
pub enum Texture {
    Color(SolidColor),
    CheckerTexture(CheckerTexture),
    ImageTexture(ImageTexture),
    Noise(NoiseTexture),
    Isotropic(Isotropic),
}

impl Texture {
    pub fn value(&self, uv: &Vector2<f64>, p: &Vector3<f64>) -> Vector3<f64> {
        match self {
            Texture::Color(color) => color.value(uv, p),
            Texture::CheckerTexture(tex) => tex.value(uv, p),
            Texture::ImageTexture(tex) => tex.value(uv, p),
            Texture::Noise(tex) => tex.value(uv, p),
            Texture::Isotropic(isotropic) => isotropic.tex.value(uv, p),
        }
    }
}
#[derive(Debug, Clone)]

pub struct SolidColor {
    pub albedo: Vector3<f64>,
}

impl SolidColor {
    pub fn new(albedo: Vector3<f64>) -> Self {
        Self { albedo }
    }
    pub fn value(&self, _uv: &Vector2<f64>, _p: &Vector3<f64>) -> Vector3<f64> {
        self.albedo
    }
}

#[derive(Debug, Clone)]
pub struct CheckerTexture {
    pub odd: Box<Texture>,
    pub even: Box<Texture>,
    pub inv_scale: f64,
}

impl CheckerTexture {
    pub fn new(odd: Box<Texture>, even: Box<Texture>, scale: f64) -> Self {
        Self {
            odd,
            even,
            inv_scale: 1.0 / scale,
        }
    }
    pub fn new_with_color(odd: &Vector3<f64>, even: &Vector3<f64>, scale: f64) -> Self {
        Self {
            odd: Box::new(Texture::Color(SolidColor::new(odd.clone()))),
            even: Box::new(Texture::Color(SolidColor::new(even.clone()))),
            inv_scale: 1.0 / scale,
        }
    }

    pub fn value(&self, uv: &Vector2<f64>, p: &Vector3<f64>) -> Vector3<f64> {
        let x = (self.inv_scale * p.x).floor() as i32;
        let y = (self.inv_scale * p.y).floor() as i32;
        let z = (self.inv_scale * p.z).floor() as i32;

        let is_even = (x + y + z) % 2 == 0;
        if is_even {
            self.even.value(uv, p)
        } else {
            self.odd.value(uv, p)
        }
    }
}
#[derive(Debug, Clone)]
pub struct ImageTexture {
    image: RgbImage,
}

impl ImageTexture {
    pub fn new(image: RgbImage) -> Self {
        Self { image }
    }
    pub fn value(&self, uv: &Vector2<f64>, _p: &Vector3<f64>) -> Vector3<f64> {
        if self.image.height() <= 0 {
            return Vector3::new(0.0, 1.0, 1.0);
        }

        let u = uv.x.clamp(0.0, 1.0);
        let v = 1.0 - uv.y.clamp(0.0, 1.0);
        let i = ((u * (self.image.width()) as f64) as u32).clamp(0, self.image.width()-1);
        let j = ((v * (self.image.height()) as f64) as u32).clamp(0, self.image.height()-1);
        let pixel = self.image.get_pixel(i, j);
        let scale = 1.0 / 255.0;
        // println!("{},{}",i,j);
        Vector3::new(
            pixel.0[0] as f64 * scale,
            pixel.0[1] as f64 * scale,
            pixel.0[2] as f64 * scale,
        )
    }
}
#[derive(Debug, Clone)]

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
    pub fn value(&self, _uv: &Vector2<f64>, p: &Vector3<f64>) -> Vector3<f64> {
        // let noise = self.noise.noise(&(p * self.scale)) * 0.5 + 0.5;
        let noise: f64 = 0.5 * (1.0 + (self.scale * p.z + 10.0 * self.noise.turb(p, 7)).sin());
        Vector3::new(noise, noise, noise)
    }
}
#[derive(Debug, Clone)]

pub struct Isotropic {
    tex: Box<Texture>,
}

impl Isotropic {
    pub fn new(tex: Texture) -> Self {
        Self { tex: Box::new(tex) }
    }
    pub  fn new_with_color(albedo: Vector3<f64>) -> Self {
        Self {
            tex: Box::new(Texture::Color(SolidColor::new(albedo))),
        }
    }
    pub fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vector3<f64>)> {
        let scattered= Ray::new_with_time(hit_record.p, random_unit_vector(), ray.time);
        let attenuation = self.tex.value(&hit_record.uv, &hit_record.p).clone();
        Some((scattered,attenuation))
    
    }
}
