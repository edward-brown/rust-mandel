use num::Complex;
use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let bounds = (1920, 1920);
    let top_left = Complex { re: -2.0, im: 2.0 };
    let bottom_right = Complex { re: 1.0, im: -2.0 };

    let mut pixel_data = vec![0; bounds.0 * bounds.1 * 3];
    render(bounds.0, bounds.1, top_left, bottom_right, &mut pixel_data);
    write_image("output.png", &pixel_data, bounds)
    .expect("Error writing file");

    println!("Elapsed: {:.2?}", now.elapsed());
}
 
fn write_image(filename: &str, pixel_data: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error>{
    let output = File::create(filename)?;

    let encoder = PNGEncoder::new(output);
    encoder.encode(pixel_data, bounds.0 as u32, bounds.1 as u32, ColorType::RGB(8))?;

    Ok(())
}

fn render(width: usize, height: usize, top_left: Complex<f64>, bottom_right: Complex<f64>, pixel_data: &mut [u8]) {
    let mut count = 0;
    for h in 0..height {
        for w in 0..width {
            // Get pixel pos & iteration count
            let c = pixel_to_mandelspace((w, h), (width, height), top_left, bottom_right);
            let ic = iteration_count(c);

            // Calc colors
            let r = (255.0 * c.norm_sqr()) * ic as f64;
            let g = (255.0) * ic as f64;
            let b = (20.0 * c.norm_sqr()) * ic as f64;

            // Set pixel data
            pixel_data[count + 0] = r as u8;
            pixel_data[count + 1] = g as u8;
            pixel_data[count + 2] = b as u8;
            count += 3;
        }
    }
}

fn iteration_count(c: Complex<f64>) -> u8 {
    let mut count = 0;
    let mut z = Complex { re: 0.0, im: 0.0 };

    while z.norm_sqr() < 4.0 && count < 255 {
        z = z * z + c;
        count += 1;
    }

    count
}

fn pixel_to_mandelspace(pixel: (usize, usize), bounds: (usize, usize), top_left: Complex<f64>, bottom_right: Complex<f64>) -> Complex<f64> {
    let x_step = (bottom_right.re - top_left.re) / bounds.0 as f64;
    let y_step = (top_left.im - bottom_right.im) / bounds.1 as f64;

    Complex {
        re: top_left.re + (pixel.0 as f64 * x_step),
        im: bottom_right.im + (pixel.1 as f64 * y_step)
    }
}