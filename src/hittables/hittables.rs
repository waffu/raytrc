use super::hit_record::HitRecord;
use super::sphere::Sphere;
use super::Hittable;
use crate::utility::ray::Ray;

pub enum Hittables {
    Sphere(Sphere),
}

// route to correct trait implementor
// saves using dynamic dispatch (a LOT slower)
impl Hittable for Hittables {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self {
            Hittables::Sphere(sphere) => sphere.hit(ray, t_min, t_max),
        }
    }
}
