use std::fmt::Debug;

use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec::{Color, Vec4}, rtweekend::random_f64,
};

pub trait Material: Debug + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec4)>;
}

#[derive(Debug, Clone)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec4)> {
        let mut scatter_dir = rec.normal + Vec4::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_dir.near_zero() {
            scatter_dir = rec.normal;
        }

        Some((Ray::new(rec.p, scatter_dir), self.albedo))
    }
}


#[derive(Debug, Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec4)> {
        let reflected = Vec4::reflect(r_in.direction().unit_vector(), rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz * Vec4::random_in_unit_sphere());

        if Vec4::dot(scattered.direction(), rec.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Self {
        Self { ref_idx }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec4)> {
        let attenuation = Color::new(1.0, 1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face { 1.0 / self.ref_idx } else { self.ref_idx };

        let unit_dir = r_in.direction().unit_vector();
        let cos_theta = f64::min(Vec4::dot(-unit_dir, rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = sin_theta * refraction_ratio > 1.0;

        let direction = if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > random_f64() {
            Vec4::reflect(unit_dir, rec.normal)
        } else {
            Vec4::refract(unit_dir, rec.normal, refraction_ratio)
        };

        let scattered = Ray::new(rec.p, direction);
        Some((scattered, attenuation))
    }
}