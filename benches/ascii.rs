use ascii::app::convert::convert_to_ascii;
use criterion::{criterion_group, criterion_main, Criterion};

#[allow(dead_code)]
fn pug(c: &mut Criterion) {
    c.bench_function("ascii pug", |b| {
        b.iter(|| convert_to_ascii(image::open("./pug.png").unwrap()))
    });
}

#[allow(dead_code)]
fn heart(c: &mut Criterion) {
    c.bench_function("ascii heart", |b| {
        b.iter(|| convert_to_ascii(image::open("./heart.png").unwrap()))
    });
}

#[allow(dead_code)]
fn coconut(c: &mut Criterion) {
    c.bench_function("ascii coconut", |b| {
        b.iter(|| convert_to_ascii(image::open("./coconut.png").unwrap()))
    });
}

#[allow(dead_code)]
fn butterfly(c: &mut Criterion) {
    c.bench_function("ascii butterfly", |b| {
        b.iter(|| convert_to_ascii(image::open("./butterfly.png").unwrap()))
    });
}

criterion_group!(benches, pug, heart, coconut, butterfly);
criterion_main!(benches);
