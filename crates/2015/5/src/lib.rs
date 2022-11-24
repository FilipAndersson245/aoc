use anyhow::Result;

use helper::AOCError;
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

        vowels += i32::from(is_vowel(ch));

        duplicates += i32::from(ch == *next_ch);

        let word = format!("{}{}", ch, next_ch);
        naughty_words += i32::from(is_naughty_word(&word));
    }

    u32::from(vowels >= 3 && duplicates >= 1 && naughty_words == 0)
}

pub fn p1(input: &str) -> Result<impl ToString, AOCError> {
    Ok(input.lines().map(p1_line).sum::<u32>())
}

fn is_containing_pair(input: &str) -> bool {
    let regex = Lazy::new(|| Regex::new(r"([a-z][a-z]).*\1").expect("To be valid"));
    regex.is_match(input).expect("to not fail")
}

fn is_containing_repeat(input: &str) -> bool {
    let regex = Lazy::new(|| Regex::new(r"([a-z])[a-z]\1").expect("To be valid"));
    regex.is_match(input).expect("to not fail")
}

fn p2_line(input: &str) -> u32 {
    match is_containing_repeat(input) && is_containing_pair(input) {
        true => 1,
        false => 0,
    }
}

pub fn p2(input: &str) -> Result<impl ToString, AOCError> {
    Ok(input.lines().map(p2_line).sum::<u32>())
}
