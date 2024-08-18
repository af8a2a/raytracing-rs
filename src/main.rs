use std::f32::consts::PI;

use image::{Pixel, Rgb, RgbImage};
use nalgebra::Vector3;
use pbrt_rs::{
    camera::Camera,
    hit::{sphere, Hittable},
    material::{Dielectric, Lambertian, Material, Metal},
    scene::Scene,
    util::{random_f32, range_random_f32},
};

fn main() {
    let material_ground = Material::Diffuse(Lambertian::new(Vector3::new(0.5, 0.5, 0.5)));

    let mut scene = Scene::default();

    scene.objects.push(Hittable::Sphere(sphere::Sphere {
        center: Vector3::new(0.0, -100.0, 0.0),
        radius: 100.0,
        material: material_ground,
    }));
    for i in -11..11 {
        for j in -11..11 {
            let choose_mat = random_f32();
            let center = Vector3::new(
                i as f32 + 0.9 * random_f32(),
                0.2,
                j as f32 + 0.9 * random_f32(),
            );
            if (center - Vector3::new(4.0, 0.2, 0.0)).norm() > 0.9 {
                let material = if choose_mat < 0.8 {
                    Material::Diffuse(Lambertian::new(Vector3::new(
                        random_f32() * random_f32(),
                        random_f32() * random_f32(),
                        random_f32() * random_f32(),
                    )))
                } else if choose_mat < 0.95 {
                    let fuzz = range_random_f32(0.0, 0.5);
                    Material::Metal(Metal::new(
                        Vector3::new(
                            0.5 * (1.0 + random_f32()),
                            0.5 * (1.0 + random_f32()),
                            0.5 * (1.0 + random_f32()),
                        ),
                        fuzz,
                    ))
                } else {
                    Material::Dielectric(Dielectric::new(1.5))
                };
                scene.objects.push(Hittable::Sphere(sphere::Sphere {
                    center,
                    radius: 0.2,
                    material,
                }))
            }
        }
    }
    let material1 = Material::Dielectric(Dielectric::new(1.5));
    scene.objects.push(Hittable::Sphere(sphere::Sphere {
        center: Vector3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: material1,
    }));
    let material2 = Material::Diffuse(Lambertian::new(Vector3::new(0.4, 0.2, 0.1)));
    scene.objects.push(Hittable::Sphere(sphere::Sphere {
        center: Vector3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: material2,
    }));
    let material3 = Material::Metal(Metal::new(Vector3::new(0.7, 0.6, 0.5), 0.0));
    scene.objects.push(Hittable::Sphere(sphere::Sphere {
        center: Vector3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: material3,
    }));

    let mut camera = Camera::new(16.0 / 9.0, 1200);
    camera.look_from = Vector3::new(13.0, 2.0, 3.0);
    camera.look_at = Vector3::new(0.0, 0.0, -1.0);
    camera.vup = Vector3::new(0.0, 1.0, 0.0);
    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.0;
    camera.vfov = 20.0;
    camera.sample_per_pixel = 32;
    camera.depth = 50;
    camera.reinit();
    camera.render(&scene);
}
