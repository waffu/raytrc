use image::RgbImage;
use log::info;
use rand::Rng;
use rayon::prelude::IntoParallelIterator;
use waytracer::hittables::sphere::Sphere;
use waytracer::hittables::Hittable;
use waytracer::utility::ray::Ray;
use waytracer::utility::rgb::write_pixel;
use waytracer::{
    hittables::{hit_record::HitRecord, hittables::Hittables},
    utility::{camera::Camera, vec3::*},
};
use std::io::{Result};
use rayon::iter::ParallelIterator;
use image::imageops::flip_vertical_in_place;

fn main() -> Result<()> {

    env_logger::Builder::new().filter_level(log::LevelFilter::Trace).init();

    // image
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width = 100;
    let image_height = (image_width as f32 / aspect_ratio) as u32;
    let samples_per_pixel = 1000;
    let max_depth = 50;

    let mut buffer = RgbImage::new(image_width, image_height);

    // world
    let mut world: Vec<Hittables> = Vec::new();
    let sph = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
    let sph2 = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0);

    world.push(Hittables::Sphere(sph));
    world.push(Hittables::Sphere(sph2));

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

            let pixel_colour: Rgb = (0..samples_per_pixel).into_par_iter().map(|_| {

                let mut rng = rand::thread_rng();
                let u: f32 = (i as f32 + rng.gen_range(0.0..1.0)) / (image_width as f32);
                let v: f32 = (j as f32 + rng.gen_range(0.0..1.0)) / (image_height as f32);

                let ray: Ray = camera.get_ray(u, v);

                cast_ray(ray, &world, max_depth)

            }).sum();

            write_pixel(&mut buffer, i, j, pixel_colour, samples_per_pixel);
        }
    }

    info!("image saved in buffer");

    // Flip image buffer to account for rays starting at bottom left corner.
    flip_vertical_in_place(&mut buffer);

    match buffer.save_with_format("img.png", image::ImageFormat::Png) {
        Ok(_) => info!("image written to file"),
        Err(e) => println!("{:?}", e)
    }

    Ok(())
}

pub fn cast_ray(ray: Ray, world: &Vec<Hittables>, depth: i32) -> Rgb {
    let mut rec = HitRecord::new();

    // Depth of recursion has been reached, representing a fully absorbed light ray (black shadow).
    if depth <= 0 {
        return Rgb::new(0.0, 0.0, 0.0);
    }

    // If get_obj_closest_intersection has a value, create a random point in a unit sphere normal to the hit intersection,
    // proceed to recursively call 'cast_ray', with the ray originating from the intersection point and directing towards the random point.
    // Along with this, decrease the depth by 1 and multiply the return value by 0.5 to account for perceived light loss.
    match get_obj_closest_intersection(ray, world, &mut rec) {
        Some(_) => {
            let target: Point3 = rec.point + rec.normal + Vec3::random_in_unit_sphere();
            return cast_ray(Ray::new(rec.point, target - rec.point), world, depth - 1) * 0.5;
        }

        None => {
            let unit_direction: Vec3 = ray.direction().unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            Rgb::new(1.0, 1.0, 1.0) * (1.0 - t) + Rgb::new(0.5, 0.7, 1.0) * t
        }
    }
}


// For every enum in 'world', call the hit implementation, routing to the underlying hit implementation for the relevant struct.
// This will modify the HitRecord 'rec' with details of the intersection. Use the t_max parameter with the updated rec.t field to update 'hit_obj'
// until it represents the closest intersection.
pub fn get_obj_closest_intersection<'a>(ray: Ray, world: &'a Vec<Hittables>, rec: &mut HitRecord) -> Option<&'a Hittables> {

    let mut hit_obj: Option<&Hittables> = None;
    let mut closest_t = f32::MAX;

    for object in world.iter() {
        if object.hit(ray, 0.001, closest_t, rec) {
            closest_t = rec.t;
            hit_obj = Some(object);
        }
    }
    
    hit_obj
}