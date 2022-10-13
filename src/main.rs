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
    let mut world = World::new();
    let mat_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let mat_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let mat_left = Rc::new(Dielectric::new(1.5));
    let mat_left_inner = Rc::new(Dielectric::new(1.5));
    let mat_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.3));

    world.push(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0), 
        0.5,
        mat_center)));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        mat_ground)));
    world.push(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0), 
        0.5, 
        mat_left)));
    world.push(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0), 
        -0.4, 
        mat_left_inner)));
    world.push(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0), 
        0.5, 
        mat_right)));

    // Camera
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f64 = 1.0;
    
    let camera = Camera::new(VIEWPORT_WIDTH, VIEWPORT_HEIGHT, FOCAL_LENGTH);

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
