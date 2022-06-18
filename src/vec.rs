use std::{ops::{Index, IndexMut, Neg, Add, AddAssign, Mul, MulAssign, Div, DivAssign, SubAssign, Sub}, fmt::Display};

use rand::Rng;

#[derive(Copy, Clone, Debug)]
pub struct Vec4 {
    e: [f64; 4],
}

impl Vec4 {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Vec4 {
        Vec4 { e: [x, y, z, w] }
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

    pub fn w(&self) -> f64 {
        self.e[3]
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2] + self.e[3] * self.e[3]
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
        let r4 = rng.gen::<f64>();
        Self::new(r1, r2, r3, r4)
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random() * 2.0 - Self::new(1.0, 1.0, 1.0, 1.0);
            if p.length_squared() < 1.0 {
                break p;
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit_vector()
    }

    pub fn sqrt(self) -> Self {
        Self::new(self.e[0].sqrt(), self.e[1].sqrt(), self.e[2].sqrt(), self.e[3].sqrt())
    }

    pub fn sqrt_inplace(&mut self) {
        self.e[0] = self.e[0].sqrt();
        self.e[1] = self.e[1].sqrt();
        self.e[2] = self.e[2].sqrt();
        self.e[3] = self.e[3].sqrt();
    }

    pub fn near_zero(&self) -> bool {
        const EPSILON: f64 = 0.0000001;
        self.e[0].abs() < EPSILON 
        && self.e[1].abs() < EPSILON 
        && self.e[2].abs() < EPSILON 
        && self.e[3].abs() < EPSILON
    }

    pub fn dot(v1: Self, v2: Self) -> f64 {
        v1[0] * v2[0] + v1[1] * v2[1] + v1[2] * v2[2] + v1[3] * v2[3]
    }

    // pub fn cross(u: Self, v: Self) -> Self {
    //     Self {
    //         e: [
    //             u[1] * v[2] - u[2] * v[1],
    //             u[2] * v[0] - u[0] * v[2],
    //             u[0] * v[1] - u[1] * v[0],
    //         ],
    //     }
    // }

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

impl Index<usize> for Vec4 {
    type Output = f64;

    fn index(&self, i: usize) -> &f64 {
        &self.e[i]
    }
}

impl IndexMut<usize> for Vec4 {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        &mut self.e[i]
    }
}

impl Neg for Vec4 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            e: [
                -self.e[0],
                -self.e[1],
                -self.e[2],
                -self.e[3],
            ],
        }
    }
}

impl Add for Vec4 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
                self.e[3] + other.e[3],
            ],
        }
    }
}

impl AddAssign for Vec4 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub for Vec4 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2],
                self.e[3] - other.e[3],
            ],
        }
    }
}

impl SubAssign for Vec4 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Mul<f64> for Vec4 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            e: [
                self.e[0] * other,
                self.e[1] * other,
                self.e[2] * other,
                self.e[3] * other,
            ],
        }
    }
}

impl Mul<Vec4> for f64 {
    type Output = Vec4;

    fn mul(self, other: Vec4) -> Vec4 {
        other * self
    }
}

impl MulAssign<f64> for Vec4 {
    fn mul_assign(&mut self, other: f64) {
        *self = *self * other;
    }
}

impl Mul<Self> for Vec4 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            e: [
                self.e[0] * other.e[0],
                self.e[1] * other.e[1],
                self.e[2] * other.e[2],
                self.e[3] * other.e[3],
            ],
        }
    }
}

impl Div<f64> for Vec4 {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        self * (1.0 / other)
    }
}

impl DivAssign<f64> for Vec4 {
    fn div_assign(&mut self, other: f64) {
        *self = *self / other;
    }
}

impl Display for Vec4 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.e[0], self.e[1], self.e[2])
    }
}

pub type Color = Vec4;
pub type Point4 = Vec4;