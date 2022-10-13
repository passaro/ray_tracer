use std::io::{stderr, Write};
use crate::vec::Color;

pub fn print_ppm_image<F: FnMut(u64, u64) -> Color>(
    width: u64, 
    height: u64, 
    mut pixel_color: F) {

    println!("P3");
    println!("{} {}", width, height);
    println!("255");

    for j in (0..height).rev() {
        eprint!("\rScanlines: {:4}", height - j);
        stderr().flush().unwrap();

        for i in 0..width {
            println!("{}", format_color(pixel_color(i,j)));
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