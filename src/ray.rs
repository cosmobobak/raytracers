use crate::vec3::{Point3, Vec3};

struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    fn new(orig: Point3, dir: Vec3) -> Ray {
        Ray {
            orig,
            dir,
        }
    }

    fn origin(&self) -> Point3 {
        self.orig
    }

    fn direction(&self) -> Vec3 {
        self.dir
    }

    fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }

    
}