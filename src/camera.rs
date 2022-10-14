use super::vec::{Vec3, Point3};
use super::ray::Ray;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64, 
        aspect_ratio: f64) -> Camera {

        // Vertical field-of-view in degrees
        let theta = std::f64::consts::PI / 180.0 * vfov;
        let viewport_height = 2.0 * (theta / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let cw = (lookfrom - lookat).normalized();
        let cu = vup.cross(cw).normalized();
        let cv = cw.cross(cu);

        let horizontal = viewport_width * cu;
        let vertical = viewport_height * cv;
        let lower_left_corner = lookfrom 
            - horizontal / 2.0 
            - vertical / 2.0 
            - cw;

        Camera {
            origin: lookfrom,
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
