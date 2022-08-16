use super::vec3::*;

#[derive(Clone, Copy)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {

    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { orig: origin, dir: direction }
    }
    pub fn at(self, t: f32) -> Point3 {
        self.orig + (self.dir*t)
    }

    pub fn direction(self) -> Vec3 {
        self.dir
    }

    pub fn origin(self) -> Point3 {
        self.orig
    }
}