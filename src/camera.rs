use core::f64;
use std::{
    f64::{consts::PI, INFINITY, NAN},
    ops::Add,
};

use image::{ImageFormat, Rgb, RgbImage};
use nalgebra::{clamp, Vector3};
use rayon::iter::ParallelIterator;

use crate::{
    hit::Hittable,
    pdf::{HittablePdf, MixturePdf},
    ray::Ray,
    scene::Scene,
    util::{random_f64, random_in_unit_sphere, random_unit_vector, sample_square, Interval},
};

pub struct Camera {
    pub aspect_ratio: f64,        // Ratio of image width over height
    pub image_width: usize,       // Rendered image width in pixel count
    pub samples_per_pixel: usize, // Count of random samples for each pixel
    pub max_depth: usize,         // Maximum number of ray bounces into scene
    pub background: Vector3<f64>, // Background color for rays that miss
    pub vfov: f64,                // Vertical field of view in degrees
    pub lookfrom: Vector3<f64>,   // Camera origin
    pub lookat: Vector3<f64>,     // Point camera is looking at
    pub vup: Vector3<f64>,        // Camera up vector
    pub defocus_angle: f64,       // Defocus blur angle
    pub focus_dist: f64,          // Focus distance
    image_height: usize,          // Rendered image height
    sqrt_spp: usize,              // Square root of samples per pixel
    recip_sqrt_spp: f64,          // Reciprocal of square root of samples per pixel
    center: Vector3<f64>,         // Camera center
    pixel00_loc: Vector3<f64>,    // Location of pixel 0, 0
    pixel_delta_u: Vector3<f64>,  // Offset to pixel to the right
    pixel_delta_v: Vector3<f64>,  // Offset to pixel below
    u: Vector3<f64>,              // Camera horizontal axis
    v: Vector3<f64>,              // Camera vertical axis
    w: Vector3<f64>,              // Camera forward axis
    defocus_disk_u: Vector3<f64>, // Defocus disk horizontal axis
    defocus_disk_v: Vector3<f64>, // Defocus disk vertical axis
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            background: Vector3::default(),
            vfov: 90.0,
            lookfrom: Vector3::new(0.0, 0.0, -1.0),
            lookat: Vector3::new(0.0, 0.0, 0.0),
            vup: Vector3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            image_height: 0,
            sqrt_spp: 10.0_f64.sqrt() as usize,
            recip_sqrt_spp: 1.0 / (10.0_f64.sqrt()),
            center: Vector3::default(),
            pixel00_loc: Vector3::default(),
            pixel_delta_u: Vector3::default(),
            pixel_delta_v: Vector3::default(),
            u: Vector3::default(),
            v: Vector3::default(),
            w: Vector3::default(),
            defocus_disk_u: Vector3::default(),
            defocus_disk_v: Vector3::default(),
        }
    }
}

fn linear_to_gamma(color: f64) -> f64 {
    if color > 0.0 {
        color.sqrt()
    } else {
        0.0
    }
}

fn color_to_rgb(color: Vector3<f64>, sample_per_pixel: f64) -> Rgb<u8> {
    let r = color.x;
    let g = color.y;
    let b = color.z;

    let r = if r.is_nan() { 0.0 } else { r };
    let g = if g.is_nan() { 0.0 } else { g };
    let b = if b.is_nan() { 0.0 } else { b };

    let scale = 1.0 / sample_per_pixel as f64;
    let r = scale * r;
    let g = scale * g;
    let b = scale * b;

    Rgb([
        (256.0 * f64::clamp(linear_to_gamma(r), 0.0, 0.999)) as u8,
        (256.0 * f64::clamp(linear_to_gamma(g), 0.0, 0.999)) as u8,
        (256.0 * f64::clamp(linear_to_gamma(b), 0.0, 0.999)) as u8,
    ])
}

