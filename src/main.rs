use image::imageops::flip_vertical_in_place;
use image::RgbImage;
use log::info;
use rand::Rng;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;
use std::io::Result;
use waytracer::hittables::material::{Lambertian, Materials, Metal};
use waytracer::hittables::sphere::Sphere;
use waytracer::utility::ray::Ray;
use waytracer::utility::rgb::write_pixel;
use waytracer::{
    hittables::hittables::Hittables,
    utility::{camera::Camera, vec3::*},
};

fn main() -> Result<()> {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Trace)
        .init();

    // image
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width = 1300;
    let image_height = (image_width as f32 / aspect_ratio) as u32;
    let samples_per_pixel = 500;
    let max_depth = 50;

    let mut buffer = RgbImage::new(image_width, image_height);

    // world
    let mut world: Vec<Hittables> = Vec::new();

    let material_ground = Materials::Lambertian(Lambertian {
        albedo: Rgb::new(0.8, 0.8, 0.0),
    });
    let material_center = Materials::Lambertian(Lambertian {
        albedo: Rgb::new(0.7, 0.3, 0.3),
    });
    let material_left = Materials::Metal(Metal {
        albedo: Rgb::new(0.8, 0.8, 0.8),
        fuzz: 0.3,
    });
    let material_right = Materials::Metal(Metal {
        albedo: Rgb::new(0.8, 0.6, 0.2),
        fuzz: 1.0,
    });

    let sph = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground);
    let sph2 = Sphere::new(Point3::new(0.0, 0.0, -2.0), 0.5, material_center);
    let sph3 = Sphere::new(Point3::new(-1.0, 0.0, -2.0), 0.5, material_left);
    let sph4 = Sphere::new(Point3::new(1.0, 0.0, -2.0), 0.5, material_right);

    world.push(Hittables::Sphere(sph));
    world.push(Hittables::Sphere(sph2));
    world.push(Hittables::Sphere(sph3));
    world.push(Hittables::Sphere(sph4));

    // camera
    let camera = Camera::new();

    // log info
    info!("starting render...");
    info!("dimensions: {} {}", image_width, image_height);
    info!("samples per pixel: {}", samples_per_pixel);
    info!("maximum ray depth: {}", max_depth);
    info!("with {} objects", world.len());

    for j in (0..image_height).rev() {
        // Might slow down render a tiny bit but I like the output :)
        if j % 20 == 0 {
            info!("scanlines remaining: {}", j);
        }
        // For every pixel, generate 
        for i in 0..image_width {
            let pixel_colour: Rgb = (0..samples_per_pixel)
                .into_par_iter()
                .map(|_| {
                    let mut rng = rand::thread_rng();
                    let u: f32 = (i as f32 + rng.gen_range(0.0..1.0)) / (image_width as f32);
                    let v: f32 = (j as f32 + rng.gen_range(0.0..1.0)) / (image_height as f32);

                    let ray: Ray = camera.get_ray(u, v);

                    ray.cast_ray(&world, max_depth)
                })
                .sum();

            write_pixel(&mut buffer, i, j, pixel_colour, samples_per_pixel);
        }
    }

    info!("image saved in buffer");

    // Flip image buffer to account for rays starting at bottom left corner.
    flip_vertical_in_place(&mut buffer);

    match buffer.save_with_format("img.png", image::ImageFormat::Png) {
        Ok(_) => info!("image written to file"),
        Err(e) => println!("{:?}", e),
    }

    Ok(())
}