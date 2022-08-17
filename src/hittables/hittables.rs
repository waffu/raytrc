use super::Hittable;
use super::hit_record::HitRecord;
use super::sphere::Sphere;
use crate::utility::ray::Ray;


pub enum Hittables {
    Sphere(Sphere),
}

// route to correct trait implementor
// saves using dynamic dispatch (a LOT slower)
impl Hittable for Hittables {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {

        match self {
            Hittables::Sphere(sphere) => {
                let mut temp_rec: HitRecord =  HitRecord::default();
                let mut hit_anything: bool = false;
                let mut closest_so_far = t_max;

                if sphere.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                    hit_anything = true;
                    closest_so_far = temp_rec.clone().t;
                    *rec = temp_rec.clone();
                }

                return hit_anything;
            }
        }
    }
}