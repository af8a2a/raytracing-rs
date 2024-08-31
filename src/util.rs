use core::f64;
use std::f64::consts::PI;

use nalgebra::Vector3;
use rand::{distributions::Uniform, thread_rng, Rng};
#[derive(Debug, Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

pub const EMPTY_INTERVAL: Interval = Interval {
    min: f64::INFINITY,
    max: f64::NEG_INFINITY,
};
pub const UNIVERSE_INTERVAL: Interval = Interval {
    min: f64::NEG_INFINITY,
    max: f64::INFINITY,
};

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, value: f64) -> bool {
        self.min <= value && value <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
    pub fn size(&self) -> f64 {
        self.max - self.min
    }
    pub fn clamp(&self, value: f64) -> f64 {
        if value < self.min {
            self.min
        } else if value > self.max {
            self.max
        } else {
            value
        }
    }
    pub fn merge(lhs: &Self, rhs: &Self) -> Self {
        Self::new(lhs.min.min(rhs.min), lhs.max.max(rhs.max))
    }
    pub fn expand(&self, delta: f64) -> Interval {
        let padding = delta / 2.0;
        Interval::new(self.min - padding, self.max + padding)
    }

    pub fn add_scalar(&self, scalar: f64) -> Self {
        Self::new(self.min + scalar, self.max + scalar)
    }
}
impl Default for Interval {
    fn default() -> Self {
        Self::new(f64::INFINITY, f64::NEG_INFINITY)
    }
}

pub fn random_f64() -> f64 {
    let mut rng = thread_rng();

    rng.sample(Uniform::new(0.0, 1.0))
}
pub fn random_range_f64(min: f64, max: f64) -> f64 {
    let mut rng = thread_rng();

    rng.sample(Uniform::new(min, max))
}

/// Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
pub fn sample_square() -> Vector3<f64> {
    Vector3::new(random_f64() - 0.5, random_f64() - 0.5, 0.0)
}

pub fn random_vec() -> Vector3<f64> {
    Vector3::new(random_f64(), random_f64(), random_f64())
}

pub fn random_vec_range(min: f64, max: f64) -> Vector3<f64> {
    Vector3::new(
        random_range_f64(min, max),
        random_range_f64(min, max),
        random_range_f64(min, max),
    )
}

pub fn random_in_unit_sphere() -> Vector3<f64> {
    loop {
        let p = random_vec_range(-1.0, 1.0);
        if p.norm_squared() <= 1.0 {
            return p;
        }
    }
  }
  

pub fn random_unit_vector() -> Vector3<f64> {
    random_in_unit_sphere().normalize()
}

pub fn random_on_hemisphere(normal: &Vector3<f64>) -> Vector3<f64> {
    let on_unit_sphere = random_unit_vector();
    if on_unit_sphere.dot(normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

pub fn near_zero(v: &Vector3<f64>) -> bool {
    let s = 1e-8;
    (v.x.abs() < s) && (v.y.abs() < s) && (v.z.abs() < s)
}

pub fn reflect(v: &Vector3<f64>, n: &Vector3<f64>) -> Vector3<f64> {
    v - 2.0 * v.dot(n) * n
}

pub fn refract(uv: &Vector3<f64>, n: &Vector3<f64>, etai_over_etat: f64) -> Vector3<f64> {
    let cos_theta = (-uv).dot(n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.norm_squared()).abs().sqrt() * n;
    r_out_perp + r_out_parallel
}

pub fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

pub fn random_int(min: i32, max: i32) -> i32 {
    let mut rng = thread_rng();

    rng.sample(Uniform::new(min, max))
}





pub fn random_cosine_direction() -> Vector3<f64> {
    let r1 = random_f64();
    let r2 = random_f64();
  
    let phi = 2.0 * PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();
    let z = (1.0 - r2).sqrt();
  
    Vector3::new(x, y, z)
  }
  