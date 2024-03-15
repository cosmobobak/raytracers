use crate::{vec::Color, Float};
use std::io::Write;

pub fn write<W: Write>(stream: &mut W, pixel_color: Color, samples_per_pixel: u32) {
    #![allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    let mut pixel_color = pixel_color;

    // Divide the color by the number of samples.
    let scale = 1.0 / samples_per_pixel as Float;
    pixel_color *= scale;

    // Gamma-correct for gamma=2.
    pixel_color.sqrt_inplace();

    writeln!(
        stream,
        "{} {} {}",
        (256.0 * pixel_color.x().clamp(0.0, 0.999)) as i32,
        (256.0 * pixel_color.y().clamp(0.0, 0.999)) as i32,
        (256.0 * pixel_color.z().clamp(0.0, 0.999)) as i32
    )
    .expect("write to stream failed");
}
