
use image::RgbImage;
use nalgebra::{Vector2, Vector3};

use crate::noise::Perlin;
#[derive(Debug, Clone)]
pub enum Texture {
    Color(SolidColor),
    CheckerTexture(CheckerTexture),
    ImageTexture(ImageTexture),
    Noise(NoiseTexture)
}

impl Texture {
    pub fn value(&self, uv: &Vector2<f32>, p: &Vector3<f32>) -> Vector3<f32> {
        match self {
            Texture::Color(color) => color.value(uv, p),
            Texture::CheckerTexture(tex) => tex.value(uv, p),
            Texture::ImageTexture(tex) => tex.value(uv, p),
            Texture::Noise(tex) => tex.value(uv, p),
        }
    }
}
#[derive(Debug, Clone)]

pub struct SolidColor {
    pub albedo: Vector3<f32>,
}

impl SolidColor {
    pub fn new(albedo: Vector3<f32>) -> Self {
        Self { albedo }
    }
    pub fn value(&self, _uv: &Vector2<f32>, _p: &Vector3<f32>) -> Vector3<f32> {
        self.albedo.clone()
    }
}

#[derive(Debug, Clone)]
pub struct CheckerTexture {
    pub odd: Box<Texture>,
    pub even: Box<Texture>,
    pub inv_scale: f32,
}

impl CheckerTexture {
    pub fn new(odd: Box<Texture>, even: Box<Texture>, scale: f32) -> Self {
        Self {
            odd,
            even,
            inv_scale: 1.0 / scale,
        }
    }
    pub fn new_with_color(odd: &Vector3<f32>, even: &Vector3<f32>, scale: f32) -> Self {
        Self {
            odd: Box::new(Texture::Color(SolidColor::new(odd.clone()))),
            even: Box::new(Texture::Color(SolidColor::new(even.clone()))),
            inv_scale: 1.0 / scale,
        }
    }

    pub fn value(&self, uv: &Vector2<f32>, p: &Vector3<f32>) -> Vector3<f32> {
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
    pub fn value(&self, uv: &Vector2<f32>, _p: &Vector3<f32>) -> Vector3<f32> {
        if self.image.height() <= 0 {
            return Vector3::new(0.0, 1.0, 1.0);
        }

        let u = uv.x.clamp(0.0, 1.0);
        let v = 1.0 - uv.y.clamp(0.0, 1.0);
        let i = (u * self.image.width() as f32) as u32;
        let j = (v * self.image.height() as f32) as u32;
        let pixel = self.image.get_pixel(i, j);
        let scale = 1.0 / 255.0;
        // println!("{},{}",i,j);
        Vector3::new(
            pixel.0[0] as f32 * scale,
            pixel.0[1] as f32 * scale,
            pixel.0[2] as f32 * scale,
        )
    }
}
#[derive(Debug, Clone)]

pub struct NoiseTexture {
    noise: Perlin,
    scale: f32,
}

impl NoiseTexture {
    pub fn new(scale: f32) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
    pub fn value(&self, _uv: &Vector2<f32>, p: &Vector3<f32>) -> Vector3<f32> {
        let noise = self.noise.noise(&(p*self.scale));
        Vector3::new(noise, noise, noise)
    }
}
