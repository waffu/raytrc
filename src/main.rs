use std::{io::{stdout, Result}};
use raytracer::{utility::{vec3::*, rgb::write_color}, hittables::{hittables::Hittables, hit_record::HitRecord}};
use raytracer::utility::ray::Ray;
use raytracer::hittables::Hittable;
use raytracer::hittables::sphere::Sphere;


// use raytracer::{rgb::{write_color, Rgb}, vec3::Vec3, point3::Point3};
// use raytracer::ray::Ray;

fn main() -> Result<()> {
    // image
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width = 500;
    let image_height = (image_width as f32 / aspect_ratio) as i32;

    // world
    let mut world: Vec<Hittables> = Vec::new();
    let sph = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
    let sph2 = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0);
    
    world.push(Hittables::Sphere(sph));
    world.push(Hittables::Sphere(sph2));
    

    // camera
    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    // camera vecs
    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length);



    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprintln!("\rscanlines remaining: {}", j);
        for i in 0..image_width {

            // ratio of current pos
            let u: f32 = (i as f32) / (image_width) as f32;
            let v: f32 = (j as f32) / (image_height) as f32;

            let ray = Ray::new(origin, lower_left_corner + horizontal*u + vertical*v - origin);

            write_color(&mut stdout(), ray_color(ray, &world));
        } 
    }

    Ok(())

}

pub fn ray_color(ray: Ray, world: &Vec<Hittables>) -> Rgb {
    let mut rec: HitRecord = HitRecord::new();

    for object in world.iter() {
        if object.hit(ray, 0.0, f32::MAX, &mut rec) {
            return rec.normal + Rgb::new(1.0, 1.0, 1.0) * 0.5;
        }
    }
    let unit_direction: Vec3 = ray.direction().unit_vector();
    let t = 0.5*(unit_direction.y() + 1.0);
    Rgb::new(1.0, 1.0, 1.0)*(1.0-t) + Rgb::new(0.5, 0.7, 1.0)*t
} 