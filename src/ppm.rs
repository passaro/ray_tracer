use std::fs::File;
use std::io::{stdout, Write};
use std::path::Path;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::size::Size;
use crate::vec::Color;

pub fn save_ppm_image<F: Fn(u64, u64) -> Color + Sync + Send>(
    image_file: &Path,
    size: Size, 
    pixel_color: F) -> Result<(), std::io::Error> {

    let mut fs = File::create(image_file)?;

    writeln!(&mut fs, "P3")?;
    writeln!(&mut fs, "{} {}", size.width(), size.height())?;
    writeln!(&mut fs, "255")?;

    for j in (0..size.height()).rev() {
        print!("\rScanlines: {:4}", size.height() - j);
        stdout().flush().unwrap();

        let scanline: Vec<Color> = (0..size.width())
            .into_par_iter()
            .map(|i| { pixel_color(i,j) })
            .collect();

        for color in scanline {
            writeln!(&mut fs, "{}", format_color(color))?;
        }
    }
    println!("\nDone.");
    Ok(())
}

fn format_color(color: Color) -> String {
    format!(
        "{} {} {}",
        (256.0 * color.x().clamp(0.0, 0.999)) as u64,
        (256.0 * color.y().clamp(0.0, 0.999)) as u64,
        (256.0 * color.z().clamp(0.0, 0.999)) as u64
    )
}