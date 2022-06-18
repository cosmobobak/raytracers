#![allow(dead_code, unused_imports)]

use hittable::{HitRecord, Hittable};
use rayon::prelude::*;
use std::{f64::INFINITY, io::Write, pin::Pin, rc::Rc};

use crate::{
    camera::Camera,
    color::write_color,
    hittable_collection::HittableVec,
    ray::Ray,
    rtweekend::random_f64,
    sphere::Sphere,
    vec::{Color, Point4, Vec4}, material::{Lambertian, Metal, Dielectric, Material},
};

mod camera;
mod color;
mod hittable;
mod hittable_collection;
mod ray;
mod rtweekend;
mod sphere;
mod vec;
mod material;

fn ray_color(r: &Ray, world: &impl Hittable, depth: usize) -> Color {
    const SPHERE_COEFF: f64 = 0.5;

    // If we've hit the ray bounce limit, no more light is gathered.
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, INFINITY) {
        if let Some((scattered, attenuation)) = rec.material.scatter(r, &rec) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        } else {
            return Color::new(0.0, 0.0, 0.0, 0.0);
        }
    }

    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0, 1.0)
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 1000;
    let max_ray_bounces = 50;

    // World
    let mut world = HittableVec::new();

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.2, 0.2, 0.2, 0.0)));
    let material_left   = Rc::new(Lambertian::new(Color::new(0.2, 0.2, 0.2, 0.0)));
    let material_right  = Rc::new(Metal::new(Color::new(0.4, 0.4, 0.2, 0.0), 1.0));

    world.add(Box::new(
        Sphere::new(Point4::new( 0.0, -100.5, -1.0, 0.0), 100.0, material_ground)));
    world.add(Box::new(
        Sphere::new(Point4::new( 0.0,    0.0, -2.0, 0.0),   0.5, material_center)));
    world.add(Box::new(
        Sphere::new(Point4::new(-1.5,    0.0, -2.0, 0.0),   0.5, material_left.clone())));
    world.add(Box::new(
        Sphere::new(Point4::new( 1.5,    0.0, -2.0, 0.0),   0.5, material_right.clone())));
    world.add(Box::new(
        Sphere::new(Point4::new(-1.0,    1.0, -2.0, 0.0),   0.5, material_left)));
    world.add(Box::new(
        Sphere::new(Point4::new( 1.0,    1.0, -2.0, 0.0),   0.5, material_right)));

    // Camera
    let camera = Camera::new();

    // Render

    let outfile = std::fs::File::create("out.ppm").unwrap();
    let mut out = std::io::BufWriter::new(outfile);

    writeln!(&mut out, "P3\n{} {}\n255", image_width, image_height)
        .expect("write to stream failed");

    for j in (0..image_height).rev() {
        print!("\rScanlines remaining: {:05}", j);
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random_f64()) / (image_width - 1) as f64;
                let v = (j as f64 + random_f64()) / (image_height - 1) as f64;
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_ray_bounces);
            }

            write_color(&mut out, pixel_color, samples_per_pixel);
        }
    }

    print!("\nDone.\n");
}
