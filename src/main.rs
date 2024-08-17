use image::{Pixel, Rgb, RgbImage};
use nalgebra::Vector3;
use pbrt_rs::ray::Ray;

fn ray_color(ray: &Ray) -> Vector3<f32> {
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

fn main() {
    let aspect_ratio: f32 = 16.0 / 9.0;
    let width = 400.0;
    let height = width / aspect_ratio;

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let origin = Vector3::new(0.0, 0.0, 0.0);

    let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vector3::new(0.0, viewport_height, 0.0);

    let pixel_delta_u = viewport_u / width;
    let pixel_delta_v = viewport_v / height;

    let viewport_upper_left =
        origin - Vector3::new(0.0, 0.0, focal_length) - pixel_delta_u / 2.0 - pixel_delta_v / 2.0;

    let pixel100_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let mut image = RgbImage::new(width as u32, height as u32);
    for j in 0..height as u32 {
        for i in 0..width as u32 {
            let pixel_center =
                pixel100_loc + (i as f32 * pixel_delta_u) + (j as f32 * pixel_delta_v);

            let ray_dir = pixel_center - origin;
            let ray = Ray::new(origin, ray_dir);
            let color = ray_color(&ray);
            image.put_pixel(i, j, color_to_rgb(color));
        }
    }
    image.save("image.png").expect("Failed to save image");
}
