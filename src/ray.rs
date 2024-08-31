use nalgebra::Vector3;
#[derive(Debug,Default, Clone)]
pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>) -> Self {
        Self {
            origin,
            direction,
            time: 0.0,
        }
    }
    pub fn new_with_time(origin: Vector3<f64>, direction: Vector3<f64>, time: f64) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }
    pub fn at(&self, t: f64) -> Vector3<f64> {
        self.origin + t * self.direction
    }
}
