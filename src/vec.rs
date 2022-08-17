use std::{ops::{Index, IndexMut, Neg, Add, AddAssign, Mul, MulAssign, Div, DivAssign, SubAssign, Sub}, fmt::Display};

use rand::Rng;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(self) -> Self {
        self / self.length()
    }

    pub fn rgb(self) -> (f64, f64, f64) {
        (self.e[0], self.e[1], self.e[2])
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let r1 = rng.gen::<f64>();
        let r2 = rng.gen::<f64>();
        let r3 = rng.gen::<f64>();
        Self::new(r1, r2, r3)
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random() * 2.0 - Self::new(1.0, 1.0, 1.0);
            if p.length_squared() < 1.0 {
                break p;
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit_vector()
    }

    pub fn sqrt(self) -> Self {
        Self::new(self.e[0].sqrt(), self.e[1].sqrt(), self.e[2].sqrt())
    }

    pub fn sqrt_inplace(&mut self) {
        self.e[0] = self.e[0].sqrt();
        self.e[1] = self.e[1].sqrt();
        self.e[2] = self.e[2].sqrt();
    }

    pub fn near_zero(&self) -> bool {
        const EPSILON: f64 = 0.000_000_1;
        self.e[0].abs() < EPSILON 
        && self.e[1].abs() < EPSILON 
        && self.e[2].abs() < EPSILON 
    }

    pub fn dot(v1: Self, v2: Self) -> f64 {
        v1[0] * v2[0] + v1[1] * v2[1] + v1[2] * v2[2]
    }

    pub fn cross(u: Self, v: Self) -> Self {
        Self {
            e: [
                u[1] * v[2] - u[2] * v[1],
                u[2] * v[0] - u[0] * v[2],
                u[0] * v[1] - u[1] * v[0],
            ],
        }
    }

    pub fn reflect(v: Self, n: Self) -> Self {
        v - 2.0 * Self::dot(v, n) * n
    }

    pub fn refract(uv: Self, n: Self, etai_over_etat: f64) -> Self {
        let cos_theta = f64::min(Self::dot(-uv, n), 1.0);
        let r_out_perp =  etai_over_etat * (uv + cos_theta*n);
        let r_out_parallel = -f64::sqrt(f64::abs(1.0 - r_out_perp.length_squared())) * n;
        r_out_perp + r_out_parallel
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, i: usize) -> &f64 {
        &self.e[i]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        &mut self.e[i]
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            e: [
                -self.e[0],
                -self.e[1],
                -self.e[2],
            ],
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ],
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2],
            ],
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            e: [
                self.e[0] * other,
                self.e[1] * other,
                self.e[2] * other,
            ],
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        other * self
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = *self * other;
    }
}

impl Mul<Self> for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            e: [
                self.e[0] * other.e[0],
                self.e[1] * other.e[1],
                self.e[2] * other.e[2],
            ],
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        self * (1.0 / other)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        *self = *self / other;
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.e[0], self.e[1], self.e[2])
    }
}

pub type Color = Vec3;
pub type Point3 = Vec3;