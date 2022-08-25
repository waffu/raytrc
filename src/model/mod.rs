pub mod hit_record;
pub mod hittables;
pub mod material;
pub mod sphere;

use crate::utility::ray::Ray;
use hit_record::HitRecord;

/// Implement this on types representing objects in 3D space in order to 
/// query whether they intersect with a 'Ray' and subsequently, return the details of the intersection
/// through a 'HitRecord' type
pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
