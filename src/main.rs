mod camera;
mod hit;
mod image;
mod material;
mod ray;
mod size;
mod sphere;
mod vec;

use std::sync::Arc;
use clap::Parser;

use camera::Camera;
use hit::{Hit, World};
use material::{Dielectric, Lambertian, Metal};
use rand::Rng;
use ray::Ray;
use size::Size;
use sphere::Sphere;
use vec::{Color, Point3, Vec3};


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

fn random_scene() -> World {
    let mut rng = rand::thread_rng();
    let mut world = World::new();

    let ground_mat = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let ground_sphere = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_mat);

    world.push(Box::new(ground_sphere));

    for a in -11..=11 {
        for b in -11..=11 {
            let choose_mat: f64 = rng.gen();
            let center = Point3::new((a as f64) + rng.gen_range(0.0..0.9),
                                     0.2,
                                     (b as f64) + rng.gen_range(0.0..0.9));

            if choose_mat < 0.8 {
                // Diffuse
                let albedo = Color::random(0.0..1.0) * Color::random(0.0..1.0);
                let sphere_mat = Arc::new(Lambertian::new(albedo));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.push(Box::new(sphere));
            } else if choose_mat < 0.95 {
                // Metal
                let albedo = Color::random(0.4..1.0);
                let fuzz = rng.gen_range(0.0..0.5);
                let sphere_mat = Arc::new(Metal::new(albedo, fuzz));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.push(Box::new(sphere));
            } else {
                // Glass
                let sphere_mat = Arc::new(Dielectric::new(1.5));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.push(Box::new(sphere));
            }
        }
    }

    let mat1 = Arc::new(Dielectric::new(1.5));
    let mat2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let mat3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    let sphere1 = Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1);
    let sphere2 = Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2);
    let sphere3 = Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3);

    world.push(Box::new(sphere1));
    world.push(Box::new(sphere2));
    world.push(Box::new(sphere3));

    world
}

#[derive(Parser)]
#[command(author = "Alessandro Passaro", version, about)]
/// Ray Tracing in One Weekend in Rust
struct Arguments {
    
    #[arg(short='i', long, default_value_t = Size::new(1200, 800))]
    image_size: Size,

    #[arg(short, long, default_value_t = 500)]
    samples_per_pixel: u64,

    #[arg(short, long, default_value_t = 50)]
    max_depth: u64,

    #[arg(short, long, default_value_t = 20.0)]
    vertical_field_of_view: f64,
}

impl Arguments {
    fn image_size(&self) -> Size { self.image_size }
    fn samples_per_pixel(&self) -> u64 { self.samples_per_pixel }
    fn max_depth(&self) -> u64 { self.max_depth }
    fn vertical_field_of_view(&self) -> f64 { self.vertical_field_of_view }
}

fn main() {

    let args = Arguments::parse();

    // World
    let world = random_scene();

    // Camera
    let camera = Camera::new(
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        args.vertical_field_of_view(), 
        args.image_size().aspect_ratio(),
        0.1,
        10.0);

    image::print_ppm_image(
        args.image_size(),
        |i, j| { 
            
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..args.samples_per_pixel() {
                let mut rng = rand::thread_rng();
                let random_u: f64 = rng.gen();
                let random_v: f64 = rng.gen();

                let (u, v) = args.image_size().transform(
                    (i as f64) + random_u, 
                    (j as f64) + random_v);

                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world, args.max_depth());
            }

            (pixel_color / args.samples_per_pixel() as f64).sqrt()
        } );
}
