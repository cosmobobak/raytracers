use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    vec::{Point3, Vec3}, Float,
};

pub struct Sphere<'a> {
    center: Point3,
    radius: Float,
    mat_ptr: &'a dyn Material,
}

impl<'a> Sphere<'a> {
    pub fn new(center: Point3, radius: Float, mat_ptr: &'a dyn Material) -> Sphere<'a> {
        Self { center, radius, mat_ptr }
    }
}

impl<'a> Hittable for Sphere<'a> {
    fn hit(&self, r: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord<'a>> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(oc, r.direction());
        let c = Float::mul_add(self.radius, -self.radius, oc.length_squared());

        let discriminant = Float::mul_add(half_b, half_b, -(a * c));
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in an acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if root <= t_min || t_max <= root {
            root = (-half_b + sqrtd) / a;
            if root <= t_min || t_max <= root {
                return None;
            }
        }

        let p = r.at(root);
        let mut rec = HitRecord::new(p, root, self.mat_ptr, r, (p - self.center) / self.radius);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        Some(rec)
    }
}
