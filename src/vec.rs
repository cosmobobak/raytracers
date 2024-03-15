use std::{
    fmt::Display,
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign},
};

use rand::Rng;

#[derive(Copy, Clone, Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { e: [x, y, z] }
    }

    pub const fn x(&self) -> f64 {
        self.e[0]
    }

    pub const fn y(&self) -> f64 {
        self.e[1]
    }

    pub const fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length_squared(self) -> f64 {
        self.e.into_iter().map(|x| x * x).sum()
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(self) -> Self {
        self / self.length()
    }

    pub const fn rgb(self) -> (f64, f64, f64) {
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
        self.e[0].abs() < EPSILON && self.e[1].abs() < EPSILON && self.e[2].abs() < EPSILON
    }

    pub fn dot(self, other: Self) -> f64 {
        self.e.into_iter().zip(other.e).map(|(a, b)| a * b).sum()
    }

    pub fn cross(self, other: Self) -> Self {
        Self {
            e: [
                self[1].mul_add(other[2], -(self[2] * other[1])),
                self[2].mul_add(other[0], -(self[0] * other[2])),
                self[0].mul_add(other[1], -(self[1] * other[0])),
            ],
        }
    }

    pub fn reflect(self, other: Self) -> Self {
        self - 2.0 * Self::dot(self, other) * other
    }

    pub fn refract(self, other: Self, etai_over_etat: f64) -> Self {
        let cos_theta = f64::min(Self::dot(-self, other), 1.0);
        let r_out_perp = etai_over_etat * (self + cos_theta * other);
        let r_out_parallel = -f64::sqrt(f64::abs(1.0 - r_out_perp.length_squared())) * other;
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
            e: [-self.e[0], -self.e[1], -self.e[2]],
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
            e: [self.e[0] * other, self.e[1] * other, self.e[2] * other],
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

impl Sum for Vec3 {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::new(0.0, 0.0, 0.0), |a, b| a + b)
    }
}

pub type Color = Vec3;
pub type Point3 = Vec3;
