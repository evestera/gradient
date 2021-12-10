use std::error::Error;
use hsl::HSL;
use image::{Rgb, RgbImage};

fn main() -> Result<(), impl Error> {
    let width = 3508;
    let height = 3508;
    let mut buffer = RgbImage::new(width, height);
    for x in 0..width {
        for y in 0..height {
            let hsl = HSL {
                h: (((x + y) as f64) / ((width + height) as f64)) * 360.0,
                s: 1.0,
                l: 0.5
            };
            let rgb = hsl.to_rgb();
            buffer.put_pixel(x, y, Rgb([rgb.0, rgb.1, rgb.2]));
        }
    }
    buffer.save("out.png")
}
