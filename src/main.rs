#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(dead_code)]

use hittable::Hittable;
use rayon::prelude::*;
use std::io::Write;

use crate::{
    camera::Camera,
    hittable_collection::HittableVec,
    material::{Lambertian, Metal},
    ray::Ray,
    rtweekend::random_f64,
    sphere::Sphere,
    vec::{Color, Point3},
};

mod camera;
mod color;
mod hittable;
mod hittable_collection;
mod material;
mod ray;
mod rtweekend;
mod sphere;
mod vec;

fn ray_color(r: &Ray, world: &impl Hittable, depth: usize) -> Color {
    const SPHERE_COEFF: f64 = 0.5;

    // If we've hit the ray bounce limit, no more light is gathered.
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some((scattered, attenuation)) = rec.material.scatter(r, &rec) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }

    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800;
    #[allow(clippy::cast_possible_truncation)]
    let image_height = (f64::from(image_width) / aspect_ratio) as i32;
    let samples_per_pixel = 1000;
    let max_ray_bounces = 50;

    // World
    let mut world = HittableVec::new();

    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.2, 0.2, 0.2));
    let material_left = Lambertian::new(Color::new(0.2, 0.2, 0.2));
    let material_right = Metal::new(Color::new(0.4, 0.4, 0.2), 1.0);

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        &material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -2.0),
        0.5,
        &material_center,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.5, 0.0, -2.0),
        0.5,
        &material_left,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1.5, 0.0, -2.0),
        0.5,
        &material_right,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 1.0, -2.0),
        0.5,
        &material_left,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 1.0, -2.0),
        0.5,
        &material_right,
    )));
    let world_owned = world;
    let world = &world_owned;

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
            let pixel_color = (0..samples_per_pixel)
                .into_par_iter()
                .map(|_| {
                    let u = (f64::from(i) + random_f64()) / f64::from(image_width - 1);
                    let v = (f64::from(j) + random_f64()) / f64::from(image_height - 1);
                    let r = camera.get_ray(u, v);
                    ray_color(&r, world, max_ray_bounces)
                })
                .sum();

            color::write(&mut out, pixel_color, samples_per_pixel);
        }
    }

    print!("\nDone.\n");
}
