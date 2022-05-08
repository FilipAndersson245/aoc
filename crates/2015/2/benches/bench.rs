use aoc_2015_2::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn benchmark(c: &mut Criterion) {
    let input = get_input();
    let input = parse_input(&input);

    let mut bp1 = c.benchmark_group("2015_2_p1");
    bp1.significance_level(0.01).sample_size(500);
    bp1.noise_threshold(0.05);
    bp1.bench_function("v1", |b| b.iter(|| p1(black_box(&input))));
    bp1.finish();

    let mut bp2 = c.benchmark_group("2015_2_p2");
    bp2.significance_level(0.01).sample_size(500);
    bp2.noise_threshold(0.05);
    bp2.bench_function("v1", |b| b.iter(|| p2(black_box(&input))));
    bp2.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
