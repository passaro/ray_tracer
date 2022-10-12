use std::io::{stderr, Write};

mod vec;
use vec::Color;


fn main() {
    const IMAGE_WIDTH: u64 = 1024;
    const IMAGE_HEIGHT: u64 = 1024;

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines: {:4}", IMAGE_HEIGHT - j);
        stderr().flush().unwrap();

        for i in 0..IMAGE_WIDTH {
            let pixel_color = Color::new(
                (i as f64) / ((IMAGE_WIDTH - 1) as f64),
                (j as f64) / ((IMAGE_HEIGHT - 1) as f64),
                0.75);

            println!("{}", pixel_color.format_color());
        }
    }
    eprintln!("\nDone.");
}
