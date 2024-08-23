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
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let u= u * u * (3.0 - 2.0 * u);
        let v= v * v * (3.0 - 2.0 * v);
        let w= w * w * (3.0 - 2.0 * w);


        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;

        let mut c = [[[0.0; 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let x = self.perm_x[(i as usize + di) & 255];
                    let y = self.perm_y[(j as usize + dj) & 255];
                    let z = self.perm_z[(k as usize + dk) & 255];
                    c[di][dj][dk] = self.randflots[x ^ y ^ z];
                }
            }
        }
        Self::trilinear_interp(&c, u, v, w)
    }

    fn trilinear_interp(c: &[[[f32; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += ((i as f32 * u) + (1.0 - i as f32) * (1.0 - u))
                        * (j as f32 * v + (1.0 - j as f32) * (1.0 - v))
                        * (k as f32 * w + (1.0 - k as f32) * (1.0 - w))
                        * c[i][j][k]
                }
            }
        }
        accum
    }
}
