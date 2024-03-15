use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray, Float,
};

pub struct HittableVec<'a> {
    objects: Vec<Box<dyn Hittable + 'a>>,
}

impl<'a> HittableVec<'a> {
    pub fn new() -> Self {
        Self { objects: Vec::new() }
    }

    pub fn add(&mut self, object: Box<dyn Hittable + 'a>) {
        self.objects.push(object);
    }
}

impl<'a> Hittable for HittableVec<'a> {
    fn hit(&self, r: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
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
