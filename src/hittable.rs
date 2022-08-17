use crate::{
    material::Material,
    ray::Ray,
    vec::{Point3, Vec3},
};

#[derive(Debug, Clone)]
pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub material: &'a dyn Material,
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        p: Point3,
        t: f64,
        material: &'a dyn Material,
        ray: &Ray,
        outward_normal: Vec3,
    ) -> HitRecord<'a> {
        let front_face = Vec3::dot(ray.direction(), outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Self {
            p,
            normal,
            t,
            material,
            front_face,
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
