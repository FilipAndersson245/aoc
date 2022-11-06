use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Schema<'a> {
    pub shortcut: HashMap<String, u16>,
    pub base: HashMap<&'a str, Vec<&'a str>>,
}

pub fn get_input() -> Result<String> {
    let input = std::fs::read_to_string("../../../crates/2015/7/src/7.txt")?;
    Ok(input)
}

pub fn p1(input: &str, key: &str) -> u16 {
    let mut knowledge = Schema {
        shortcut: HashMap::with_capacity(256),
        base: HashMap::with_capacity(256),
    };
    for s in input.lines() {
        let data = s.trim().split("->").collect_vec();
        let ident = data[1].trim();
        let wire = data[0].trim().split_ascii_whitespace().collect_vec();
        knowledge.base.insert(ident, wire);
    }

    p1_recursive(&mut knowledge, key)
}

fn p1_recursive(knowledge: &mut Schema, key: &str) -> u16 {
    if let Ok(val) = key.parse::<u16>() {
        return val;
    }
    if let Some(val) = knowledge.shortcut.get(&key.to_string()) {
        return *val;
    }

    let shortcut = match knowledge.base[key][..] {
        [first, cmd, second] => {
            let first = p1_recursive(knowledge, first);
            let second = p1_recursive(knowledge, second);
            match cmd {
                "AND" => first & second,
                "OR" => first | second,
                "RSHIFT" => first >> second,
                "LSHIFT" => first << second,
                _ => unreachable!(),
            }
        }
        [_, val_or_ident] => !p1_recursive(knowledge, val_or_ident),
        [val_or_ident] => p1_recursive(knowledge, val_or_ident),
        [..] => unreachable!(),
    };

    knowledge.shortcut.insert(key.to_string(), shortcut);

    shortcut
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let data = "123 -> x
        456 -> y
        x -> p
        x AND y -> d
        x OR y -> e
        x LSHIFT 2 -> f
        y RSHIFT 2 -> g
        NOT x -> h
        NOT y -> i";
        assert_eq!(p1(data, "e"), 507);
        assert_eq!(p1(data, "i"), 65079);
        assert_eq!(p1(data, "y"), 456);
        assert_eq!(p1(data, "p"), 123);
    }
}
