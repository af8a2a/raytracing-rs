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
    texture::{CheckerTexture, ImageTexture, NoiseTexture, SolidColor, Texture},
    util::{random_f32, random_vec, random_vec_range},
};

fn cornell_box() {
    let mut scene = Scene::default();
    let mut light_scene = Scene::default();
    let red = Material::Diffuse(Lambertian::new_with_color(Vector3::new(0.65, 0.05, 0.05)));
    let white = Material::Diffuse(Lambertian::new_with_color(Vector3::new(0.73, 0.73, 0.73)));
    let green = Material::Diffuse(Lambertian::new_with_color(Vector3::new(0.12, 0.45, 0.15)));
    let light =
        Material::DiffuseLight(DiffuseLight::new_with_color(Vector3::new(15.0, 15.0, 15.0)));

    scene.add(Hittable::Quad(Quad::new(
        Vector3::new(555.0, 0.0, 0.0),
        Vector3::new(0.0, 555.0, 0.0),
        Vector3::new(0.0, 0.0, 555.0),
        green.clone(),
    )));

    scene.add(Hittable::Quad(Quad::new(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 555.0, 0.0),
        Vector3::new(0.0, 0.0, 555.0),
        red.clone(),
    )));

    light_scene.add(Hittable::Quad(Quad::new(
        Vector3::new(343.0, 554.0, 132.0),
        Vector3::new(-130.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, -105.0),
        light.clone(),
    )));

    scene.add(Hittable::Quad(Quad::new(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(555.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));

    scene.add(Hittable::Quad(Quad::new(
        Vector3::new(555.0, 555.0, 555.0),
        Vector3::new(-555.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));

    scene.add(Hittable::Quad(Quad::new(
        Vector3::new(0.0, 0.0, 555.0),
        Vector3::new(555.0, 0.0, 0.0),
        Vector3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let box1 = box_scene(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    let box1 = RotateY::new(Hittable::PrefabScene(box1), 15.0);
    let box1 = Translate::new(Hittable::Rotate(box1), Vector3::new(265.0, 0.0, 295.0));

    scene.add(Hittable::Translate(box1));

    let box2 = box_scene(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(165.0, 165.0, 165.0),
        white.clone(),
    );
    let box2 = RotateY::new(Hittable::PrefabScene(box2), -18.0);
    let box2 = Translate::new(Hittable::Rotate(box2), Vector3::new(130.0, 0.0, 65.0));
    scene.add(Hittable::Translate(box2));
    // println!("{:#?}",scene);
    let mut cam = Camera::default();

    cam.aspect_ratio = 1.0;
    cam.image_width = 400;
    cam.sample_per_pixel = 100;
    cam.depth = 10;
    cam.background = Vector3::default();

    cam.vfov = 40.0;
    cam.look_from = Vector3::new(278.0, 278.0, -800.0);
    cam.look_at = Vector3::new(278.0, 278.0, 0.0);
    cam.vup = Vector3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;
}

fn main() {
    cornell_box();
}
