use clap::Parser;

/// Simple PPM image generator
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Width of the image in pixels
    #[clap(short = 'w', default_value_t = 512)]
    image_width: usize,
    /// Height of the image in pixels
    #[clap(short = 'h', default_value_t = 512)]
    image_height: usize,
}

fn main() {
    let args = Args::parse();
    let image_width = args.image_width;
    let image_height = args.image_height;
    let mut result: Vec<(i32, i32, i32)> = Vec::with_capacity(image_width * image_height);

    match partition(image_height, num_cpus::get()) {
        Ok(partitions) => {
            println!("P3\n{image_width} {image_height}\n255\n");

            for p in partitions {
                let mut r = render_chunk(image_width, image_height, p.0, p.1);
                result.append(&mut r);
            }
        }
        Err(e) => println!("Partitioning failed: {e}"),
    }
    result
        .iter()
        .rev()
        .for_each(|r| println!("{} {} {}", r.0, r.1, r.2));
}

fn partition(image_height: usize, num_threads: usize) -> Result<Vec<(usize, usize)>, String> {
    // Parallelize by horizontal lines of pixels
    // Info needed: index of current line, save results in a vec.
    if num_threads == 0 {
        return Err("Number of threads must not be 0".to_owned());
    }
    if image_height <= num_threads {
        return Ok(vec![(0, image_height)]);
    }

    let chunk_size = image_height / num_threads;
    let remainder = image_height % num_threads;
    let mut result = Vec::with_capacity(num_threads);

    (0..num_threads).for_each(|c| {
        let start = c * chunk_size;
        let end = start + chunk_size - 1;
        result.push((start, end))
    });
    if let Some(last) = result.last_mut() {
        last.1 += remainder;
    }

    Ok(result)
}

fn render_chunk(
    image_width: usize,
    image_height: usize,
    row_start: usize,
    row_end: usize,
) -> Vec<(i32, i32, i32)> {
    assert!(row_start <= row_end);
    let mut result = Vec::with_capacity(row_start.abs_diff(row_end) * image_width);
    let image_width_f = image_width as f64 - 1.0;
    let image_height_f = image_height as f64 - 1.0;

    (row_start..=row_end).for_each(|h| {
        (0..image_width).rev().for_each(|w| {
            let c_f64 = (w as f64 / image_width_f, h as f64 / image_height_f, 0.25);
            let c_i = (
                (255.999 * c_f64.0) as i32,
                (255.999 * c_f64.1) as i32,
                (255.999 * c_f64.2) as i32,
            );
            result.push((c_i.0, c_i.1, c_i.2));
        })
    });
    result
}

#[cfg(test)]
mod tests {
    use crate::partition;

    #[test]
    fn test_partition() {
        let partitions = partition(32, 4);
        let exp_partitions = vec![(0, 7), (8, 15), (16, 23), (24, 31)];
        assert_eq!(partitions, Ok(exp_partitions));

        let partitions = partition(35, 4);
        let exp_partitions = vec![(0, 7), (8, 15), (16, 23), (24, 34)];
        assert_eq!(partitions, Ok(exp_partitions));

        let partitions = partition(3, 4);
        let exp_partitions = vec![(0, 3)];
        assert_eq!(partitions, Ok(exp_partitions));

        let partitions = partition(1, 1);
        let exp_partitions = vec![(0, 1)];
        assert_eq!(partitions, Ok(exp_partitions));

        let partitions = partition(0, 8);
        let exp_partitions = vec![(0, 0)];
        assert_eq!(partitions, Ok(exp_partitions));

        let partitions = partition(64, 0);
        assert_eq!(
            partitions,
            Err("Number of threads must not be 0".to_owned())
        );
    }
}
