use anyhow::Result;

use itertools::Itertools;

use fancy_regex::Regex;
use once_cell::unsync::Lazy;

pub fn get_input() -> Result<Vec<String>> {
    let input = std::fs::read_to_string("../../../crates/2015/5/src/5.txt")?
        .lines()
        .map_into::<String>()
        .collect_vec();
    Ok(input)
}

fn is_vowel(input: char) -> bool {
    matches!(input, 'a' | 'e' | 'i' | 'o' | 'u')
}

fn is_naughty_word(input: &str) -> bool {
    matches!(input, "ab" | "cd" | "pq" | "xy")
}

fn p1_line(input: &str) -> u32 {
    let mut iter = input.chars().peekable();
    let mut vowels = 0;
    let mut duplicates = 0;
    let mut naughty_words = 0;
    while let Some(ch) = iter.next() {
        let next_ch = iter.peek().unwrap_or(&' ');

        vowels += if is_vowel(ch) { 1 } else { 0 };

        duplicates += if ch == *next_ch { 1 } else { 0 };

        let word = format!("{}{}", ch, next_ch);
        naughty_words += if is_naughty_word(&word) { 1 } else { 0 };
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

fn is_containing_pair(input: &str) -> bool {
    let regex = Lazy::new(|| Regex::new(r"([a-z][a-z]).*\1").expect("To be valid"));
    regex.is_match(input).expect("to not fail")
}

fn is_containing_repet(input: &str) -> bool {
    let regex = Lazy::new(|| Regex::new(r"([a-z])[a-z]\1").expect("To be valid"));
    regex.is_match(input).expect("to not fail")
}

fn p2_line(input: &str) -> u32 {
    match is_containing_repet(input) && is_containing_pair(input) {
        true => 1,
        false => 0,
    }
}

pub fn p2(input: &Vec<String>) -> Result<u32> {
    Ok(input.iter().map(|line| p2_line(line)).sum::<u32>())
}

#[cfg(test)]
mod p1 {
    use super::*;

    #[test]
    fn t1() {
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
}

#[cfg(test)]
mod p2 {
    use super::*;

    #[test]
    fn t1() {
        let input = "qjhvhtzxzqqjkmpb";
        assert_eq!(p2_line(input), 1);

        let input = "xyxy";
        assert_eq!(is_containing_pair(input), true);

        let input = "xxyxx";
        assert_eq!(p2_line(input), 1);

        let input = "uurcxstgmygtbstg";
        assert_eq!(p2_line(input), 0);

        let input = "ieodomkazucvgmuy";
        assert_eq!(p2_line(input), 0);

        let input = "aaa";
        assert_eq!(p2_line(input), 0);
    }
}