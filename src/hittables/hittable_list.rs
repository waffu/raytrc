use super::Hittable;
use super::hit_record::HitRecord;
use crate::utility::ray::Ray;

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