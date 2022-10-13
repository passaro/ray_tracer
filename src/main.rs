mod hit;
mod image;
mod ray;
mod sphere;
mod vec;

use hit::Hit;
use ray::Ray;
use vec::{Color, Point3, Vec3};


fn ray_color(r: &Ray) -> Color {
    let sphere = sphere::Sphere::new(
        Point3::new(0.0, 0.0, -1.0), 
        0.5);
    
    if let Some(rec) = sphere.hit(r, 0.0, f64::INFINITY) {
        0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0))
    } else {
        let unit_direction = r.direction().normalized();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) 
        + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 1024;
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin 
                        - horizontal / 2.0 
                        - vertical / 2.0
                        - Vec3::new(0.0, 0.0, focal_length);


    image::print_ppm_image(
        IMAGE_WIDTH, 
        IMAGE_HEIGHT, 
        |i, j| { 
            
            let u = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            let v = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);

            let r = Ray::new(origin, 
                lower_left_corner
                + u * horizontal 
                + v * vertical 
                - origin);
            ray_color(&r) 
        } );
}
