use anyhow::Result;

use hex::ToHex;
use md5::{Digest, Md5};
use std::fmt;

pub fn get_input() -> Result<String> {
    let input = std::fs::read_to_string("../../../crates/2015/4/src/4.txt")?;
    Ok(input)
}

pub fn p1(input: &str) -> Result<i32> {
    let mut counter = 0;
    let sh = Md5::new();
    loop {
        let mut sh = sh.clone();
        let test_key = fmt::format(format_args!("{}{}", input, counter));
        sh.update(test_key);
        let result = sh.finalize();

        let s = result.encode_hex::<String>();

        if s.starts_with("00000") {
            break;
        }

        counter += 1;
    }
    Ok(counter)
}

pub fn p2(input: &str) -> Result<i32> {
    let mut counter = 0;
    let sh = Md5::new();
    loop {
        let mut sh = sh.clone();
        let test_key = fmt::format(format_args!("{}{}", input, counter));
        sh.update(test_key);
        let result = sh.finalize();

        let s = result.encode_hex::<String>();

        if s.starts_with("000000") {
            break;
        }

        counter += 1;
    }
    Ok(counter)
}

#[cfg(test)]
#[cfg(disabled)]
mod aoc_2015_4 {
    use super::*;

    #[test]
    fn p1_test() {
        let key = "abcdef";
        let a = p1(key).unwrap();
        assert_eq!(a, 609043);

        let key = "pqrstuv";
        let a = p1(key).unwrap();
        assert_eq!(a, 1048970);

        let key = get_input().unwrap();
        let a = p1(&key).unwrap();
        assert_eq!(a, 282749)
    }

    #[test]
    fn p2_test() {
        let key = get_input().unwrap();
        let a = p2(&key).unwrap();
        assert_eq!(a, 9962624)
    }
}
