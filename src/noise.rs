use std::mem::swap;

use nalgebra::Vector3;

use crate::util::{random_f32, random_int};

pub const POINT_COUNT: usize = 256;
#[derive(Debug, Clone)]
pub struct Perlin {
    randflots: [f32; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    fn perlin_generate_perm() -> [usize; POINT_COUNT] {
        let mut randflots = [0; POINT_COUNT];
        randflots
            .iter_mut()
            .enumerate()
            .for_each(|(idx, x)| *x = idx);
        Perlin::permute(&mut randflots);
        randflots
    }
    fn permute(p: &mut [usize; POINT_COUNT]) {
        for i in (1..POINT_COUNT).rev() {
            let target = random_int(0, i as i32) as usize;
            p.swap(target, i);
        }
    }

    pub fn new() -> Self {
        let mut randflots = [0.0; POINT_COUNT];
        randflots.iter_mut().for_each(|x| *x = random_f32());
        let perm_x = Perlin::perlin_generate_perm();
        let perm_y = Perlin::perlin_generate_perm();
        let perm_z = Perlin::perlin_generate_perm();
        Self {
            randflots,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: &Vector3<f32>) -> f32 {
        let i = ((4.0 * p.x) as i32) & 255;
        let j = ((4.0 * p.y) as i32) & 255;
        let k = ((4.0 * p.z) as i32) & 255;
        self.randflots[self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]]
    }
}
