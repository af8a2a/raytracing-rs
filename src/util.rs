use nalgebra::Vector3;
use rand::{distributions::Uniform, thread_rng, Rng};
#[derive(Debug, Clone)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, value: f32) -> bool {
        self.min <= value && value <= self.max
    }

    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }
    pub fn size(&self) -> f32 {
        self.max - self.min
    }
    pub fn clamp(&self, value: f32) -> f32 {
        if value < self.min {
            self.min
        } else if value > self.max {
            self.max
        } else {
            value
        }
    }
}
impl Default for Interval {
    fn default() -> Self {
        Self::new(f32::MAX, f32::MIN)
    }
}

pub fn random_f32() -> f32 {
    let mut rng = thread_rng();

    rng.sample(Uniform::new(0.0, 1.0))
}
pub fn range_random_f32(min: f32, max: f32) -> f32 {
    let mut rng = thread_rng();

    rng.sample(Uniform::new(min, max))
}

/// Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
pub fn sample_square() -> Vector3<f32> {
    Vector3::new(random_f32() - 0.5, random_f32() - 0.5, 0.0)
}

pub fn random_vec() -> Vector3<f32> {
    Vector3::new(random_f32(), random_f32(), random_f32())
}

pub fn random_vec_range(min: f32, max: f32) -> Vector3<f32> {
    Vector3::new(
        range_random_f32(min, max),
        range_random_f32(min, max),
        range_random_f32(min, max),
    )
}

pub fn random_in_unit_sphere() -> Vector3<f32> {
    loop {
        let p = random_vec_range(-1.0, 1.0);
        if p.norm() <= 1.0 {
            return p.normalize();
        }
    }
}

pub fn random_on_hemisphere(normal: &Vector3<f32>) -> Vector3<f32> {
    let on_unit_sphere = random_in_unit_sphere();
    if on_unit_sphere.dot(normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

pub fn near_zero(v: &Vector3<f32>) -> bool {
    let s = 1e-8;
    (v.x.abs() < s) && (v.y.abs() < s) && (v.z.abs() < s)
}

pub fn reflect(v: &Vector3<f32>, n: &Vector3<f32>) -> Vector3<f32> {
    v - 2.0 * v.dot(n) * n
}

pub fn refract(uv: &Vector3<f32>, n: &Vector3<f32>, etai_over_etat: f32) -> Vector3<f32> {
    let cos_theta = (-uv).dot(n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -((1.0 - r_out_perp.norm_squared()).abs().sqrt()) * n;
    r_out_perp + r_out_parallel
}

pub fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
    let r0= (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * ((1.0 - cosine).powi(5))
}

