use crate::{vec::Color, rtweekend::clamp};
use std::io::Write;

pub fn write_color<W: Write>(stream: &mut W, pixel_color: Color, samples_per_pixel: u32) {
    let mut pixel_color = pixel_color;

    // Divide the color by the number of samples.
    let scale = 1.0 / samples_per_pixel as f64;
    pixel_color *= scale;

    // Gamma-correct for gamma=2.
    pixel_color.sqrt_inplace();

    writeln!(stream, "{} {} {}",
        (256.0 * clamp(pixel_color.x(), 0.0, 0.999)) as i32, 
        (256.0 * clamp(pixel_color.y(), 0.0, 0.999)) as i32, 
        (256.0 * clamp(pixel_color.z(), 0.0, 0.999)) as i32).expect("write to stream failed");
}