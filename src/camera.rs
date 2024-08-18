use std::cell::OnceCell;

use image::{Rgb, RgbImage};
use nalgebra::{clamp, Vector3};

use crate::{
    ray::Ray,
    scene::Scene,
    util::{random_in_unit_sphere, random_on_hemisphere, sample_square, Interval},
};

fn color_to_rgb(color: Vector3<f32>) -> Rgb<u8> {
    Rgb([
        (256.0 * f32::clamp(color.x, 0.0, 0.999)) as u8,
        (256.0 * f32::clamp(color.y, 0.0, 0.999)) as u8,
        (256.0 * f32::clamp(color.z, 0.0, 0.999)) as u8,
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
}

impl Camera {
    pub fn new(aspect_ratio: f32, image_width: u32) -> Camera {
        let image_height = ((image_width as f32 / aspect_ratio) as u32).max(1);
        let center = Vector3::new(0.0, 0.0, 0.0);

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * image_width as f32 / image_height as f32;

        let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;
        let viewport_upper_left =
            center - Vector3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
        Camera {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            sample_per_pixel: 32,
            depth: 50,
        }
    }

    pub fn render(&self, scene: &Scene) {
        let width = self.image_width;
        let height = self.image_height;
        // let pixel00_loc = self.pixel00_loc;
        // let pixel_delta_u = self.pixel_delta_u;
        // let pixel_delta_v = self.pixel_delta_v;
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
                let dir = record.normal + random_in_unit_sphere();
                return 0.5 * Self::ray_color(&Ray::new(record.p, dir), scene, depth - 1);
            }
            None => {
                let unit_direction = ray.direction.normalize();
                let t = 0.5 * (unit_direction.y + 1.0);
                (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
            }
        }
    }
}
