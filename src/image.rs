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
            println!("{}", pixel_color(i,j).format_color());
        }
    }
    eprintln!("\nDone.");
}