mod camera;
mod hit;
mod image;
mod ray;
mod sphere;
mod vec;

use camera::Camera;
use hit::{Hit, World};
use rand::Rng;
use ray::Ray;
use sphere::Sphere;
use vec::{Color, Point3, Vec3};

fn ray_color<H: Hit>(r: &Ray, hittable: &H, depth: u64) -> Color {
    if depth <= 0 {
        // If we've exceeded the ray bounce limit, no more light is gathered
        return Color::new(0.0, 0.0, 0.0);
    }
    
    if let Some(rec) = hittable.hit(r, 0.001, f64::INFINITY) {
        // Rejection method:
        // let target = rec.p + rec.normal + Vec3::random_in_unit_sphere();
        
        // True Lambertian Reflection:
        let target = rec.p + rec.normal + Vec3::random_in_unit_sphere().normalized();

        let r = Ray::new(rec.p, target - rec.p);
        0.5 * ray_color(&r, hittable, depth - 1)
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
    const MAX_DEPTH: u64 = 5;

    // World
    let mut world = World::new();
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0), 
        0.5)));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0)));

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
