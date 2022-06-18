use crate::vec::{Point4, Vec4};

pub struct Ray {
    orig: Point4,
    dir: Vec4,
}

impl Ray {
    pub fn new(orig: Point4, dir: Vec4) -> Ray {
        Ray {
            orig,
            dir,
        }
    }

    pub fn origin(&self) -> Point4 {
        self.orig
    }

    pub fn direction(&self) -> Vec4 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point4 {
        self.orig + t * self.dir
    }
}