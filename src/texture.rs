use nalgebra::{Vector2, Vector3};
#[derive(Debug, Clone)]
pub enum Texture {
    Color(SolidColor),
    CheckerTexture(CheckerTexture),
}

impl Texture {
    pub fn value(&self, uv: &Vector2<f32>, p: &Vector3<f32>) -> Vector3<f32> {
        match self {
            Texture::Color(color) => color.value(uv, p),
            Texture::CheckerTexture(tex) => tex.value(uv, p),
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
    pub fn new(odd: Box<Texture>, even: Box<Texture>, inv_scale: f32) -> Self {
        Self {
            odd,
            even,
            inv_scale,
        }
    }
    pub fn new_with_color(odd: &Vector3<f32>, even: &Vector3<f32>, inv_scale: f32) -> Self {
        Self {
            odd: Box::new(Texture::Color(SolidColor::new(odd.clone()))),
            even: Box::new(Texture::Color(SolidColor::new(even.clone()))),
            inv_scale,
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
