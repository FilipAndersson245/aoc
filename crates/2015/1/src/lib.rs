use anyhow::anyhow;
use anyhow::bail;
use anyhow::ensure;
use anyhow::Context;
use anyhow::Result;

use num::FromPrimitive;
use num::ToPrimitive;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use itertools::Itertools;
use lazy_regex;

#[cold]
#[inline(never)]
pub fn get_input() -> Result<&str> {
    include_str!("1.txt")
}

pub fn parse_input(input: &str) {
    regex!("");
    lazy_regex::regex_captures!()
}

pub fn p1() -> Result<i32> {
    Ok(0)
}

pub fn p2() -> Result<i32> {
    Ok(0)
}

#[cfg(test)]
mod aoc_2015_1 {
    use super::*;
}
