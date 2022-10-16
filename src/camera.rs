use super::vec::{Vec3, Point3};
use super::ray::Ray;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(viewport_width: f64, viewport_height: f64, focal_length: f64) -> Camera {
        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner = origin 
            - horizontal / 2.0 
            - vertical / 2.0 
            - Vec3::new(0.0, 0.0, focal_length);

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let direction = self.lower_left_corner 
            + u * self.horizontal 
            + v * self.vertical 
            - self.origin;
        Ray::new(self.origin, direction)
    }
}
