use aoc_2015_1::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn benchmark(c: &mut Criterion) {
    let input = get_input();

    let mut bp1 = c.benchmark_group("2015_1_p1");
    bp1.significance_level(0.01).sample_size(250);
    bp1.bench_function("v1", |b| b.iter(|| p1(black_box(&input))));
    bp1.bench_function("v2", |b| b.iter(|| p1_v2(black_box(&input))));
    bp1.finish();

    let mut bp2 = c.benchmark_group("2015_1_p2");
    bp2.significance_level(0.01).sample_size(250);
    bp2.bench_function("v1", |b| b.iter(|| p2(black_box(&input))));
    bp2.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
