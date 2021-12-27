use crate::{vec3::Color, color::write_color};



mod vec3;
mod color;
mod ray;

fn main() {
    // Image

    const IMAGE_WIDTH: usize = 256;
    const IMAGE_HEIGHT: usize = 256;

    // Render

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            eprint!("\rScanlines remaining: {:03}", j);
            let pixel = Color::new(
                i as f64 / (IMAGE_WIDTH-1) as f64,
                j as f64 / (IMAGE_HEIGHT-1) as f64,
                0.25);

            write_color(&mut std::io::stdout(), pixel);
        }
    }

    eprint!("\nDone.\n");
}
