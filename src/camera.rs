use std::cell::OnceCell;

use image::{Rgb, RgbImage};
use nalgebra::{clamp, Vector3};

use crate::{
    ray::Ray,
    scene::Scene,
    util::{random_in_unit_sphere, random_on_hemisphere, sample_square, Interval},
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
    sample_per_pixel: u32,
    depth: i32,

    pub vfov: f32,
    pub look_from: Vector3<f32>,
    pub look_at: Vector3<f32>,
    pub vup: Vector3<f32>,

    u: Vector3<f32>,
    v: Vector3<f32>,
    w: Vector3<f32>,
}

impl Camera {
    pub fn new(aspect_ratio: f32, image_width: u32) -> Camera {
        let vfov = 90.0;
        let look_from = Vector3::new(0.0, 0.0, 0.0);
        let look_at = Vector3::new(0.0, 0.0, -1.0);
        let vup = Vector3::new(0.0, 1.0, 0.0);

        let image_height = ((image_width as f32 / aspect_ratio) as u32).max(1);
        let center = look_from.clone();

        let focal_length = (look_from - look_at).norm();
        let theta = f32::to_radians(vfov);
        let h = f32::tan(theta / 2.0);

        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * image_width as f32 / image_height as f32;

        let w = (look_from - look_at).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;
        let viewport_upper_left = center - (focal_length * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            sample_per_pixel: 100,
            depth: 50,
            vfov,
            look_from,
            look_at,
            vup,
            u,
            v,
            w,
        }
    }
    pub fn reinit(&mut self) {
        self.image_height = ((self.image_width as f32 / self.aspect_ratio) as u32).max(1);
        self.center = self.look_from.clone();
        let focal_length = (self.look_from - self.look_at).norm();
        let theta = f32::to_radians(self.vfov);
        let h = f32::tan(theta / 2.0);

        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * self.image_width as f32 / self.image_height as f32;

        self.w = (self.look_from - self.look_at).normalize();
        self.u = self.vup.cross(&self.w).normalize();
        self.v = self.w.cross(&self.u);

        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        self.pixel_delta_u = viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;
        let viewport_upper_left =
            self.center - (focal_length * self.w) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }
    pub fn render(&self, scene: &Scene) {
        let width = self.image_width;
        let height = self.image_height;
        // let camera_center = self.center;
        let mut image = RgbImage::new(width as u32, height as u32);
        for j in 0..height as u32 {
            for i in 0..width as u32 {
                let mut color = Vector3::new(0.0, 0.0, 0.0);
                for _ in 0..self.sample_per_pixel {
                    let ray = self.get_ray(i, j);
                    color += Self::ray_color(&ray, &scene, self.depth);
                }

                image.put_pixel(i, j, color_to_rgb(color / self.sample_per_pixel as f32));
            }
        }
        image.save("image_32spp.png").expect("Failed to save image");
    }

    fn get_ray(&self, x: u32, y: u32) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel00_loc
            + (x as f32 + offset.x) * self.pixel_delta_u
            + (y as f32 + offset.y) * self.pixel_delta_v;
        let ray_origin = self.center;
        let ray_dir = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_dir.normalize())
    }

    fn ray_color(ray: &Ray, scene: &Scene, depth: i32) -> Vector3<f32> {
        if depth <= 0 {
            return Vector3::new(0.0, 0.0, 0.0);
        }
        let hit = scene.hit(ray, &Interval::new(0.00000001, f32::MAX));
        match hit {
            Some(record) => {
                if let Some((scattered, attenuation)) = record.material.scatter(ray, &record) {
                    return attenuation.component_mul(&Self::ray_color(
                        &scattered,
                        scene,
                        depth - 1,
                    ));
                }

                return Vector3::new(0.0, 0.0, 0.0);
            }
            None => {
                let unit_direction = ray.direction.normalize();
                let t = 0.5 * (unit_direction.y + 1.0);
                (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
            }
        }
    }
}
