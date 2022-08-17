use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    vec::{Point3, Vec3},
};

pub struct Sphere<'a> {
    center: Point3,
    radius: f64,
    mat_ptr: &'a dyn Material,
}

impl<'a> Sphere<'a> {
    pub fn new(center: Point3, radius: f64, mat_ptr: &'a dyn Material) -> Sphere<'a> {
        Self {
            center,
            radius,
            mat_ptr,
        }
    }
}

impl<'a> Hittable for Sphere<'a> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'a>> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        #[allow(clippy::suspicious_operation_groupings)]
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in
        // an acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let p = r.at(root);
        Some(HitRecord::new(
            p,
            root,
            self.mat_ptr,
            r,
            (p - self.center) / self.radius,
        ))
    }
}
