use std::f32::consts::PI;

use image::{Rgb, RgbImage};
use nalgebra::{clamp, Vector3};
use rayon::iter::ParallelIterator;

use crate::{
    ray::Ray,
    scene::Scene,
    util::{random_f32, random_unit_vector, sample_square, Interval},
};

fn linear_to_gamma(color: f32) -> f32 {
    if color > 0.0 {
        color.sqrt()
    } else {
        0.0
    }
}

fn color_to_rgb(color: Vector3<f32>) -> Rgb<u8> {
    Rgb([
        (256.0 * f32::clamp(linear_to_gamma(color.x), 0.0, 0.999)) as u8,
        (256.0 * f32::clamp(linear_to_gamma(color.y), 0.0, 0.999)) as u8,
        (256.0 * f32::clamp(linear_to_gamma(color.z), 0.0, 0.999)) as u8,
    ])
}

pub struct Camera {
    pub aspect_ratio: f32, //aka fov
    pub image_width: u32,
    image_height: u32,
    center: Vector3<f32>,
    pixel00_loc: Vector3<f32>,
    pixel_delta_u: Vector3<f32>,
    pixel_delta_v: Vector3<f32>,
    pub sample_per_pixel: u32,
    pub depth: i32,

    pub vfov: f32,
    pub look_from: Vector3<f32>,
    pub look_at: Vector3<f32>,
    pub vup: Vector3<f32>,

    pub defocus_angle: f32,
    pub focus_dist: f32,
    defocus_disk_u: Vector3<f32>,
    defocus_disk_v: Vector3<f32>,

    pub background: Vector3<f32>,

    u: Vector3<f32>,
    v: Vector3<f32>,
    w: Vector3<f32>,

    sqrt_spp: u32,
    recip_sqrt_spp: f32,
}
impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            sample_per_pixel: 10,
            depth: 10,
            vfov: 90.0,
            look_from: Vector3::new(0.0, 0.0, -1.0),
            look_at: Vector3::new(0.0, 0.0, 0.0),
            vup: Vector3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            image_height: 0,
            center: Vector3::zeros(),
            pixel00_loc: Vector3::zeros(),
            pixel_delta_u: Vector3::zeros(),
            pixel_delta_v: Vector3::zeros(),
            u: Vector3::zeros(),
            v: Vector3::zeros(),
            w: Vector3::zeros(),
            defocus_disk_u: Vector3::zeros(),
            defocus_disk_v: Vector3::zeros(),
            background: Vector3::new(1.0, 1.0, 1.0),
            sqrt_spp: 1,
            recip_sqrt_spp: 1.0,
        }
    }
}

impl Camera {
    pub fn initialize(&mut self) {
        self.image_height = (self.image_width as f32 / self.aspect_ratio) as u32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.center = self.look_from;

        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f32 / self.image_height as f32);

        self.w = (self.look_from - self.look_at).normalize();
        self.u = (self.vup.cross(&self.w)).normalize();
        self.v = self.w.cross(&self.u);

        // 计算水平和垂直视口边缘上的向量。
        let viewport_u = self.u * viewport_width;
        let viewport_v = -self.v * viewport_height;

        // 计算从像素到像素的水平和垂直增量向量。
        self.pixel_delta_u = viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;

        // 计算左上角像素的位置。
        let viewport_upper_left =
            self.center - (self.focus_dist * self.w) - (0.5 * viewport_u) - (0.5 * viewport_v);
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        // 计算相机失焦盘的基向量。
        let defocus_radius = self.focus_dist * ((self.defocus_angle / 2.0).to_radians()).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;

        self.sqrt_spp = (self.sample_per_pixel as f32).sqrt() as u32;
        self.recip_sqrt_spp = 1.0 / self.sqrt_spp as f32;
    }

    pub fn render(&mut self, scene: &Scene) {
        self.initialize();

        let width = self.image_width;
        let height = self.image_height;

        let mut image = RgbImage::new(width as u32, height as u32);

        image.par_enumerate_pixels_mut().for_each(|(i, j, pixel)| {
            let mut color = Vector3::new(0.0, 0.0, 0.0);
            for _ in 0..self.sample_per_pixel {
                let ray = self.get_ray(i, j);
                color += self.ray_color(&ray, &scene, self.depth);
            }
            *pixel = color_to_rgb(color / self.sample_per_pixel as f32);
        });

        image.save("image.png").expect("Failed to save image");
    }

    fn get_ray(&self, x: u32, y: u32) -> Ray {
        let offset = self.sample_sqare_stratified(x, y);
        // let offset=sample_square();
        let pixel_sample = self.pixel00_loc
            + (x as f32 + offset.x) * self.pixel_delta_u
            + (y as f32 + offset.y) * self.pixel_delta_v;
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_dir = pixel_sample - ray_origin;
        let ray_time = random_f32();
        Ray::new_with_time(ray_origin, ray_dir, ray_time)
    }

    fn ray_color(&self, ray: &Ray, scene: &Scene, depth: i32) -> Vector3<f32> {
        if depth <= 0 {
            return Vector3::new(0.0, 0.0, 0.0);
        }

        let hit = scene.hit(ray, &Interval::new(0.0001, f32::MAX));

        match hit {
            Some(record) => {
                let color_from_emission = record.material.emitted(&record.uv, &record.p);
                match record.material.scatter(ray, &record) {
                    Some((scattered, attenuation)) => {
                        let scattering_pdf = record.material.pdf(ray, &scattered, &record);
                        // let scattering_pdf = 1.0;
                        let pdf_value = 1.0 / (2.0 * PI);
                        let pdf_value = scattering_pdf;
                        // let pdf_value = scattering_pdf;
                        let color_from_scatter = (scattering_pdf
                            * attenuation.component_mul(&self.ray_color(
                                &scattered,
                                scene,
                                depth - 1,
                            )))
                            / pdf_value;
                        return color_from_emission + color_from_scatter;
                    }
                    None => color_from_emission,
                }
            }
            None => self.background,
        }
    }

    fn defocus_disk_sample(&self) -> Vector3<f32> {
        let p = random_unit_vector();
        self.center + self.defocus_disk_u * p.x + self.defocus_disk_v * p.y
    }

    fn sample_sqare_stratified(&self, x: u32, y: u32) -> Vector3<f32> {
        let px = (x as f32 + random_f32()) * self.recip_sqrt_spp - 0.5;
        let py = (y as f32 + random_f32()) * self.recip_sqrt_spp - 0.5;
        Vector3::new(px, py, 0.0)
    }
}
