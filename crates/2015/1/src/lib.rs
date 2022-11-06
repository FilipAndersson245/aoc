use anyhow::{Ok, Result};
use itertools::Itertools;

pub fn get_input() -> Result<String> {
    let input = std::fs::read_to_string("../../../crates/2015/1/src/1.txt")?;
    Ok(input)
}

pub fn p1(input: &str) -> Result<i32> {
    let count = input.chars().counts();
    let left_para = count.get(&'(').cloned().unwrap_or_default() as i32;
    let right_para = count.get(&')').cloned().unwrap_or_default() as i32;
    Ok(left_para - right_para)
}

pub fn p1_v2(input: &str) -> Result<i32> {
    Ok(input.chars().fold(0, |accum, ch| match ch {
        '(' => accum + 1,
        ')' => accum - 1,
        _ => unreachable!(),
    }))
}

pub fn p2(input: &str) -> Result<usize> {
    let mut state = 0;
    for (idx, ch) in input.char_indices() {
        state += match ch {
            '(' => 1,
            ')' => -1,
            _ => unreachable!(),
        };

        if state == -1 {
            return Ok(idx + 1);
        }
    }
    unreachable!()
}

#[cfg(test)]
mod p1 {
    use super::*;
    fn test_p1_helper(input: &str, expected: i32) {
        let res = p1(input).unwrap();
        assert_eq!(res, expected);
    }

    #[test]
    fn t1() {
        test_p1_helper("(((", 3);
        test_p1_helper("((()", 2);
        test_p1_helper("()()()", 0);
        test_p1_helper("()()())", -1);
    }
}

#[cfg(test)]
mod p2 {
    use super::*;
    fn test_p2_helper(input: &str, expected: usize) {
        let res = p2(input).unwrap();
        assert_eq!(res, expected);
    }

    #[test]
    fn t1() {
        test_p2_helper(")", 1);
        test_p2_helper("()())", 5);
    }
}
