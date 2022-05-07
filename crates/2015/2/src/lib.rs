use anyhow::anyhow;
use anyhow::bail;
use anyhow::ensure;
use anyhow::Context;
use anyhow::Ok;
use anyhow::Result;

use num::FromPrimitive;
use num::ToPrimitive;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use itertools::Itertools;
use lazy_regex::regex;

#[cold]
#[inline(never)]
pub fn get_input() -> &'static str {
    include_str!("2.txt")
}

pub fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            let matches = lazy_regex::regex_captures!(r#"(\d+)x(\d+)+(\d+)"#, line).unwrap();
            let a: u32 = matches.1.parse().unwrap();
            let b: u32 = matches.2.parse().unwrap();
            let c: u32 = matches.3.parse().unwrap();
            vec![a, b, c]
        })
        .collect_vec()
}

pub fn p1(input: &Vec<Vec<u32>>) -> Result<u32> {
    let mut res = 0;
    for row in input {
        let l = row[0];
        let w = row[1];
        let h = row[2];
        let min_side = row.iter().min().unwrap().clone();
        res += (2 * l * w) + (2 * w * h) + (2 * h * l) + min_side;
    }
    Ok(res)
}

pub fn p2(input: &str) -> Result<i32> {
    Ok(0)
}

#[cfg(test)]
mod aoc_2015_2 {
    use super::*;

    fn p1_helper(input: &str, expection: u32) {
        let data = parse_input(input);
        let res = p1(&data).unwrap();
        assert_eq!(res, expection)
    }

    #[test]
    fn test_p1() {
        p1_helper("2x3x4", 58);
        p1_helper("1x1x10", 43);
    }
}
