use crate::utility::vec3::{Vec3, Point3};
use crate::utility::ray::Ray;

#[derive(Clone, Debug)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord{
            p: Vec3::default(),
            normal: Vec3::default(),
            t: 0.0,
            front_face: false,
        }
    }
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            p: Vec3::default(),
            normal: Vec3::default(),
            t: 0.0,
            front_face: false,
        }
    }
    pub fn set_face_normal(&mut self, ray: Ray, outword_normal: Vec3) {
        self.front_face = Vec3::dot(ray.direction(), outword_normal) < 0.0;
        self.normal = if self.front_face {outword_normal} else {outword_normal.negative()};
    }
}