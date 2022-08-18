use rand::Rng;
use raytracer::hittables::sphere::Sphere;
use raytracer::hittables::Hittable;
use raytracer::utility::ray::Ray;
use raytracer::{
    hittables::{hit_record::HitRecord, hittables::Hittables},
    utility::{camera::Camera, rgb::write_color, vec3::*},
};
use std::io::{stdout, Result};
fn main() -> Result<()> {
    let mut rng = rand::thread_rng();

    // image
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width = 500;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // world
    let mut world: Vec<Hittables> = Vec::new();
    let sph = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
    let sph2 = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0);

    world.push(Hittables::Sphere(sph));
    world.push(Hittables::Sphere(sph2));

    // camera
    let camera = Camera::new();

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprintln!("\rscanlines remaining: {}", j);
        for i in 0..image_width {
            let mut pixel_colour: Rgb = Rgb::new(0.0, 0.0, 0.0);

            for _ in 0..samples_per_pixel {
                let u: f32 = (i as f32 + rng.gen_range(0.0..1.0)) / (image_width as f32);
                let v: f32 = (j as f32 + rng.gen_range(0.0..1.0)) / (image_height as f32);

                let ray: Ray = camera.get_ray(u, v);

                pixel_colour = pixel_colour + cast_ray(ray, &world, max_depth);
            }

            write_color(&mut stdout(), pixel_colour, samples_per_pixel);
        }
    }

    Ok(())
}

pub fn cast_ray(ray: Ray, world: &Vec<Hittables>, depth: i32) -> Rgb {
    let mut rec = HitRecord::new();
    let mut closest_t = f32::MAX;
    let mut hit_obj: Option<&Hittables> = None;

    // Depth of recursion has been reached, representing a fully absorbed light ray (black shadow).
    if depth <= 0 {
        return Rgb::new(0.0, 0.0, 0.0);
    }

    // For every enum in 'world', call the hit implementation, routing to the underlying hit implementation for the relevant structs.
    // This will modify the HitRecord 'rec' with details of the intersection. Use the t_max parameter with the updated rec.t field to update 'hit_obj'
    // until it represents the closest intersection.
    for object in world.iter() {
        if object.hit(ray, 0.001, closest_t, &mut rec) {
            closest_t = rec.t;
            hit_obj = Some(object);
        }
    }

    // If hit_obj has a value, create a random point in a unit sphere normal to the hit intersection,
    // proceed to recursively call 'cast_ray', with the ray originating from the intersection point and directing towards the random point.
    // Along with this, decrease the depth by 1 and multiply the return value by 0.5 to account for perceived light loss.
    match hit_obj {
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
