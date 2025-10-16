use criterion::{criterion_group, criterion_main, Criterion};
use olympus::{io::imread, morpho::maxtree, Image2d, C4};
use tos_dt::{add_median_border, distance_transform, immersion};

const IMAGE_FILENAME: &str = "/home/baptou/Pictures/amazigh.pgm";

fn bench_add_border(c: &mut Criterion) {
    let mut img = Image2d::<u8>::default();
    imread(IMAGE_FILENAME, &mut img).unwrap();

    c.bench_function("add_border", |b| b.iter(|| add_median_border(&img)));
    let bordered = add_median_border(&img);

    c.bench_function("immersion", |b| b.iter(|| immersion(&img)));
    let immersed = immersion(&bordered);

    c.bench_function("distance_transform", |b| {
        b.iter(|| distance_transform(&immersed))
    });
    let (dt, _) = distance_transform(&immersed);

    c.bench_function("maxtree", |b| b.iter(|| maxtree(&dt, &C4)));
}

criterion_group!(benches, bench_add_border);
criterion_main!(benches);
