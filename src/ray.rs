use crate::{vec::{Point3, Vec3}, Float};

pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub const fn new(orig: Point3, dir: Vec3) -> Self {
        Self { orig, dir }
    }

    pub const fn origin(&self) -> Point3 {
        self.orig
    }

    pub const fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: Float) -> Point3 {
        self.orig + t * self.dir
    }
}
