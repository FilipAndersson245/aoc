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
use lazy_regex::regex;

pub fn get_input() -> Result<String> {
    let input = std::fs::read_to_string("../../../crates/§year§/§day§/src/§day§.txt")?;
    Ok(input)
}

pub fn parse_input(input: &str) {
    regex!("");
}

pub fn p1(input: &str) -> Result<i32> {
    Ok(0)
}

pub fn p2(input: &str) -> Result<i32> {
    Ok(0)
}



#[cfg(test)]
mod aoc_§year§_§day§ {
    use super::*;
}
