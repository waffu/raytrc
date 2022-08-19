pub mod hit_record;
pub mod hittables;
pub mod material;
pub mod sphere;

use crate::utility::ray::Ray;
use hit_record::HitRecord;

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}
