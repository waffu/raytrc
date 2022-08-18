use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub struct Vec3(f32, f32, f32);

pub type Point3 = Vec3;
pub type Rgb = Vec3;

impl Vec3 {
    // create a new vec3 tuple
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3(x, y, z)
    }

    // x of a vec3
    pub fn x(&self) -> f32 {
        self.0
    }

    // y of a vec3
    pub fn y(&self) -> f32 {
        self.1
    }

    // z of a vec3
    pub fn z(&self) -> f32 {
        self.2
    }

    // square root of a vec3
    pub fn len_sqr(&self) -> f32 {
        (self.x() * self.x()) + (self.y() * self.y()) + (self.z() * self.z())
    }

    // length of a vec3
    pub fn len(self) -> f32 {
        f32::sqrt(self.len_sqr())
    }

    // dot product of a vec3
    pub fn dot(self, v: Vec3) -> f32 {
        (v.x() * self.x()) + (v.y() * self.y()) + (v.z() * self.z())
    }

    // change sign of vector
    pub fn negative(self) -> Self {
        Vec3(self.x() * -1.0, self.y() * -1.0, self.z() * -1.0)
    }

    pub fn unit_vector(self) -> Self {
        self.clone() / self.len()
    }

    pub fn rand() -> Self {
        let mut rng = rand::thread_rng();

        Vec3(
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
        )
    }

    pub fn random(min: f32, max: f32) -> Self {
        let mut rng = rand::thread_rng();

        Vec3(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random(-1.0, 1.0);
            if p.len_sqr() >= 1.0 {
                continue;
            };
            return p;
        }
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
