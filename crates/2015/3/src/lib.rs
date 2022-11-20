use anyhow::Result;
use helper::*;
use std::collections::HashSet;

pub fn p1(input: &str) -> Result<impl ToString, AOCError> {
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

pub fn p2(input: &str) -> Result<impl ToString, AOCError> {
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
