mod camera;
mod hit;
mod image;
mod material;
mod ray;
mod sphere;
mod vec;

use std::rc::Rc;

use camera::Camera;
use hit::{Hit, World};
use material::{Dielectric, Lambertian, Metal};
use rand::Rng;
use ray::Ray;
use sphere::Sphere;
use vec::{Color, Point3};


fn ray_color<H: Hit>(r: &Ray, hittable: &H, depth: u64) -> Color {
    if depth <= 0 {
        // If we've exceeded the ray bounce limit, no more light is gathered
        return Color::new(0.0, 0.0, 0.0);
    }
    
    if let Some(rec) = hittable.hit(r, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = rec.material.scatter(r, &rec) {
            attenuation * ray_color(&scattered, hittable, depth - 1)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
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
    const SAMPLES_PER_PIXEL: u64 = 100;
    const MAX_DEPTH: u64 = 50;

    // World
    let r: f64 = (std::f64::consts::PI / 4.0).cos(); 
    let mut world = World::new();

    let mat_left = Rc::new(Lambertian::new(Color::new(0.0, 0.0, 1.0)));
    let mat_right = Rc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0)));

    let sphere_left = Sphere::new(Point3::new(-r, 0.0, -1.0), r, mat_left);
    let sphere_right = Sphere::new(Point3::new(r, 0.0, -1.0), r, mat_right);

    world.push(Box::new(sphere_left));
    world.push(Box::new(sphere_right));

    // Camera
    const VERTICAL_FIELD_OF_VIEW: f64 = 90.0;
    const FOCAL_LENGTH: f64 = 1.0;
    let camera = Camera::new(VERTICAL_FIELD_OF_VIEW, ASPECT_RATIO, FOCAL_LENGTH);

    let mut rng = rand::thread_rng();
    image::print_ppm_image(
        IMAGE_WIDTH, 
        IMAGE_HEIGHT, 
        |i, j| { 
            
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let random_u: f64 = rng.gen();
                let random_v: f64 = rng.gen();

                let u = ((i as f64) + random_u) / ((IMAGE_WIDTH - 1) as f64);
                let v = ((j as f64) + random_v) / ((IMAGE_HEIGHT - 1) as f64);

                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }

            (pixel_color / SAMPLES_PER_PIXEL as f64).sqrt()
        } );
}
