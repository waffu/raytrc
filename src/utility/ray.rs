use crate::hittables::{hittables::Hittables, hit_record::HitRecord, material::Material, Hittable};

use super::vec3::*;

#[derive(Clone, Copy)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
        }
    }
    pub fn at(self, t: f32) -> Point3 {
        self.orig + (self.dir * t)
    }

    pub fn direction(self) -> Vec3 {
        self.dir
    }

    pub fn origin(self) -> Point3 {
        self.orig
    }

    pub fn cast_ray(self, world: &Vec<Hittables>, depth: i32) -> Rgb {

        // Depth of recursion has been reached, representing a fully absorbed light ray (black shadow).
        if depth <= 0 {
            return Rgb::new(0.0, 0.0, 0.0);
        }
    
        match self.get_obj_closest_intersection(world) {
            Some(rec) => {
                let mut scattered = Ray::new(Point3::default(), Vec3::default());
                let mut attenuation = Rgb::default();
                if rec
                    .mat
                    .scatter(&self, rec.clone(), &mut attenuation, &mut scattered)
                {
                    return attenuation * scattered.cast_ray(world, depth - 1);
                }
                return Rgb::new(0.0, 0.0, 0.0);
            }
            None => {
                let unit_direction: Vec3 = self.direction().unit_vector();
                let t = 0.5 * (unit_direction.y() + 1.0);
                return Rgb::new(1.0, 1.0, 1.0) * (1.0 - t) + Rgb::new(0.5, 0.7, 1.0) * t
            }
        }
    }
    
    // Produce a HitRecord for the closest object intersection for a given ray
    // by iterating through the objects with decrementing t values according to most recent HitRecord.
    pub fn get_obj_closest_intersection(
        self,
        world: &Vec<Hittables>
    ) -> Option<HitRecord> {
    
        let mut temp_rec = None;
        let mut closest_t = f32::MAX;
    
        for object in world.iter() {
            if let Some(rec) = object.hit(self, 0.001, closest_t) {
                closest_t = rec.t;
                temp_rec = Some(rec);
            }
        }
    
        temp_rec
    }
}
