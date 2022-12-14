use std::io::{stdout, Write};
use std::path::Path;

use image::ImageError;
use rand::Rng;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use super::camera::Camera;
use super::hit::{Hit, World};
use super::ray::Ray;
use super::size::Size;
use super::vec::Color;

pub struct Render {
    world: World,
    camera: Camera,
    samples_per_pixel: u64,
    max_depth: u64,
    image_size: Size,
}

impl Render {
    pub fn new(
        world: World,
        camera: Camera,
        samples_per_pixel: u64,
        max_depth: u64,
        image_size: Size) -> Render {
        
        Render { world, camera, samples_per_pixel, max_depth, image_size }
    }

    pub fn pixel_color(&self, i: u64, j: u64) -> Color { 
        let mut pixel_color = Color::new(0.0, 0.0, 0.0);
        for _ in 0..self.samples_per_pixel {
            let mut rng = rand::thread_rng();
            let random_u: f64 = rng.gen();
            let random_v: f64 = rng.gen();

            let (u, v) = self.image_size.transform(
                (i as f64) + random_u, 
                (j as f64) + random_v);

            let r = self.camera.get_ray(u, v);
            pixel_color += ray_color(&r, &self.world, self.max_depth);
        }

        (pixel_color / self.samples_per_pixel as f64).sqrt()
    }

    pub fn render_to_image(&self, image_file: &Path) -> Result<(), ImageError> {
        let buffer = render_buffer(
            self.image_size, 
            |i, j| { self.pixel_color(i, j) } );
        
        image::save_buffer(
            image_file, 
            &buffer[..], 
            self.image_size.width() as u32, 
            self.image_size.height() as u32, 
            image::ColorType::Rgb8)
    }

}

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

fn render_buffer<F: Fn(u64, u64) -> Color + Sync + Send>(
    size: Size, 
    pixel_color: F) -> Vec<u8> {
    let buffer_size = (size.width() * size.height() * 3) as usize;
    let mut buffer: Vec<u8> = Vec::with_capacity(buffer_size);
    for j in (0..size.height()).rev() {
        print!("\rScanlines: {:4}", size.height() - j);
        stdout().flush().unwrap();

        let scanline: Vec<Color> = (0..size.width())
            .into_par_iter()
            .map(|i| { pixel_color(i,j) })
            .collect();

        for color in scanline {
            buffer.push(to_u8(color.x()));
            buffer.push(to_u8(color.y()));
            buffer.push(to_u8(color.z()));
        }
    }
    println!("\nDone.");
    buffer
}

fn to_u8(component: f64) -> u8 {
    (256.0 * component.clamp(0.0, 0.999)) as u8
}