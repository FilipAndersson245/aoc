use aoc_2015_1::*;
use brunch::{benches, Bench};

benches!(
    Bench::new("2015_1_p1_v1")
        .with_samples(5000)
        .run_seeded(get_input().unwrap(), |v| p1(&v)),
    Bench::new("2015_1_p1_v2")
        .with_samples(5000)
        .run_seeded(get_input().unwrap(), |v| p1_v2(&v)),
    Bench::spacer(),
    Bench::new("2015_1_p2")
        .with_samples(5000)
        .run_seeded(get_input().unwrap(), |v| p2(&v)),
    Bench::spacer(),
);
