use clap::Parser;

/// Simple PPM image generator
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Width of the image in pixels
    #[clap(short = 'w', default_value_t = 512)]
    image_width: usize,
    /// Height of the image in pixels
    #[clap(short = 'h', default_value_t = 512)]
    image_height: usize,
    /// Number of threads to use
    #[clap(short = 't', default_value_t = 0)]
    num_threads: usize,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    println!("P3\n{} {}\n255\n", args.image_width, args.image_height);
    let result =
        ppm_gen::generate_image_data(args.image_width, args.image_height, args.num_threads).await;
    result
        .iter()
        .rev()
        .for_each(|r| println!("{} {} {}", r.0, r.1, r.2));
}
