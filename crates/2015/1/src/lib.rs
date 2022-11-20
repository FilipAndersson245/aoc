use anyhow::Result;
use helper::*;

pub fn p1(input: &str) -> Result<impl ToString, AOCError> {
    Ok(input.chars().fold(0, |accum, ch| match ch {
        '(' => accum + 1,
        ')' => accum - 1,
        _ => unreachable!(),
    }))
}

pub fn p2(input: &str) -> Result<impl ToString, AOCError> {
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
