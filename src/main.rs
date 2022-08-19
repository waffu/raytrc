use image::imageops::flip_vertical_in_place;
use image::RgbImage;
use log::info;
use rand::Rng;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;
use std::io::Result;
use waytracer::hittables::material::{Lambertian, Material, Materials, Metal};
use waytracer::hittables::sphere::Sphere;
use waytracer::hittables::Hittable;
use waytracer::utility::ray::Ray;
use waytracer::utility::rgb::write_pixel;
use waytracer::{
    hittables::{hit_record::HitRecord, hittables::Hittables},
    utility::{camera::Camera, vec3::*},
};

fn main() -> Result<()> {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Trace)
        .init();

    // image
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width = 1000;
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
        for i in 0..image_width {
            let pixel_colour: Rgb = (0..samples_per_pixel)
                .into_par_iter()
                .map(|_| {
                    let mut rng = rand::thread_rng();
                    let u: f32 = (i as f32 + rng.gen_range(0.0..1.0)) / (image_width as f32);
                    let v: f32 = (j as f32 + rng.gen_range(0.0..1.0)) / (image_height as f32);

                    let ray: Ray = camera.get_ray(u, v);

                    cast_ray(ray, &world, max_depth)
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

pub fn cast_ray(ray: Ray, world: &Vec<Hittables>, depth: i32) -> Rgb {

    // Depth of recursion has been reached, representing a fully absorbed light ray (black shadow).
    if depth <= 0 {
        return Rgb::new(0.0, 0.0, 0.0);
    }

    // Produce a HitRecord for the closest intersection for a given ray, calculate RGB based
    match get_obj_closest_intersection(ray, world) {
        Some(rec) => {
            let mut scattered = Ray::new(Point3::default(), Vec3::default());
            let mut attenuation = Rgb::default();
            if rec
                .mat
                .scatter(&ray, rec.clone(), &mut attenuation, &mut scattered)
            {
                return attenuation * cast_ray(scattered, world, depth - 1);
            }
            return Rgb::new(0.0, 0.0, 0.0);
        }
        None => {
            let unit_direction: Vec3 = ray.direction().unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            return Rgb::new(1.0, 1.0, 1.0) * (1.0 - t) + Rgb::new(0.5, 0.7, 1.0) * t
        }
    }
}

pub fn get_obj_closest_intersection<'a>(
    ray: Ray,
    world: &'a Vec<Hittables>
) -> Option<HitRecord> {

    let mut temp_rec: Option<HitRecord> = None;
    let mut closest_t = f32::MAX;

    for object in world.iter() {
        if let Some(rec) = object.hit(ray, 0.001, closest_t) {
            closest_t = rec.t;
            temp_rec = Some(rec);
        }
    }

    temp_rec
}
