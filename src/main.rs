use std::f32::consts::PI;

use image::{Pixel, Rgb, RgbImage};
use nalgebra::Vector3;
use pbrt_rs::{
    camera::Camera,
    hit::{sphere, Hittable},
    material::{Dielectric, Lambertian, Material, Metal},
    scene::Scene,
};

fn main() {
    let material_ground = Material::Diffuse(Lambertian::new(Vector3::new(0.8, 0.8, 0.0)));
    let material_center = Material::Diffuse(Lambertian::new(Vector3::new(0.1, 0.2, 0.5)));
    let material_left = Material::Dielectric(Dielectric::new(1.5));
    let material_bubble = Material::Dielectric(Dielectric::new(1.00 / 1.50));
    let material_right = Material::Metal(Metal::new(Vector3::new(0.8, 0.6, 0.2), 1.0));

    let mut scene = Scene::default();

    scene.objects.push(Hittable::Sphere(sphere::Sphere {
        center: Vector3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: material_ground,
    }));

    scene.objects.push(Hittable::Sphere(sphere::Sphere {
        center: Vector3::new(0.0, 0.0, -1.2),
        radius: 0.5,
        material: material_center,
    }));
    scene.objects.push(Hittable::Sphere(sphere::Sphere {
        center: Vector3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: material_left,
    }));
    scene.objects.push(Hittable::Sphere(sphere::Sphere {
        center: Vector3::new(-1.0, 0.0, -1.0),
        radius: 0.4,
        material: material_bubble,
    }));

    scene.objects.push(Hittable::Sphere(sphere::Sphere {
        center: Vector3::new(1.0, 0.0, -1.0),
        radius: 0.5,
        material: material_right,
    }));
    let mut camera = Camera::new(16.0 / 9.0, 400);
    camera.look_from = Vector3::new(-2.0, 2.0, 1.0);
    camera.look_at = Vector3::new(0.0, 0.0, -1.0);
    camera.vup = Vector3::new(0.0, 1.0, 0.0);
    // camera.vfov = 20.0;
    camera.reinit();
    camera.render(&scene);
}
