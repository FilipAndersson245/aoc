use aoc_2015_1::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn benchmark(c: &mut Criterion) {
    let input = get_input();

    let mut bp1 = c.benchmark_group("2015_1_p1");
    bp1.bench_function("v1", |b| b.iter(|| p1(black_box(&input))));
    bp1.finish();

    let mut bp1 = c.benchmark_group("2015_1_p1");
    bp1.bench_function("v1", |b| b.iter(|| p2(black_box(&input))));
    bp1.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
