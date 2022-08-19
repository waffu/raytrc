use super::material::Materials;
use super::{hit_record::HitRecord, Hittable};
use crate::utility::ray::Ray;
use crate::utility::vec3::{Point3, Vec3};

#[derive(Clone)]
pub struct Sphere {
    center: Point3,
    radius: f32,
    material: Materials,
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc: Vec3 = ray.origin() - self.center;
        let a = ray.direction().len_sqr();
        let half_b = Vec3::dot(oc, ray.direction());
        let c = oc.len_sqr() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        };
        let sqrtd = f32::sqrt(discriminant);

        // find nearest root in acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let hit_point = ray.at(root);

        // Modify the HitRecord mutable reference to reflect the values of the current
        // sphere intersection along with the material of the current sphere.
        let mut rec = HitRecord {
            t: root,
            normal: Vec3::default(),
            point: hit_point,
            mat: self.material.clone(),
            front_face: false,
        };
        rec.t = root;
        rec.point = ray.at(rec.t);
        rec.mat = self.material.clone();
        let outward_normal: Vec3 = (rec.point - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);

        Some(rec)
    }
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, material: Materials) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}
