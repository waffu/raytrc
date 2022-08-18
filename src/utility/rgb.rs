use image::ImageBuffer;
use crate::utility::vec3::Rgb;
use image;

pub fn write_pixel(buffer: &mut ImageBuffer<image::Rgb<u8>, Vec<u8>>, x: u32, y: u32, color: Rgb, samples_per_pixel: i32) {
    let mut r = color.x();
    let mut g = color.y();
    let mut b = color.z();

    let scale = 1.0 / samples_per_pixel as f32;

    r = f32::sqrt(scale * r);
    g = f32::sqrt(scale * g);
    b = f32::sqrt(scale * b);

    let ir = (256 as f32 * clamp(r, 0.0, 0.999)) as u8;
    let ig = (256 as f32 * clamp(g, 0.0, 0.999)) as u8;
    let ib = (256 as f32 * clamp(b, 0.0, 0.999)) as u8;

    buffer.put_pixel(x as u32, y as u32, image::Rgb([ir, ig, ib]));
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    };
    if x > max {
        return max;
    };
    x
}
