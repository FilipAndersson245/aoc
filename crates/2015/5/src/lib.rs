use anyhow::Result;

use itertools::Itertools;

pub fn get_input() -> Result<Vec<String>> {
    let input = std::fs::read_to_string("../../../crates/2015/5/src/5.txt")?
        .lines()
        .map_into::<String>()
        .collect_vec();
    Ok(input)
}

fn p1_line(input: &str) -> u32 {
    let mut iter = input.chars().peekable();
    let mut vowels = 0;
    let mut duplicates = 0;
    let mut naughty_words = 0;
    while let Some(ch) = iter.next() {
        let next_ch = iter.peek().unwrap_or(&' ');

        vowels += if matches!(ch, 'a' | 'e' | 'i' | 'o' | 'u') {
            1
        } else {
            0
        };

        duplicates += if ch == *next_ch { 1 } else { 0 };

        naughty_words += if matches!(
            format!("{}{}", ch, next_ch).as_str(),
            "ab" | "cd" | "pq" | "xy"
        ) {
            1
        } else {
            0
        };
    }

    if vowels >= 3 && duplicates >= 1 && naughty_words == 0 {
        1
    } else {
        0
    }
}

pub fn p1(input: &Vec<String>) -> Result<u32> {
    Ok(input.iter().map(|line| p1_line(line)).sum::<u32>())
}

pub fn p2(input: &Vec<String>) -> Result<i32> {
    Ok(0)
}

#[cfg(test)]
mod aoc_2015_5 {
    use super::*;

    #[test]
    fn test_p1_oneliner() {
        let input = "ugknbfddgicrmopn";
        assert_eq!(p1_line(input), 1);

        let input = "aaa";
        assert_eq!(p1_line(input), 1);

        let input = "jchzalrnumimnmhp";
        assert_eq!(p1_line(input), 0);

        let input = "haegwjzuvuyypxyu";
        assert_eq!(p1_line(input), 0);

        let input = "dvszwmarrgswjxmb";
        assert_eq!(p1_line(input), 0);
    }

    #[test]
    fn test_p1() {
        let input = get_input().unwrap();
        assert_eq!(p1(&input).unwrap(), 1);
    }
}
