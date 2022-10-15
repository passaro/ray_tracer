mod camera;
mod hit;
mod ppm;
mod material;
mod ray;
mod render;
mod scene;
mod size;
mod sphere;
mod vec;

use std::path::{Path, PathBuf};

use clap::Parser;

use camera::Camera;
use render::Render;
use size::Size;
use vec::{Point3, Vec3};


#[derive(Parser)]
#[command(author = "Alessandro Passaro", version, about)]
/// Ray Tracing in One Weekend in Rust
struct Arguments {
    
    image_file: PathBuf,

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
    fn image_file(&self) -> &Path { self.image_file.as_ref() }
    fn image_size(&self) -> Size { self.image_size }
    fn samples_per_pixel(&self) -> u64 { self.samples_per_pixel }
    fn max_depth(&self) -> u64 { self.max_depth }
    fn vertical_field_of_view(&self) -> f64 { self.vertical_field_of_view }
}

fn main() {

    let args = Arguments::parse();

    let camera = Camera::new(
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        args.vertical_field_of_view(), 
        args.image_size().aspect_ratio(),
        0.1,
        10.0);

    let render = Render::new(
        scene::random_scene(),
        camera,
        args.samples_per_pixel(),
        args.max_depth(),
        args.image_size());

    render.render_to_image(args.image_file())
        .expect(format!("Error writing to '{}'.", args.image_file().display()).as_str());
}
