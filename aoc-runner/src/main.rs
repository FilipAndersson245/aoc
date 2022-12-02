// use clap::Parser;
use helper::{get_input, AOCError};

use std::env;

fn execute<T, K>(p1: T, p2: K) -> Option<String>
where
    T: Sized + ToString,
    K: Sized + ToString,
{
    let p1 = p1.to_string();
    let p2 = p2.to_string();
    Some(format!("Part 1:\n{}\n\nPart 2:\n{}", p1, p2))
}

fn main() -> Result<(), AOCError> {
    let args: Vec<String> = env::args().collect();
    let year = args[1].parse::<usize>()?;
    let day = args[2].parse::<usize>()?;

    if !(1..=25).contains(&day) {
        return Err(AOCError::IncorrectInput("Day should be between 1-25"));
    }

    if year < 2015 {
        return Err(AOCError::IncorrectInput("Year need to be be after 2014"));
    }

    let input = get_input(year, day)?;

    let result = match year {
        // 2015 => match day {
        //     1 => execute(aoc_2015_1::p1(&input)?, aoc_2015_1::p2(&input)?),
        //     2 => execute(aoc_2015_2::p1(&input)?, aoc_2015_2::p2(&input)?),
        //     3 => execute(aoc_2015_3::p1(&input)?, aoc_2015_3::p2(&input)?),
        //     4 => execute(aoc_2015_4::p1(&input)?, aoc_2015_4::p2(&input)?),
        //     5 => execute(aoc_2015_5::p1(&input)?, aoc_2015_5::p2(&input)?),
        //     6 => execute(aoc_2015_6::p1(&input)?, aoc_2015_6::p2(&input)?),
        //     7 => execute(aoc_2015_7::p1(&input)?, aoc_2015_7::p2(&input)?),
        //     _ => None,
        // },
        2022 => match day {
            1 => execute(aoc_2022_1::p1(&input)?, aoc_2022_1::p2(&input)?),
            2 => execute(aoc_2022_2::p1(&input)?, aoc_2022_2::p2(&input)?),
            // 3 => execute(aoc_2022_3::p1(&input)?, aoc_2022_3::p2(&input)?),
            // 4 => execute(aoc_2022_4::p1(&input)?, aoc_2022_4::p2(&input)?),
            // 5 => execute(aoc_2022_5::p1(&input)?, aoc_2022_5::p2(&input)?),
            // 6 => execute(aoc_2022_6::p1(&input)?, aoc_2022_6::p2(&input)?),
            _ => None,
        },
        _ => None,
    };

    if let Some(res) = result {
        println!("{}", res);
    } else {
        return Err(AOCError::IncorrectInput("Implementation was not found."));
    }

    Ok(())
}
