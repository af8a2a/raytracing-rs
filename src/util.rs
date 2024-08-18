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
}
impl Default for Interval {
    fn default() -> Self {
        Self::new(f32::MAX, f32::MIN)
    }
}


const EMPTY_INTERVAL: Interval = Interval {
    min: f32::MAX,
    max: f32::MIN,
};
const UNIVERSE_INTERVAL: Interval = Interval {
    min: f32::MIN,
    max: f32::MAX,
};
