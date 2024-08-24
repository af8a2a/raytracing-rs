use std::f32::consts::PI;

use image::{Pixel, Rgb, RgbImage};
use nalgebra::Vector3;
use pbrt_rs::{
    bvh::BVHNode,
    camera::Camera,
    hit::{
        quad::Quad,
        sphere::{self, Sphere},
        Hittable,
    },
    material::{Dielectric, Lambertian, Material, Metal},
    scene::Scene,
    texture::{CheckerTexture, ImageTexture, NoiseTexture, SolidColor, Texture},
    util::{random_f32, random_vec, range_random_f32},
};

fn build_random_scene() {
    let checker = Texture::CheckerTexture(CheckerTexture::new_with_color(
        &Vector3::new(0.2, 0.3, 0.1),
        &Vector3::new(0.9, 0.9, 0.9),
        0.32,
    ));
    let material_ground = Material::Diffuse(Lambertian::new(checker));

    let mut scene = Scene::default();

    scene.add(Hittable::Sphere(sphere::Sphere::new(
        Vector3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    )));
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
                    Material::Diffuse(Lambertian::new_with_color(Vector3::new(
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
                // let center2 = center + Vector3::new(0.0, range_random_f32(0.0, 0.5), 0.0);
                scene.add(Hittable::Sphere(sphere::Sphere::new(center, 0.2, material)));
            }
        }
    }
    let material1 = Material::Dielectric(Dielectric::new(1.5));
    scene.add(Hittable::Sphere(sphere::Sphere::new(
        Vector3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Material::Diffuse(Lambertian::new_with_color(Vector3::new(0.4, 0.2, 0.1)));
    scene.add(Hittable::Sphere(sphere::Sphere::new(
        Vector3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Material::Metal(Metal::new(Vector3::new(0.7, 0.6, 0.5), 0.0));
    scene.add(Hittable::Sphere(sphere::Sphere::new(
        Vector3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));
    let scene = Scene::new_with_bvh(pbrt_rs::hit::Hittable::BVHNode(BVHNode::new_with_scene(
        &scene,
    )));
    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 800;

    camera.look_from = Vector3::new(13.0, 2.0, 3.0);
    camera.look_at = Vector3::new(0.0, 0.0, -1.0);
    camera.vup = Vector3::new(0.0, 1.0, 0.0);
    camera.defocus_angle = 0.0;
    camera.focus_dist = 10.0;
    camera.vfov = 20.0;
    camera.sample_per_pixel = 250;
    camera.depth = 50;
    camera.render(&scene);

}

fn checkered_spheres() {
    let mut scene = Scene::default();
    let checker = Texture::CheckerTexture(CheckerTexture::new_with_color(
        &Vector3::new(0.2, 0.3, 0.1),
        &Vector3::new(0.9, 0.9, 0.9),
        0.32,
    ));
    scene.add(Hittable::Sphere(sphere::Sphere::new(
        Vector3::new(0.0, -10.0, 0.0),
        10.0,
        Material::Diffuse(Lambertian::new(checker.clone())),
    )));
    scene.add(Hittable::Sphere(sphere::Sphere::new(
        Vector3::new(0.0, 10.0, 0.0),
        10.0,
        Material::Diffuse(Lambertian::new(checker)),
    )));
    let scene = Scene::new_with_bvh(pbrt_rs::hit::Hittable::BVHNode(BVHNode::new_with_scene(
        &scene,
    )));
    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;

    camera.look_from = Vector3::new(0.0, 0.0, 12.0);
    camera.look_at = Vector3::new(0.0, 0.0, 0.0);
    camera.vup = Vector3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.0;
    camera.focus_dist = 10.0;
    camera.vfov = 20.0;
    camera.sample_per_pixel = 100;
    camera.depth = 50;
    camera.render(&scene);
}
fn earth() {
    let image = image::open("assets/earthmap.jpg")
        .expect("fail to open")
        .into_rgb8();

    let earth_texture = ImageTexture::new(image);
    let earth_surface = Material::Diffuse(Lambertian::new(Texture::ImageTexture(earth_texture)));
    let globe = Sphere::new(Vector3::new(0.0, 0.0, 0.0), 2.0, earth_surface);
    let mut scene = Scene::default();
    scene.add(Hittable::Sphere(globe));

    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;

    camera.look_from = Vector3::new(0.0, 0.0, 12.0);
    camera.look_at = Vector3::new(0.0, 0.0, 0.0);
    camera.vup = Vector3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.0;
    camera.focus_dist = 10.0;
    camera.vfov = 20.0;
    camera.sample_per_pixel = 100;
    camera.depth = 50;
    camera.render(&scene);
}

fn perlin_sphere() {
    let mut scene = Scene::default();
    let pertext = NoiseTexture::new(4.0);

    scene.add(Hittable::Sphere(sphere::Sphere::new(
        Vector3::new(0.0, -100.0, 0.0),
        100.0,
        Material::Diffuse(Lambertian::new(Texture::Noise(pertext.clone()))),
    )));

    scene.add(Hittable::Sphere(sphere::Sphere::new(
        Vector3::new(0.0, 2.0, 0.0),
        2.0,
        Material::Diffuse(Lambertian::new(Texture::Noise(pertext))),
    )));

    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;

    camera.look_from = Vector3::new(0.0, 0.0, 12.0);
    camera.look_at = Vector3::new(0.0, 0.0, 0.0);
    camera.vup = Vector3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.0;
    camera.focus_dist = 10.0;
    camera.vfov = 20.0;
    camera.sample_per_pixel = 100;
    camera.depth = 50;
    camera.render(&scene);
}

fn quads() {
    let mut scene = Scene::default();
    let left_red = Material::Diffuse(Lambertian::new_with_color(Vector3::new(1.0, 0.2, 0.2)));
    let back_green = Material::Diffuse(Lambertian::new_with_color(Vector3::new(0.2, 1.0, 0.2)));
    let right_blue = Material::Diffuse(Lambertian::new_with_color(Vector3::new(0.2, 0.2, 1.0)));
    let upper_orange = Material::Diffuse(Lambertian::new_with_color(Vector3::new(1.0, 0.5, 0.0)));
    let lower_teal = Material::Diffuse(Lambertian::new_with_color(Vector3::new(0.2, 0.8, 0.8)));

    scene.add(Hittable::Quad(Quad::new(
        Vector3::new(-3.0, -2.0, 5.0),
        Vector3::new(0.0, 0.0, -4.0),
        Vector3::new(0.0, 4.0, 0.0),
        left_red,
    )));

    scene.add(Hittable::Quad(Quad::new(
        Vector3::new(-2.0, -2.0, 0.0),
        Vector3::new(4.0, 0.0, 0.0),
        Vector3::new(0.0, 4.0, 0.0),
        back_green,
    )));
    scene.add(Hittable::Quad(Quad::new(
        Vector3::new(3.0, -2.0, 1.0),
        Vector3::new(0.0, 0.0, 4.0),
        Vector3::new(0.0, 4.0, 0.0),
        right_blue,
    )));
    scene.add(Hittable::Quad(Quad::new(
        Vector3::new(-2.0, 3.0, 1.0),
        Vector3::new(4.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 4.0),
        upper_orange,
    )));
    scene.add(Hittable::Quad(Quad::new(
        Vector3::new(-2.0, -3.0, 5.0),
        Vector3::new(4.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, -4.0),
        lower_teal,
    )));

    let mut camera = Camera::default();
    camera.look_from = Vector3::new(0.0, 0.0, 9.0);
    camera.look_at = Vector3::new(0.0, 0.0, 0.0);
    camera.vup = Vector3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.0;
    camera.vfov = 80.0;
    camera.sample_per_pixel = 160;
    camera.depth = 50;
    camera.render(&scene);
}

fn main() {
    quads();
}
