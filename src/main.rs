use std::{io::{stdout, Result}};
use raytracer::{rgb::{write_color, Rgb}, vec3::Vec3, point3::Point3};
use raytracer::ray::Ray;

fn main() -> Result<()> {
    // image
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width = 500;
    let image_height = (image_width as f32 / aspect_ratio) as i32;

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

            write_color(&mut stdout(), ray_color(ray));
        } 
    }

    Ok(())

}

pub fn ray_color(ray: Ray) -> Rgb {
    let t = hit_sphere(Point3::new(0.0,0.0,-1.0), 0.5, ray);
    if t > 0.0 {
        let n = Vec3::unit_vector(ray.at(t) - Vec3::new(0.0, 0.0, -1.0));
        return Rgb::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) * 0.5;
    }
    let unit_direction: Vec3 = ray.direction().unit_vector();
    let t = 0.5*(unit_direction.y() + 1.0);
    Rgb::new(1.0, 1.0, 1.0)*(1.0-t) + Rgb::new(0.5, 0.7, 1.0)*t
} 

pub fn hit_sphere(center: Point3, radius: f32, ray: Ray) -> f32 {
    let oc: Vec3 = ray.origin() - center;
    
    let a = ray.direction().len_sqr();
    let half_b = Vec3::dot(oc, ray.direction());
    let c = oc.len_sqr() - (radius*radius);
    let discriminant = half_b*half_b - a*c;
    if discriminant < 0.0 {
        return - 1.0;
    } else {
        return (-half_b - f32::sqrt(discriminant) ) / a;
    }
}
#[derive(Clone)]
pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f32,
    front_face: bool,
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord{
            p: Vec3::default(),
            normal: Vec3::default(),
            t: 0.0,
            front_face: true,
        }
    }
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: Ray, outword_normal: Vec3) {
        self.front_face = Vec3::dot(ray.direction(), outword_normal) < 0.0;
        self.normal = if self.front_face {outword_normal} else {outword_normal.negative()};
    }
}
pub trait Hittable {
    fn hit(&self, ray: Ray, t_max: f32, t_min: f32, rec: &mut HitRecord) -> bool;
}

#[derive(Clone, Copy)]
pub struct Sphere {
    center: Point3,
    radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_max: f32, t_min: f32, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = ray.origin() - self.center;
        let a = ray.direction().len_sqr();
        let half_b = Vec3::dot(oc, ray.direction());
        let c = oc.len_sqr() - self.radius*self.radius;

        let discriminant = half_b*half_b - a*c;

        if discriminant < 0.0 {return false};
        let sqrtd = f32::sqrt(discriminant);

        // find nearest root in acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal: Vec3 = (rec.p - self.center) /  self.radius;
        rec.set_face_normal(ray, outward_normal);

        return true;
    }
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Sphere {
        Sphere {
            center,
            radius,
        }
    }
}

pub struct HittableList<T: Hittable> {
    objects: Vec<T>,
}

impl<T: Hittable> HittableList<T> {
    pub fn add(&mut self, object: T) {
        self.objects.push(object);
    }
}

impl<T: Hittable> Hittable for HittableList<T> {
    fn hit(&self, ray: Ray, t_max: f32, t_min: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord =  HitRecord::default();
        let mut hit_anything: bool = false;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.clone().t;
                *rec = temp_rec.clone();
            }
        }

        return hit_anything;
    }
}