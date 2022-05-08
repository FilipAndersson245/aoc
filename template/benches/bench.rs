use aoc_§year§_§day§::*;
use brunch::{benches, Bench};

benches!(
    Bench::new("§year§_§day§_p1")
        .with_samples(5000)
        .run_seeded(get_input().unwrap(), |v| p1(&v)),
    Bench::spacer(),
    Bench::new("§year§_§day§_p2")
        .with_samples(5000)
        .run_seeded(get_input().unwrap(), |v| p2(&v)),
    Bench::spacer(),
);