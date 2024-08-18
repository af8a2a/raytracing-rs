use image::{Pixel, Rgb, RgbImage};
use nalgebra::{Normed, Vector3};
use pbrt_rs::ray::Ray;

fn ray_color(ray: &Ray) -> Vector3<f32> {
    let t = hit_sphere(Vector3::new(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let n = ray.at(t) - Vector3::new(0.0, 0.0, -1.0);
        return 0.5 * Vector3::new(n.x + 1.0, n.y + 1.0, n.z + 1.0);
    }
    let unit_dir = ray.direction.normalize();
    let t = 0.5 * (unit_dir.y + 1.0);
    (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
}

fn color_to_rgb(color: Vector3<f32>) -> Rgb<u8> {
    Rgb([
        (255.0 * color.x) as u8,
        (255.0 * color.y) as u8,
        (255.0 * color.z) as u8,
    ])
}

fn hit_sphere(center: Vector3<f32>, radius: f32, ray: &Ray) -> f32 {
    let oc = center - ray.origin;
    let a = ray.direction.norm_squared();
    let h = ray.direction.dot(&oc);
    let c = oc.norm_squared() - radius * radius;
    let discriminant = h * h - a * c;

    if discriminant <= 0.0 {
        -1.0
    } else {
        (h - discriminant.sqrt()) / a
    }
}

fn main() {
    let aspect_ratio: f32 = 16.0 / 9.0;

    let width = 400.0;
    let height = (width / aspect_ratio).floor();

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * width / height;
    let camera_center = Vector3::new(0.0, 0.0, 0.0);

    let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / width;
    let pixel_delta_v = viewport_v / height;

    let viewport_upper_left =
        camera_center - Vector3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

    let pixel100_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let mut image = RgbImage::new(width as u32, height as u32);
    for j in 0..height as u32 {
        for i in 0..width as u32 {
            let pixel_center =
                pixel100_loc + (i as f32 * pixel_delta_u) + (j as f32 * pixel_delta_v);

            let ray_dir = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_dir);
            let color = ray_color(&ray);
            image.put_pixel(i, j, color_to_rgb(color));
        }
    }
    image.save("image.png").expect("Failed to save image");
}
