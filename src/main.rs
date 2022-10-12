mod image;
mod vec;

use vec::Color;

fn main() {
    const IMAGE_WIDTH: u64 = 1024;
    const IMAGE_HEIGHT: u64 = 1024;

    image::print_ppm_image(
        IMAGE_WIDTH, 
        IMAGE_HEIGHT, 
        |i, j| { Color::new(
            (i as f64) / ((IMAGE_WIDTH - 1) as f64),
            (j as f64) / ((IMAGE_HEIGHT - 1) as f64),
            0.75) } );
}
