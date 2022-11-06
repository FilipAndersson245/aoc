use anyhow::Result;
use std::collections::HashSet;

pub fn get_input() -> Result<String> {
    let input = std::fs::read_to_string("../../../crates/2015/3/src/3.txt")?;
    Ok(input)
}

pub fn p1(input: &str) -> Result<i32> {
    let mut pos = (0, 0);
    let mut seen = HashSet::from([pos]);
    let mut houses = 1;
    for dir in input.chars() {
        let (x, y) = pos;
        pos = match dir {
            '^' => (x + 1, y),
            'v' => (x - 1, y),
            '<' => (x, y + 1),
            '>' => (x, y - 1),
            _ => (x, y),
        };
        if seen.insert(pos) {
            houses += 1;
        }
    }
    Ok(houses)
}

pub fn p2(input: &str) -> Result<i32> {
    let mut poss = [(0, 0), (0, 0)];
    let mut seen = HashSet::from([poss[0]]);
    let mut houses = 1;
    for (idx, dir) in input.char_indices() {
        let idx = if idx % 2 == 0 { 0 } else { 1 };
        let (x, y) = poss[idx];
        poss[idx] = match dir {
            '^' => (x + 1, y),
            'v' => (x - 1, y),
            '<' => (x, y + 1),
            '>' => (x, y - 1),
            _ => unreachable!(),
        };
        if seen.insert(poss[idx]) {
            houses += 1;
        }
    }
    Ok(houses)
}

#[cfg(test)]
mod p1 {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(p1(">").unwrap(), 2);
        assert_eq!(p1(">>").unwrap(), 3);
        assert_eq!(p1(">><").unwrap(), 3);
        assert_eq!(p1("").unwrap(), 1);
        assert_eq!(p1("^").unwrap(), 2);
        assert_eq!(p1("^>v<").unwrap(), 4);
        assert_eq!(p1("^v^v^v^v^v^").unwrap(), 2);
    }
}

#[cfg(test)]
mod p2 {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(p2("^v").unwrap(), 3);
        assert_eq!(p2("^>v<").unwrap(), 3);
        assert_eq!(p2("^v^v^v^v^v").unwrap(), 11);
    }
}
