use image::{Pixel, Rgb, RgbImage};

fn main() {
    let (width, height) = (800, 600);
    let mut image = RgbImage::new(width, height);
    for j in 0..height {
        for i in 0..width {
            image.put_pixel(
                i,
                j,
                Rgb([
                    (255.0 * i as f32 / (width - 1) as f32) as u8,
                    (255.0 * j as f32 / (height - 1) as f32) as u8,
                    0,
                ]),
            );
        }
    }
    image.save("image.png").expect("Failed to save image");
}
