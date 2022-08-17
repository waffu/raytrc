use std::{io::{stdout, Result}};
use rand::Rng;
use raytracer::{utility::{vec3::*, rgb::write_color, camera::Camera}, hittables::{hittables::Hittables, hit_record::HitRecord}};
use raytracer::utility::ray::Ray;
use raytracer::hittables::Hittable;
use raytracer::hittables::sphere::Sphere;


// use raytracer::{rgb::{write_color, Rgb}, vec3::Vec3, point3::Point3};
// use raytracer::ray::Ray;

fn main() -> Result<()> {

    let mut rng = rand::thread_rng();

    // image
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width = 500;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let samples_per_pixel = 100;

    // world
    let mut world: Vec<Hittables> = Vec::new();
    let sph = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
    let sph2 = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0);
    
    world.push(Hittables::Sphere(sph));
    world.push(Hittables::Sphere(sph2));
    
    // camera
    let cam = Camera::new();



    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprintln!("\rscanlines remaining: {}", j);
        for i in 0..image_width {

            let mut pixel_colour: Rgb = Rgb::new(0.0, 0.0, 0.0);


            for _ in 0..samples_per_pixel {
                let u: f32 = (i as f32 + rng.gen_range(0.0..1.0)) / (image_width) as f32;
                let v: f32 = (j as f32 + rng.gen_range(0.0..1.0)) / (image_height) as f32;

                let ray: Ray = cam.get_ray(u, v);

                pixel_colour = pixel_colour + ray_color(ray, &world);

            };

            write_color(&mut stdout(), pixel_colour, samples_per_pixel);
        } 
    }

    Ok(())

}

pub fn ray_color(ray: Ray, world: &Vec<Hittables>) -> Rgb {
    let mut rec: HitRecord = HitRecord::new();

    for object in world.iter() {
        if object.hit(ray, 0.0, f32::MAX, &mut rec) {
            return (rec.normal + Rgb::new(1.0, 1.0, 1.0)) * 0.5;
        }
    }
    let unit_direction: Vec3 = ray.direction().unit_vector();
    let t = 0.5*(unit_direction.y() + 1.0);
    Rgb::new(1.0, 1.0, 1.0)*(1.0-t) + Rgb::new(0.5, 0.7, 1.0)*t
} 