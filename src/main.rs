use vec3::dot;
use std::io::Write;

use crate::{color::write_color, ray::Ray, vec3::{Color, Point3, Vec3}};

mod color;
mod ray;
mod vec3;

fn hits_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - *center;
    let a = r.direction().length_squared();
    let half_b = dot(&oc, &r.direction());
    let c = oc.length_squared() - radius*radius;
    let discriminant = half_b*half_b - a*c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_color(r: &Ray) -> Color {
    let t = hits_sphere(&Point3::new(0.0,0.0,-1.0), 0.5, r);
    if t > 0.0 {
        let n = (r.at(t) - Point3::new(0.0, 0.0, -1.0)).unit_vector();
        return Color::new(n.x()+1.0, n.y()+1.0, n.z()+1.0) * 0.5;
    }
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 2400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // Camera

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render

    let outfile = std::fs::File::create("out.ppm").unwrap();
    let mut out = std::io::BufWriter::new(outfile);

    writeln!(&mut out, "P3\n{} {}\n255", image_width, image_height).expect("write to stream failed");

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {:05}", j);
        for i in 0..image_width {
            
            let u = i as f64 / (image_width-1) as f64;
            let v = j as f64 / (image_height-1) as f64;
            let r = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical - origin);
            let pixel_color = ray_color(&r);

            write_color(&mut out, pixel_color);
        }
    }

    eprint!("\nDone.\n");
}
