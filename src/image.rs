use std::io::{stderr, Write};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::vec::Color;

pub fn print_ppm_image<F: Fn(u64, u64) -> Color + Sync + Send>(
    width: u64, 
    height: u64, 
    pixel_color: F) {

    println!("P3");
    println!("{} {}", width, height);
    println!("255");

    for j in (0..height).rev() {
        eprint!("\rScanlines: {:4}", height - j);
        stderr().flush().unwrap();

        let scanline: Vec<Color> = (0..width)
            .into_par_iter()
            .map(|i| { pixel_color(i,j) })
            .collect();

        for color in scanline {
            println!("{}", format_color(color));
        }
    }
    eprintln!("\nDone.");
}

fn format_color(color: Color) -> String {
    format!(
        "{} {} {}",
        (256.0 * color.x().clamp(0.0, 0.999)) as u64,
        (256.0 * color.y().clamp(0.0, 0.999)) as u64,
        (256.0 * color.z().clamp(0.0, 0.999)) as u64
    )
}