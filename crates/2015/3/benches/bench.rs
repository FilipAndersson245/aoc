use aoc_2015_3::*;
use brunch::{benches, Bench};

benches!(
    Bench::new("2015_3_p1")
        .with_samples(5000)
        .run_seeded(get_input().unwrap(), |v| p1(&v)),
    Bench::spacer(),
    Bench::new("2015_3_p2")
        .with_samples(5000)
        .run_seeded(get_input().unwrap(), |v| p2(&v)),
    Bench::spacer(),
);
