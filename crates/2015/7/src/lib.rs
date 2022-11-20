use anyhow::Result;
use helper::AOCError;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Schema<'a> {
    pub shortcut: HashMap<String, u16>,
    pub base: HashMap<&'a str, Vec<&'a str>>,
}

pub fn p1(input: &str) -> Result<impl ToString, AOCError> {
    let key = "a";
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

    Ok(p1_recursive(&mut knowledge, key))
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

pub fn p2(input: &str) -> Result<impl ToString, AOCError> {
    p1(input)
}
