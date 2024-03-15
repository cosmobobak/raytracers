use std::{
    fmt::Display,
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign},
};

use rand::{rngs::ThreadRng, Rng};

use crate::Float;

#[derive(Copy, Clone, Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct Vec3 {
    e: [Float; 3],
}

impl Vec3 {
    pub const fn new(x: Float, y: Float, z: Float) -> Self {
        Self { e: [x, y, z] }
    }

    pub const fn x(&self) -> Float {
        self.e[0]
    }

    pub const fn y(&self) -> Float {
        self.e[1]
    }

    pub const fn z(&self) -> Float {
        self.e[2]
    }

    pub fn length_squared(self) -> Float {
        self.e[2].mul_add(self.e[2], self.e[0].mul_add(self.e[0], self.e[1] * self.e[1]))
    }

    pub fn length(&self) -> Float {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(self) -> Self {
        self / self.length()
    }

    pub const fn rgb(self) -> (Float, Float, Float) {
        (self.e[0], self.e[1], self.e[2])
    }

    pub fn random(rng: &mut ThreadRng) -> Self {
        let r1 = rng.gen::<Float>();
        let r2 = rng.gen::<Float>();
        let r3 = rng.gen::<Float>();
        Self::new(r1, r2, r3)
    }

    pub fn random_in_unit_sphere(rng: &mut ThreadRng) -> Self {
        loop {
            let p = Self::random(rng) * 2.0 - Self::new(1.0, 1.0, 1.0);
            if p.length_squared() < 1.0 {
                break p;
            }
        }
    }

    pub fn random_unit_vector(rng: &mut ThreadRng) -> Self {
        Self::random_in_unit_sphere(rng).unit_vector()
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
        const EPSILON: Float = 0.000_000_1;
        self.e[0].abs() < EPSILON && self.e[1].abs() < EPSILON && self.e[2].abs() < EPSILON
    }

    pub fn dot(self, other: Self) -> Float {
        self.e[2].mul_add(other.e[2], self.e[0].mul_add(other.e[0], self.e[1] * other.e[1]))
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

    pub fn refract(self, other: Self, etai_over_etat: Float) -> Self {
        let cos_theta = Float::min(Self::dot(-self, other), 1.0);
        let r_out_perp = etai_over_etat * (self + cos_theta * other);
        let r_out_parallel = -Float::sqrt(Float::abs(1.0 - r_out_perp.length_squared())) * other;
        r_out_perp + r_out_parallel
    }
}

impl Index<usize> for Vec3 {
    type Output = Float;

    fn index(&self, i: usize) -> &Float {
        &self.e[i]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut Float {
        &mut self.e[i]
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self { e: [-self.e[0], -self.e[1], -self.e[2]] }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { e: [self.e[0] + other.e[0], self.e[1] + other.e[1], self.e[2] + other.e[2]] }
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
        Self { e: [self.e[0] - other.e[0], self.e[1] - other.e[1], self.e[2] - other.e[2]] }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Mul<Float> for Vec3 {
    type Output = Self;

    fn mul(self, other: Float) -> Self {
        Self { e: [self.e[0] * other, self.e[1] * other, self.e[2] * other] }
    }
}

impl Mul<Vec3> for Float {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        other * self
    }
}

impl MulAssign<Float> for Vec3 {
    fn mul_assign(&mut self, other: Float) {
        *self = *self * other;
    }
}

impl Mul<Self> for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self { e: [self.e[0] * other.e[0], self.e[1] * other.e[1], self.e[2] * other.e[2]] }
    }
}

impl Div<Float> for Vec3 {
    type Output = Self;

    fn div(self, other: Float) -> Self {
        self * (1.0 / other)
    }
}

impl DivAssign<Float> for Vec3 {
    fn div_assign(&mut self, other: Float) {
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
