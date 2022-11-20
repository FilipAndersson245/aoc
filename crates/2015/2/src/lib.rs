use anyhow::Result;
use helper::*;
use itertools::Itertools;

pub type Row = [u32; 3];
pub type Rows = Vec<Row>;

fn parse_input(input: &str) -> Vec<Row> {
    input
        .lines()
        .map(|line| {
            let mut row: Row = line
                .split('x')
                .map(|s| s.parse::<u32>().unwrap())
                .collect_vec()
                .try_into()
                .expect("Wrong number of parameters");
            row.sort_unstable();
            row
        })
        .collect_vec()
}

pub fn p1(input: &str) -> Result<impl ToString, AOCError> {
    let input = parse_input(input);
    let res: u32 = input.iter().fold(0, |acc, row| {
        let [l, w, h] = row;
        let res = (2 * l * w) + (2 * w * h) + (2 * h * l) + (l * w);
        acc + res
    });
    Ok(res)
}

pub fn p2(input: &str) -> Result<impl ToString, AOCError> {
    let input = parse_input(input);
    let res = input.iter().fold(0, |acc, row| {
        let [l, w, h] = row;
        let res = (l + l + w + w) + (l * w * h);
        acc + res
    });
    Ok(res)
}
