use std::rc::Rc;

use crate::{vec::{Point4, Vec4}, ray::Ray, material::Material};

#[derive(Debug, Clone)]
pub struct HitRecord {
    pub p: Point4,
    pub normal: Vec4,
    pub t: f64,
    pub material: Rc<dyn Material>,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(
        p: Point4,
        t: f64,
        material: Rc<dyn Material>,
        ray: &Ray,
        outward_normal: Vec4,
    ) -> Self {
        let front_face = Vec4::dot(ray.direction(), outward_normal) < 0.0;
        let normal = if front_face { outward_normal } else { -outward_normal };
        Self {
            p,
            normal,
            t,
            material,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}