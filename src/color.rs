use crate::vec3::Color;
use std::io::Write;

pub fn write_color<W: Write>(stream: &mut W, pixel_color: Color) {
    // Write the translated [0,255] value of each color component.
    writeln!(stream, "{} {} {}",
        (255.999 * pixel_color.x()) as i32, 
        (255.999 * pixel_color.y()) as i32, 
        (255.999 * pixel_color.z()) as i32).expect("write to stream failed");
}