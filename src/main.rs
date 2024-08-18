use image::{Pixel, Rgb, RgbImage};
use nalgebra::{Normed, Vector3};
use pbrt_rs::{
    hit::{sphere, Hittable},
    ray::Ray,
    scene::Scene,
};

fn ray_color(ray: &Ray, scene: &Scene) -> Vector3<f32> {
    let hit = scene.hit(ray, 0.0, f32::MAX);
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

    let mut scene = Scene::default();
    scene.objects.push(Hittable::Sphere(sphere::Sphere {
        center: Vector3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    }));

    scene.objects.push(Hittable::Sphere(sphere::Sphere {
        center: Vector3::new(0.0, -100.5, -1.0),
        radius: 100.0,
    }));

    let write_png=||{
        let mut image = RgbImage::new(width as u32, height as u32);
        for j in 0..height as u32 {
            for i in 0..width as u32 {
                let pixel_center =
                    pixel100_loc + (i as f32 * pixel_delta_u) + (j as f32 * pixel_delta_v);
    
                let ray_dir = pixel_center - camera_center;
                let ray = Ray::new(camera_center, ray_dir.normalize());
                let color = ray_color(&ray, &scene);
                image.put_pixel(i, j, color_to_rgb(color));
            }
        }
        image.save("image.png").expect("Failed to save image");    
    };
    let write_ppm=||{
        println!("P3\n{} {}\n255", width, height);
        for j in 0..height as u32 {
            for i in 0..width as u32 {
                let pixel_center =
                    pixel100_loc + (i as f32 * pixel_delta_u) + (j as f32 * pixel_delta_v);
    
                let ray_dir = pixel_center - camera_center;
                let ray = Ray::new(camera_center, ray_dir.normalize());
                let color = ray_color(&ray, &scene);
                let r=(color.x*255.99) as u8;
                let g=(color.y*255.99) as u8;
                let b=(color.z*255.99) as u8;
                println!("{} {} {}",r,g,b);
            }
        }

    };
    // write_ppm();
    write_png();
}
