use image::png::PNGEncoder;
use image::ColorType;
use num::Complex;
use std::fs::File;
use std::time::Instant;

// Threads
use std::thread;

fn main() {
    let now = Instant::now();

    let num_threads = thread::available_parallelism().expect("Failed to fetch num_threads");
    let bounds = (1000, 1000);
    let top_left = Complex { re: -2.0, im: 2.0 };
    let bottom_right = Complex { re: 1.0, im: -2.0 };

    let mut pixel_data = vec![0; bounds.0 * bounds.1 * 3];
    render_threaded(
        usize::from(num_threads),
        bounds,
        (top_left, bottom_right),
        &mut pixel_data,
    );
    write_image("output.png", &pixel_data, bounds).expect("Error writing file");

    println!("Elapsed: {:.2?}", now.elapsed());
}

fn write_image(
    filename: &str,
    pixel_data: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;

    let encoder = PNGEncoder::new(output);
    encoder.encode(
        pixel_data,
        bounds.0 as u32,
        bounds.1 as u32,
        ColorType::RGB(8),
    )?;

    Ok(())
}

fn render_threaded(
    num_threads: usize,
    bounds: (usize, usize),
    space: (Complex<f64>, Complex<f64>),
    pixel_data: &mut [u8],
) {
    let box_size = (bounds.0 / num_threads, bounds.1 / num_threads);
    let bands: Vec<&mut [u8]> = pixel_data.chunks_mut(box_size.0 * box_size.1 * 3).collect();

    // Debug
    println!("Num_Threads {}", num_threads);
    println!("Num_Bands {}", bands.len());

    crossbeam::scope(|s| {
        for (i, band) in bands.into_iter().enumerate() {
            s.spawn(move |_| {
                let xpos = i % num_threads;
                let ypos = i / num_threads;

                let start_pos = (xpos * box_size.0, ypos * box_size.1);

                render(start_pos, box_size, space, band);
            });
        }
    })
    .expect("Error rendering image");
}

fn render(
    start_pos: (usize, usize),
    bounds: (usize, usize),
    space: (Complex<f64>, Complex<f64>),
    pixel_data: &mut [u8],
) {
    println!("pos: {},{}", start_pos.0, start_pos.1);
    if bounds.0 * bounds.1 > pixel_data.len() {
        println!("Exit");
        return;
    }

    // Pixel Color Mapping
    let mapping: [[u8; 3]; 16] = [
        [66, 30, 15],
        [25, 7, 26],
        [9, 1, 47],
        [4, 4, 73],
        [0, 7, 100],
        [12, 44, 138],
        [24, 82, 177],
        [57, 125, 209],
        [134, 181, 229],
        [211, 236, 248],
        [241, 233, 191],
        [248, 201, 95],
        [255, 170, 0],
        [204, 128, 0],
        [153, 87, 0],
        [0, 0, 0],
    ];

    let mut count = 0;
    for h in 0..(bounds.1 - 1) {
        for w in 0..(bounds.0 - 1) {
            // Get pixel pos & iteration count
            let c =
                pixel_to_mandelspace((start_pos.0 + w, start_pos.1 + h), bounds, space.0, space.1);
            let ic = iteration_count(c);

            let mut r = 0;
            let mut g = 0;
            let mut b = 0;

            if ic != 0 {
                let map = ic % 16;
                r = mapping[map as usize][0];
                g = mapping[map as usize][1];
                b = mapping[map as usize][2];
            }

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

fn pixel_to_mandelspace(
    pixel: (usize, usize),
    bounds: (usize, usize),
    top_left: Complex<f64>,
    bottom_right: Complex<f64>,
) -> Complex<f64> {
    let x_step = (bottom_right.re - top_left.re) / bounds.0 as f64;
    let y_step = (top_left.im - bottom_right.im) / bounds.1 as f64;

    Complex {
        re: top_left.re + (pixel.0 as f64 * x_step),
        im: bottom_right.im + (pixel.1 as f64 * y_step),
    }
}
