use std::f32::consts::PI;

use image::{Pixel, Rgb, RgbImage};
use nalgebra::Vector3;
use pbrt_rs::{
    bvh::BVHNode,
    camera::Camera,
    hit::{
        medium::ConstMedium,
        quad::{box_scene, Quad},
        sphere::{self, Sphere},
        translate::{RotateY, Translate},
        Hittable,
    },
    material::{Dielectric, DiffuseLight, Lambertian, Material, Metal},
    scene::Scene,
};

fn cornell_box() {
    let mut world = Scene::default();

    let red = Material::Diffuse(Lambertian::new_with_color((Vector3::new(0.65, 0.05, 0.05))));
    let white = Material::Diffuse(Lambertian::new_with_color(Vector3::new(0.73, 0.73, 0.73)));
    let green = Material::Diffuse(Lambertian::new_with_color(Vector3::new(0.12, 0.45, 0.15)));
    let light = Material::DiffuseLight(DiffuseLight::new_with_color(Vector3::new(15.0, 15.0, 15.0)));

    world.add(Hittable::Quad(Quad::new(
        Vector3::new(555.0, 0.0, 0.0),
        Vector3::new(0.0, 555.0, 0.0),
        Vector3::new(0.0, 0.0, 555.0),
        green,
    )));
    world.add(Hittable::Quad(Quad::new(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 555.0, 0.0),
        Vector3::new(0.0, 0.0, 555.0),
        red,
    )));
    world.add(Hittable::Quad(Quad::new(
        Vector3::new(343.0, 554.0, 332.0),
        Vector3::new(-130.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, -105.0),
        light.clone(),
    )));
    world.add(Hittable::Quad(Quad::new(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(555.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Hittable::Quad(Quad::new(
        Vector3::new(555.0, 555.0, 555.0),
        Vector3::new(-555.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Hittable::Quad(Quad::new(
        Vector3::new(0.0, 0.0, 555.0),
        Vector3::new(555.0, 0.0, 0.0),
        Vector3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let box1 = Hittable::PrefabScene(box_scene(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    let box1 = Hittable::Rotate(RotateY::new(box1, 15.0));
    let box1 =
        Hittable::Translate(Translate::new(box1, Vector3::new(265.0, 0.0, 295.0)));
    world.add(box1);

    let glass = Material::Dielectric(Dielectric::new(1.5));
    world.add(Hittable::Sphere(Sphere::new(
        Vector3::new(190.0, 90.0, 190.0),
        90.0,
        glass.clone(),
    )));

    // Light Sources.
    let mut lights = Scene::default();
    lights.add(Hittable::Quad(Quad::new(
        Vector3::new(343.0, 554.0, 332.0),
        Vector3::new(-130.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, -105.0),
        light.clone(),
    )));
    lights.add(Hittable::Sphere(Sphere::new(
        Vector3::new(190.0, 90.0, 190.0),
        90.0,
        glass.clone(),
    )));

    let mut cam = Camera::default();

    cam.aspect_ratio = 1.0;
    cam.image_width = 400;
    cam.sample_per_pixel =50;
    cam.depth = 10;
    cam.background = Vector3::default();

    cam.vfov = 40.0;
    cam.look_from = Vector3::new(278.0, 278.0, -800.0);
    cam.look_at = Vector3::new(278.0, 278.0, 0.0);
    cam.vup = Vector3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(
        Hittable::PrefabScene(world),
        Hittable::PrefabScene(lights),
    );
}

fn main() {
    cornell_box();
}
