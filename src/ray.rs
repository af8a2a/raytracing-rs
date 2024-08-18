use nalgebra::Vector3;

pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
    pub time: f32,
}

impl Ray {
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Self {
        Self {
            origin,
            direction,
            time: 0.0,
        }
    }
    pub fn new_with_time(origin: Vector3<f32>, direction: Vector3<f32>, time: f32) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }
    pub fn at(&self, t: f32) -> Vector3<f32> {
        self.origin + t * self.direction
    }
}
