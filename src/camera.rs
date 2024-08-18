use image::{Rgb, RgbImage};
use nalgebra::{clamp, Vector3};

use crate::{ray::Ray, scene::Scene, util::Interval};

fn ray_color(ray: &Ray, scene: &Scene) -> Vector3<f32> {
    let hit = scene.hit(ray, &Interval::new(0.0, f32::MAX));
    match hit {
        Some(record) => 0.5 * (record.normal + Vector3::new(1.0, 1.0, 1.0)),
        None => {
            let unit_direction = ray.direction.normalize();
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
        }
    }
}

fn color_to_rgb(color: Vector3<f32>) -> Rgb<u8> {
    Rgb([
        (255.9 * color.x) as u8,
        (255.9 * color.y) as u8,
        (255.9 * color.z) as u8,
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
        }
    }

    pub fn render(&self, scene: &Scene) {
        let width = self.image_width;
        let height = self.image_height;
        let pixel00_loc = self.pixel00_loc;
        let pixel_delta_u= self.pixel_delta_u;
        let pixel_delta_v = self.pixel_delta_v;
        let camera_center= self.center;
        let mut image = RgbImage::new(width as u32, height as u32);
        for j in 0..height as u32 {
            for i in 0..width as u32 {
                let pixel_center =
                    pixel00_loc + (i as f32 * pixel_delta_u) + (j as f32 * pixel_delta_v);

                let ray_dir = pixel_center - camera_center;
                let ray = Ray::new(camera_center, ray_dir.normalize());
                let color = ray_color(&ray, &scene);
                image.put_pixel(i, j, color_to_rgb(color));
            }
        }
        image.save("image.png").expect("Failed to save image");
    }
}
