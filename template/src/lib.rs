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
pub fn get_input() -> &str {
    include_str!("§day§.txt")
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
