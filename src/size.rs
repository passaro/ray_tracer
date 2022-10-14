use std::{fmt::Display, str::FromStr};

#[derive(Clone, Copy)]
pub struct Size {
    width: u64,
    height: u64,
}

impl Size {
    pub fn new(width: u64, height: u64) -> Size {
        Size { width, height }
    }

    pub fn aspect_ratio(&self) -> f64 { (self.width as f64) / (self.height as f64) }
    pub fn width(&self) -> u64 { self.width }
    pub fn height(&self) -> u64 { self.height }

    pub fn transform(&self, u: f64, v: f64) -> (f64, f64) {
        (u / ((self.width - 1) as f64), v / ((self.height - 1) as f64))
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}x{}", self.width, self.height))
    }
}

impl FromStr for Size {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        if let [Ok(width), Ok(height)] = s.split("x")
            .map(|a| a.parse::<u64>())
            .collect::<Vec<_>>()[..] {

            Ok(Size { width, height })
        } else {
            Err(format!("Expected format: <WIDTH>x<HEIGHT> (e.g. 800x600)."))
        }
    }
}