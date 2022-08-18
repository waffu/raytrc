use crate::utility::ray::Ray;
use crate::utility::vec3::{Point3, Vec3};

#[derive(Clone, Debug)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            point: Vec3::default(),
            normal: Vec3::default(),
            t: 0.0,
            front_face: false,
        }
    }
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            point: Vec3::default(),
            normal: Vec3::default(),
            t: 0.0,
            front_face: false,
        }
    }
    pub fn set_face_normal(&mut self, ray: Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(ray.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            outward_normal.negative()
        };
    }
}
