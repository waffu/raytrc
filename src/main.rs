use image::imageops::flip_vertical_in_place;
use image::RgbImage;
use log::info;
use rand::Rng;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;
use std::io::{Result};
use waytracer::utility::ray::Ray;
use waytracer::utility::rgb::write_pixel;
use waytracer::{
    model::hittables::Hittables,
    utility::{camera::Camera, vec3::*},
};
use waytracer::utility::settings::Settings;

fn main() -> Result<()> {

    // Setup logger
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();

    // Create a new Settings from 'settings.toml'
    let settings = Settings::new();

    // Image buffer for rendering
    let mut buffer = RgbImage::new(settings.image_width, settings.image_height);

    // Read world from JSON
    info!("reading and serializing world.json");
    let json = std::fs::read_to_string("world.json")?;
    let world: Vec<Hittables>  = serde_json::from_str(&json)?;

    // Setup camera
    let camera = Camera::new();

    info!("starting render...");
    info!("dimensions: {}, {}", settings.image_width, settings.image_height);
    info!("samples per pixel: {}", settings.samples_per_pixel);
    info!("maximum ray depth: {}", settings.max_depth);
    info!("amount of objects: {}", world.len());

    // Start rendering
    for j in (0..settings.image_height).rev() {
        // Might slow down render a tiny bit but I like the output :)
        if j % 20 == 0 {
            info!("scanlines remaining: {}", j);
        }
        // For every pixel, generate an RGB value through recursively raytracing
        // with multi-sampling and gamma correction
        for i in 0..settings.image_width {
            let pixel_colour: Rgb = (0..settings.samples_per_pixel)
                .into_par_iter()
                .map(|_| {
                    let mut rng = rand::thread_rng();
                    let u: f32 = (i as f32 + rng.gen_range(0.0..1.0)) / (settings.image_width as f32);
                    let v: f32 = (j as f32 + rng.gen_range(0.0..1.0)) / (settings.image_height as f32);

                    let ray: Ray = camera.get_ray(u, v);

                    ray.cast_ray(&world, settings.max_depth)
                })
                .sum();

            write_pixel(&mut buffer, i, j, pixel_colour, settings.samples_per_pixel);
        }
    }

    info!("image saved in buffer");

    // Flip image buffer to account for rays starting at bottom left corner
    flip_vertical_in_place(&mut buffer);

    match buffer.save(format!("img.{}", settings.file_type)) {
        Ok(_) => info!("image written to file"),
        Err(e) => println!("{:?}", e),
    }

    Ok(())
}