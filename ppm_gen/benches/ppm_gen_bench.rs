use criterion::{criterion_group, criterion_main, Criterion};
use ppm_gen::generate_image_data;

pub fn bench_ppm_gen(c: &mut Criterion) {
    let image_width = 4096;
    let image_height = image_width;
    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("ppm_t1", |b| {
        b.to_async(&rt)
            .iter(|| generate_image_data(image_width, image_height, 1));
    });
    c.bench_function("ppm_t4", |b| {
        b.to_async(&rt)
            .iter(|| generate_image_data(image_width, image_height, 4));
    });
    c.bench_function("ppm_t8", |b| {
        b.to_async(&rt)
            .iter(|| generate_image_data(image_width, image_height, 8));
    });
    c.bench_function("ppm_t12", |b| {
        b.to_async(&rt)
            .iter(|| generate_image_data(image_width, image_height, 12));
    });
}

criterion_group!(benches, bench_ppm_gen);
criterion_main!(benches);
