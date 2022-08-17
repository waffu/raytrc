pub mod hittables;
pub mod sphere;
pub mod hit_record;

use hit_record::HitRecord;
use crate::utility::ray::Ray;



pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}