use super::{hit_record::HitRecord, Hittable};
use crate::utility::ray::Ray;
use crate::utility::vec3::{Point3, Vec3};

#[derive(Clone, Copy)]
pub struct Sphere {
    center: Point3,
    radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = ray.origin() - self.center;
        let a = ray.direction().len_sqr();
        let half_b = Vec3::dot(oc, ray.direction());
        let c = oc.len_sqr() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return false;
        };
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
        rec.point = ray.at(rec.t);
        let outward_normal: Vec3 = (rec.point - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);

        return true;
    }
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}
