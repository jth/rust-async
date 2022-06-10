/*
#include <iostream>

int main() {

    // Image

    const int image_width = 256;
    const int image_height = 256;

    // Render

    std::cout << "P3\n" << image_width << ' ' << image_height << "\n255\n";

    for (int j = image_height-1; j >= 0; --j) {
        for (int i = 0; i < image_width; ++i) {
            auto r = double(i) / (image_width-1);
            auto g = double(j) / (image_height-1);
            auto b = 0.25;

            int ir = static_cast<int>(255.999 * r);
            int ig = static_cast<int>(255.999 * g);
            int ib = static_cast<int>(255.999 * b);

            std::cout << ir << ' ' << ig << ' ' << ib << '\n';
        }
    }
}
*/
use clap::Parser;

/// Simple PPM image generator
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Width of the image in pixels
    #[clap(short = 'w', default_value_t = 512)]
    image_width: u32,
    /// Height of the image in pixels
    #[clap(short = 'h', default_value_t = 512)]
    image_height: u32,
}

fn main() {
    let args = Args::parse();

    // Output PPM
    println!("P3\n{} {}\n255\n", args.image_width, args.image_height);

    for j in (0..args.image_height).rev() {
        for i in 0..args.image_width {
            let c_f64 = (
                i as f64 / (args.image_width as f64 - 1.0),
                j as f64 / (args.image_height as f64 - 1.0),
                0.25,
            );
            let c_i = (
                (255.999 * c_f64.0) as i32,
                (255.999 * c_f64.1) as i32,
                (255.999 * c_f64.2) as i32,
            );
            println!("{} {} {}", c_i.0, c_i.1, c_i.2);
        }
    }
}
