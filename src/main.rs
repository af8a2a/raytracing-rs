use image::{Pixel, Rgb, RgbImage};
use nalgebra::{Normed, Vector3};
use pbrt_rs::{
    camera::Camera, hit::{sphere, Hittable}, scene::Scene
};

fn main() {
    let mut scene = Scene::default();
    scene.objects.push(Hittable::Sphere(sphere::Sphere {
        center: Vector3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    }));

    scene.objects.push(Hittable::Sphere(sphere::Sphere {
        center: Vector3::new(0.0, -100.5, -1.0),
        radius: 100.0,
    }));
    let camera = Camera::new(16.0 / 9.0, 400);
    camera.render(&scene);
}
