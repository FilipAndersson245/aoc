use std::str::FromStr;

use anyhow::Result;
use helper::AOCError;
use itertools::Itertools;

use fancy_regex::Regex;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Commands {
    Toggle,
    TurnOff,
    TurnOn,
}

const BOARD_WIDTH: usize = 1000;
const BOARD_SIZE: usize = BOARD_WIDTH * BOARD_WIDTH;
type BoardP1 = [bool; BOARD_SIZE];
type Point = (usize, usize);

impl FromStr for Commands {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "toggle" => Ok(Commands::Toggle),
            "turn on" => Ok(Commands::TurnOn),
            "turn off" => Ok(Commands::TurnOff),
            _ => Err("Invalid command".to_string()),
        }
    }
}

fn parse(input: &str) -> Vec<(Commands, Point, Point)> {
    let re = Regex::new(r"(toggle|turn off|turn on) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    let caps = re.captures_iter(input);

    caps.map(|cap| {
        let cap = cap.unwrap();
        let command = cap.get(1).unwrap().as_str().parse::<Commands>().unwrap();
        let p1: Point = (
            cap.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            cap.get(3).unwrap().as_str().parse::<usize>().unwrap(),
        );
        let p2: Point = (
            cap.get(4).unwrap().as_str().parse::<usize>().unwrap(),
            cap.get(5).unwrap().as_str().parse::<usize>().unwrap(),
        );
        (command, p1, p2)
    })
    .collect_vec()
}

fn cord_to_index(p: &Point) -> usize {
    p.0 + p.1 * BOARD_WIDTH
}

fn cords_to_index(p1: Point, p2: Point) -> Vec<usize> {
    (p1.0.min(p2.0)..=p1.0.max(p2.0))
        .flat_map(move |x| (p1.1.min(p2.1)..=p1.1.max(p2.1)).map(move |y| cord_to_index(&(x, y))))
        .collect_vec()
}

fn new_light_state_p1(command: Commands, light_state: bool) -> bool {
    match (command, light_state) {
        (Commands::Toggle, true) => false,
        (Commands::Toggle, false) => true,
        (Commands::TurnOff, _) => false,
        (Commands::TurnOn, _) => true,
    }
}

fn lights_on(grid: &BoardP1) -> usize {
    grid.iter().filter(|i| **i).count()
}

pub fn p1(inputs: &str) -> Result<impl ToString, AOCError> {
    let mut grid: BoardP1 = [false; BOARD_SIZE];
    let inputs = parse(inputs);

    inputs.iter().for_each(|(command, p1, p2)| {
        let idxs = cords_to_index(*p1, *p2);
        idxs.iter()
            .for_each(|idx| grid[*idx] = new_light_state_p1(*command, grid[*idx]));
    });
    Ok(lights_on(&grid))
}

pub fn p2(inputs: &str) -> Result<impl ToString, AOCError> {
    let mut grid: Vec<u32> = vec![0; BOARD_SIZE];
    let inputs = parse(inputs);

    inputs.iter().for_each(|(command, p1, p2)| {
        let idxs = cords_to_index(*p1, *p2);
        idxs.iter().for_each(|idx| {
            grid[*idx] = match command {
                Commands::Toggle => grid[*idx].saturating_add(2),
                Commands::TurnOff => grid[*idx].saturating_sub(1),
                Commands::TurnOn => grid[*idx].saturating_add(1),
            }
        });
    });

    let res = grid.iter().sum::<u32>();
    Ok(res)
}
