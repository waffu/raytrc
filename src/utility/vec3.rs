use rand::Rng;
use serde::{Serialize, Deserialize};

/// A vector represented as a tuple of three f32 values.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Vec3(f32, f32, f32);

/// A point represented as a tuple of three f32 values.
pub type Point3 = Vec3;

/// An RGB colour represented as a tuple of three f32 values.
pub type Rgb = Vec3;


impl Vec3 {

    /// Create a new Vec3 struct from three f32 values.
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3(x, y, z)
    }

    /// Get the X value of a Vec3.
    pub fn x(&self) -> f32 {
        self.0
    }

    /// Get the Y value of a Vec3.
    pub fn y(&self) -> f32 {
        self.1
    }

    /// Get the Z value of a Vec3.
    pub fn z(&self) -> f32 {
        self.2
    }

    // square root of a vec3
    pub fn len_sqr(&self) -> f32 {
        (self.x() * self.x()) + (self.y() * self.y()) + (self.z() * self.z())
    }

    /// Get the length of a Vec3.
    pub fn len(self) -> f32 {
        f32::sqrt(self.len_sqr())
    }

    /// Get the dot product of a Vec3.
    pub fn dot(self, v: Vec3) -> f32 {
        (v.x() * self.x()) + (v.y() * self.y()) + (v.z() * self.z())
    }

    /// Change the sign of a Vec3.
    pub fn negative(self) -> Self {
        Vec3(self.x() * -1.0, self.y() * -1.0, self.z() * -1.0)
    }

    /// Get the unit vector of a Vec3.
    pub fn unit_vector(self) -> Self {
        self / self.len()
    }

    // a
    pub fn rand() -> Self {
        let mut rng = rand::thread_rng();

        Vec3(
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
        )
    }

    /// Generate a random Vec3 using a custom range.
    pub fn random(min: f32, max: f32) -> Self {
        let mut rng = rand::thread_rng();

        Vec3(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }


    /// Generate a random vector constrained to a unit sphere.
    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random(-1.0, 1.0);
            if p.len_sqr() >= 1.0 {
                continue;
            };
            return p;
        }
    }

    /// Get a random unit length vector.
    pub fn random_unit_vector() -> Vec3 {
        Self::unit_vector(Self::random_in_unit_sphere())
    }

    pub fn near_zero(self) -> bool {
        let s: f32 = 1e-8;

        (self.x().abs() < s) && (self.y().abs() < s) && (self.z().abs() < s)
    }

    pub fn reflect(self, n: Vec3) -> Self {
        self - 2.0 * self.dot(n) * n
    }
}

// add vec3 by vec3
impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vec3(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

// subtract &vec3 from &vec3
impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Vec3(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

// mul vec3 by f32
impl std::ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Vec3(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

// mul f32 by vec3
impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self * rhs.x(), self * rhs.y(), self * rhs.z())
    }
}

// mul vec3 by vec3
impl std::ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Vec3(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

// div vec3 by f32
impl std::ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Vec3(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3(0.0, 0.0, 0.0)
    }
}

impl std::iter::Sum for Vec3 {
    fn sum<I: Iterator<Item = Vec3>>(iter: I) -> Self {
        let mut a = Vec3::default();
        for i in iter {
            a = i + a;
        }
        a
    }
}
