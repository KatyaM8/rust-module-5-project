use broken_app::{algo, normalize};
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use std::time::Duration;

fn bench_normalize(c: &mut Criterion) {
    let text = "Hello Rust World ".repeat(3_000);
    c.bench_function("normalize_51k", |b| {
        b.iter(|| black_box(normalize(black_box(&text))))
    });
}

fn bench_fib(c: &mut Criterion) {
    c.bench_function("fib_32", |b| {
        b.iter(|| black_box(algo::slow_fib(black_box(32))))
    });
}

fn bench_dedup(c: &mut Criterion) {
    let data: Vec<u64> = (0..1_000).flat_map(|n| [n, n]).rev().collect();
    c.bench_function("dedup_2k", |b| {
        b.iter(|| black_box(algo::slow_dedup(black_box(&data))))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .sample_size(20)
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(3));
    targets = bench_normalize, bench_fib, bench_dedup
}
criterion_main!(benches);
