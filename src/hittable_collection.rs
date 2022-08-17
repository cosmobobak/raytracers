use crate::{hittable::{Hittable, HitRecord}, ray::Ray, vec::{Point3, Vec3}};



pub struct HittableVec {
    objects: Vec<Box<dyn Hittable>>
}

impl HittableVec {
    pub fn new() -> HittableVec {
        HittableVec {
            objects: Vec::new()
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableVec {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_anything = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(temp_rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = temp_rec.t;
                hit_anything = Some(temp_rec);
            }
        }

        hit_anything
    }
}