impl Camera {
    pub fn render(&mut self, world: Hittable, lights: Hittable) {
        self.initialize();

        // println!("P3\n{} {}\n255", self.image_width, self.image_height);
        // let stdout = std::io::stdout();
        let width = self.image_width;
        let height = self.image_height;

        let mut image = RgbImage::new(width as u32, height as u32);
        image.par_enumerate_pixels_mut().for_each(|(i, j, pixel)| {
            let mut color = Vector3::new(0.0, 0.0, 0.0);
            for s_j in 0..self.sqrt_spp {
                for s_i in 0..self.sqrt_spp {
                    let ray = self.get_ray(i as i32, j as i32, s_i as i32, s_j as i32);
                    color += self.ray_color(&ray, self.max_depth, &world, &lights);
                }
            }
            *pixel = color_to_rgb(color, self.samples_per_pixel as f64);
        });
        image.save("image.png").expect("Failed to save image");

        // for j in 0..self.image_height {
        //     eprintln!("\rScanlines remaining: {}", self.image_height - j);
        //     for i in 0..self.image_width {
        //         let mut pixel_color = Color::default();
        //         for s_j in 0..self.sqrt_spp {
        //             for s_i in 0..self.sqrt_spp {
        //                 let r = self.get_ray(i as i32, j as i32, s_i as i32, s_j as i32);
        //                 pixel_color += self.ray_color(&r, self.max_depth, world, lights);
        //             }
        //         }
        //         pixel_color
        //             .write_color(&mut stdout.lock(), self.samples_per_pixel)
        //             .unwrap();
        //     }
        // }

        eprintln!("\nDone.");
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as usize;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };
        self.sqrt_spp = (self.samples_per_pixel as f64).sqrt() as usize;
        self.recip_sqrt_spp = 1.0 / (self.sqrt_spp as f64);

        self.center = self.lookfrom;

        // 确定视口尺寸。
        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // 计算相机坐标系的 u,v,w 单位基向量。
        self.w = (self.lookfrom - self.lookat).normalize();
        self.u = (self.vup.cross(&self.w)).normalize();
        self.v = self.w.cross(&self.u);

        // 计算水平和垂直视口边缘上的向量。
        let viewport_u = self.u * viewport_width;
        let viewport_v = -self.v * viewport_height;

        // 计算从像素到像素的水平和垂直增量向量。
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // 计算左上角像素的位置。
        let viewport_upper_left =
            self.center - (self.focus_dist * self.w) - (0.5 * viewport_u) - (0.5 * viewport_v);
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        // 计算相机失焦盘的基向量。
        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.0).to_radians().tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    fn get_ray(&self, i: i32, j: i32, s_i: i32, s_j: i32) -> Ray {
        // Get a randomly sampled camera ray for the pixel at location i,j.
        let pixel_center =
            self.pixel00_loc + i as f64 * self.pixel_delta_u + j as f64 * self.pixel_delta_v;
        let pixel_sample = pixel_center + self.pixel_sample_square(s_i, s_j);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = random_f64();

        Ray::new_with_time(ray_origin, ray_direction, ray_time)
    }

    fn pixel_sample_square(&self, s_i: i32, s_j: i32) -> Vector3<f64> {
        // Returns a random point in the square surrounding a pixel at the origin.
        let px = -0.5 + self.recip_sqrt_spp * (s_i as f64 + random_f64());
        let py = -0.5 + self.recip_sqrt_spp * (s_j as f64 + random_f64());
        px * self.pixel_delta_u + py * self.pixel_delta_v
    }

    fn defocus_disk_sample(&self) -> Vector3<f64> {
        // Returns a random point in the defocus disk.
        let p = random_in_unit_sphere();
        self.center + p.x * self.defocus_disk_u + p.y * self.defocus_disk_v
    }

    fn ray_color(
        &self,
        r: &Ray,
        depth: usize,
        world: &Hittable,
        lights: &Hittable,
    ) -> Vector3<f64> {
        // 如果我们超过了光线反弹限制，就不再收集光线。
        if depth == 0 {
            return Vector3::default();
        }

        // 如果光线没有击中了世界中的任何东西，则返回背景颜色。
        match world.hit(r, &Interval::new(0.001, INFINITY)) {
            Some(rec) => {
                let mat = rec.material;
                let color_from_emission = mat.emitted(&rec.uv, &rec.p,  &rec);

                match mat.scatter(r, &rec) {
                    Some(srec) => {
                        if srec.skip_pdf {
                            return srec.attenuation.component_mul(&self.ray_color(
                                &srec.skip_pdf_ray,
                                depth - 1,
                                world,
                                lights,
                            ));
                        }

                        let light_pdf =
                            crate::pdf::PDF::Hittable(Box::new(HittablePdf::new(lights, rec.p)));
                        let mixed_pdf = MixturePdf::new(&light_pdf, &srec.pdf);

                        let scattered = Ray::new_with_time(rec.p, mixed_pdf.generate(), r.time);
                        let pdf = mixed_pdf.value(&scattered.direction);

                        let scattering_pdf = mat.scattering_pdf(&r, &scattered, &rec);
                        // eprintln!("pdf: {}, scattering_pdf: {}", pdf, scattering_pdf);
                        let sample_color=self.ray_color(&scattered, depth - 1, world, lights);
                        let color_from_scatter =
                            (srec.attenuation.component_mul(&sample_color) * scattering_pdf) / pdf;

                        color_from_emission + color_from_scatter
                    }
                    None => return color_from_emission,
                }
            }
            None => self.background,
        }
    }
}
