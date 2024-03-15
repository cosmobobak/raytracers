#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(dead_code)]

use hittable::Hittable;
use rand::{rngs::ThreadRng, Rng};
use std::io::Write;

use crate::{
    camera::Camera,
    hittable_collection::HittableVec,
    material::{Lambertian, Metal},
    ray::Ray,
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

type Float = f32;

fn ray_color(r: &Ray, world: &impl Hittable, depth: usize, rng: &mut ThreadRng) -> Color {
    const SPHERE_COEFF: Float = 0.5;

    // If we've hit the ray bounce limit, no more light is gathered.
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, Float::INFINITY) {
        if let Some((scattered, attenuation)) = rec.material.scatter(r, &rec, rng) {
            return attenuation * ray_color(&scattered, world, depth - 1, rng);
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
    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    let image_height = ((image_width as Float) / aspect_ratio) as i32;
    let samples_per_pixel = 1000;
    let max_ray_bounces = 50;

    // World
    let mut world = HittableVec::new();

    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.2, 0.2, 0.2));
    let material_left = Lambertian::new(Color::new(0.2, 0.2, 0.2));
    let material_right = Metal::new(Color::new(0.4, 0.4, 0.2), 1.0);

    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, &material_ground)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -2.0), 0.5, &material_center)));
    world.add(Box::new(Sphere::new(Point3::new(-1.5, 0.0, -2.0), 0.5, &material_left)));
    world.add(Box::new(Sphere::new(Point3::new(1.5, 0.0, -2.0), 0.5, &material_right)));
    world.add(Box::new(Sphere::new(Point3::new(-1.0, 1.0, -2.0), 0.5, &material_left)));
    world.add(Box::new(Sphere::new(Point3::new(1.0, 1.0, -2.0), 0.5, &material_right)));
    let world_owned = world;
    let world = &world_owned;

    // Camera
    let camera = Camera::new();

    // Render

    let outfile = std::fs::File::create("out.ppm").unwrap();
    let mut out = std::io::BufWriter::new(outfile);

    writeln!(&mut out, "P3\n{image_width} {image_height}\n255").expect("write to stream failed");

    let camera = &camera;
    std::thread::scope(move |s| {
        let mut threads = Vec::new();
        let mut senders = Vec::new();
        let mut receivers = Vec::new();
        let samples_per_thread = samples_per_pixel as usize / std::thread::available_parallelism().unwrap().get();
        for _ in 0..std::thread::available_parallelism().unwrap().get() {
            let (work_sender, work_receiver) = crossbeam::channel::bounded(0);
            let (result_sender, result_receiver) = crossbeam::channel::bounded(0);
            senders.push(work_sender);
            receivers.push(result_receiver);
            threads.push(s.spawn(move || {
                let mut rng = rand::thread_rng();
                while let Ok((i, j)) = work_receiver.recv() {
                    #[allow(clippy::cast_precision_loss)]
                    let pixel_color = (0..samples_per_thread)
                        .map(|_| {
                            let u = (i as Float + rng.gen::<Float>()) / (image_width - 1) as Float;
                            let v = (j as Float + rng.gen::<Float>()) / (image_height - 1) as Float;
                            let r = camera.get_ray(u, v);
                            ray_color(&r, world, max_ray_bounces, &mut rng)
                        })
                        .sum();
                    result_sender.send((i, j, pixel_color)).unwrap();
                }
            }));
        }

        for j in (0..image_height).rev() {
            print!("\rScanlines remaining: {j:05}");
            for i in 0..image_width {
                for sender in &senders {
                    sender.send((i, j)).unwrap();
                }

                let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                for receiver in &receivers {
                    let (_i, _j, color) = receiver.recv().unwrap();
                    pixel_color += color;
                }

                color::write(&mut out, pixel_color, samples_per_pixel);
            }
        }

        print!("\nDone.\n");

        for sender in senders {
            drop(sender);
        }

        for receiver in receivers {
            drop(receiver);
        }

        for thread in threads {
            thread.join().unwrap();
        }
    });
}
