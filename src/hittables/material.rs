use crate::utility::{ray::Ray, vec3::*};

use super::hit_record::HitRecord;

#[derive(Clone)]
pub enum Materials {
    Lambertian(Lambertian),
    Metal(Metal),
}

impl Material for Materials {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: HitRecord,
        attenuation: &mut Rgb,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            Materials::Lambertian(mat) => mat.scatter(ray_in, rec, attenuation, scattered),

            Materials::Metal(mat) => mat.scatter(ray_in, rec, attenuation, scattered),
        }
    }
}

pub trait Material {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: HitRecord,
        attenuation: &mut Rgb,
        scattered: &mut Ray,
    ) -> bool;
}

// Lambertian
#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Rgb,
}

impl Lambertian {
    pub fn new(col: Rgb) -> Self {
        Self { albedo: col }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray_in: &Ray,
        rec: HitRecord,
        attenuation: &mut Rgb,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_dir = rec.normal + Vec3::random_unit_vector();

        if scatter_dir.near_zero() {
            scatter_dir = rec.normal;
        }

        *scattered = Ray::new(rec.point, scatter_dir);
        *attenuation = self.albedo;

        true
    }
}
// End Lambertian

// Metal
#[derive(Clone)]
pub struct Metal {
    pub albedo: Rgb,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Rgb, fuzz: f32) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: HitRecord,
        attenuation: &mut Rgb,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(Vec3::unit_vector(ray_in.direction()), rec.normal);
        *scattered = Ray::new(
            rec.point,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
        );
        *attenuation = self.albedo;

        Vec3::dot(scattered.direction(), rec.normal) > 0.0
    }
}
