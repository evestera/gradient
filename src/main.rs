use std::error::Error;
use image::{Rgb,RgbImage};

fn to_u8(value: f64) -> u8 {
    let clamped_value = value.clamp(0.0, 1.0);
    (clamped_value * 255.0).round() as u8
}

fn main() -> Result<(), Box<dyn Error>> {
    if std::env::args().any(|arg| arg == "hsluv") {
        hsluv_gradient()?;
    } else {
        hsl_gradient()?;
    };
    Ok(())
}

fn hsl_gradient() -> Result<(), impl Error> {
    let width = 3508;
    let height = 3508;
    let mut buffer = RgbImage::new(width, height);
    for x in 0..width {
        for y in 0..height {
            let hsl = hsl::HSL {
                h: ((((x + y) as f64) / ((width + height) as f64)) * 360.0 * 2.0) % 360.0,
                s: 1.0,
                l: 0.5
            };
            let rgb = hsl.to_rgb();
            buffer.put_pixel(x, y, Rgb([rgb.0, rgb.1, rgb.2]));
        }
    }
    buffer.save("out.png")
}

fn hsluv_gradient() -> Result<(), impl Error> {
    let width = 3508;
    let height = 3508;
    let mut buffer = RgbImage::new(width, height);
    for x in 0..width {
        for y in 0..height {
            let hsl = hsluv::Hsluv::new(
                ((((x + y) as f64) / ((width + height) as f64)) * 360.0 * 2.0) % 360.0,
                70.0,
                70.0
            ).unwrap();
            let rgb = hsluv::Rgb::from(hsl);
            buffer.put_pixel(x, y, image::Rgb([to_u8(rgb.red), to_u8(rgb.green), to_u8(rgb.blue)]));
        }
    }
    buffer.save("out.png")
}