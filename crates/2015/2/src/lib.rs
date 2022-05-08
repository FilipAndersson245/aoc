use anyhow::Ok;
use anyhow::Result;
use itertools::Itertools;

pub type Row = [u32; 3];
pub type Rows = Vec<Row>;

#[cold]
#[inline(never)]
pub fn get_input() -> Result<String> {
    let input = std::fs::read_to_string("../../../crates/2015/2/src/2.txt")?;
    Ok(input)
}

pub fn parse_input(input: &str) -> Vec<Row> {
    input
        .lines()
        .map(|line| {
            let mut row: Row = line
                .split('x')
                .map(|s| u32::from_str_radix(s, 10).unwrap())
                .collect_vec()
                .try_into()
                .expect("Wrong number of parameters");
            row.sort_unstable();
            row
        })
        .collect_vec()
}

pub fn p1(input: &Rows) -> Result<u32> {
    let res = input.iter().fold(0, |acc, row| {
        let [l, w, h] = row;
        let res = (2 * l * w) + (2 * w * h) + (2 * h * l) + (l * w);
        acc + res
    });
    Ok(res)
}

pub fn p2(input: &Rows) -> Result<u32> {
    let res = input.iter().fold(0, |acc, row| {
        let [l, w, h] = row;
        let res = (l + l + w + w) + (l * w * h);
        acc + res
    });
    Ok(res)
}

#[cfg(test)]
mod aoc_2015_2 {
    use super::*;

    fn p1_helper(input: &str, expection: u32) {
        let data = parse_input(input);
        let res = p1(&data).unwrap();
        assert_eq!(res, expection)
    }

    fn p2_helper(input: &str, expection: u32) {
        let data = parse_input(input);
        let res = p2(&data).unwrap();
        assert_eq!(res, expection)
    }

    #[test]
    fn test_p1() {
        p1_helper("2x3x4", 58);
        p1_helper("1x1x10", 43);
    }

    #[test]
    fn test_p2() {
        p2_helper("2x3x4", 34);
        p2_helper("1x1x10", 14);
    }
}
