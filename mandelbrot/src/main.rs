use num_complex::Complex;
use png::{BitDepth, ColorType};
use std::{fs::File, io::BufWriter, path::Path};

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

fn render(
    pixels: &mut [u8],
    bounds: (u32, u32),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert!(pixels.len() == (bounds.0 * bounds.1) as usize);

    // TODO: Parallelize the renderer
    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels[(row * bounds.0 + column) as usize] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            };
        }
    }
}

fn pixel_to_point(
    bounds: (u32, u32),
    pixel: (u32, u32),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

fn write_image(filename: &str, pixels: &[u8], bounds: (u32, u32)) -> Result<(), std::io::Error> {
    let file = File::create(Path::new(filename))?;
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, bounds.0, bounds.1);
    encoder.set_color(ColorType::Grayscale);
    encoder.set_depth(BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    writer.write_image_data(&pixels)?;

    Ok(())
}

fn main() {
    let output = "/tmp/img.png";
    let img_size = (512u32, 512u32);
    // What role plays the complex plane?
    let upper_left = Complex::new(-1.20, 0.35);
    let lower_right = Complex::new(-1.0, 0.20);

    let mut pixels = vec![0; (img_size.0 * img_size.1) as usize];
    render(&mut pixels, img_size, upper_left, lower_right);

    write_image(output, &pixels, img_size).expect("error writing PNG file");
}
