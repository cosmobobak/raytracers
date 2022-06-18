use crate::{vec::{Point4, Vec4}, ray::Ray};

pub struct Camera {
    origin: Point4,
    lower_left_corner: Point4,
    horizontal: Vec4,
    vertical: Vec4,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point4::new(0.0, 0.0, 0.0, 0.0);
        let horizontal = Vec4::new(viewport_width, 0.0, 0.0, 0.0);
        let vertical = Vec4::new(0.0, viewport_height, 0.0, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec4::new(0.0, 0.0, focal_length, 0.0);

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin)
    }
}
