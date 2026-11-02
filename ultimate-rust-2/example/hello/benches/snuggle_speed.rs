use criterion::{criterion_group, criterion_main, Criterion};
use hello::snuggle;
use std::hint::black_box;

pub fn snuggle_benchmark(c: &mut Criterion) {
    c.bench_function("snuggle 2", |bencher| {
        bencher.iter(|| snuggle(black_box(2)))
    });
}

criterion_group!(benches, snuggle_benchmark);
criterion_main!(benches);
