use crate::util::answer::*;
use crate::util::input;
use anyhow::Result;
use std::collections::{HashMap, HashSet};
use regex::{Regex, Match};

pub fn run() -> Result<Answer> {
    let not_symbols: HashSet<char> = HashSet::from_iter("0123456789.".chars());

    let lines = input::get_lines("inputs/03.in")?;

    let mut symbol_positions: Vec<(i32, i32)> = Vec::new();
    let mut possible_gears: HashMap<(i32, i32), Vec<u32>> = HashMap::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if !not_symbols.contains(&c) {
                symbol_positions.push((x as i32, y as i32));
            }
            if c == '*' {
                possible_gears.insert((x as i32, y as i32), Vec::new());
            }
        }
    }

    let re = Regex::new(r"\d+")?;
    let pt1: u32 = lines.iter().enumerate().map(|(i, line)| {
        re.find_iter(line).map(|m| {
            let num: u32 = m.as_str().parse().expect("couldn't parse string to integer");
            let mut n: u32 = 0;

            // Part 1: sum all numbers which have an adjacent symbol
            for (x, y) in symbol_positions.iter() {
                if is_adjacent(&m, i as i32, *x, *y) {
                    n = num;
                    break;
                }
            }

            // Part 2: collect all numbers adjacent to each gear
            for ((x, y), ref mut v) in possible_gears.iter_mut() {
                if is_adjacent(&m, i as i32, *x, *y) {
                    v.push(num);
                }
            }

            n
        }).sum::<u32>()
    }).sum();

    // Part 2: sum the 'gear ratio' for any possible gears that have exactly two adjacent numbers
    let pt2: u32 = possible_gears.iter().filter(|(_, val)| {
        val.len() == 2
    }).map(|(_, val)| {
        val[0] * val[1]
    }).sum();

    Ok(Answer {
        pt1: pt1 as u64,
        pt2: pt2 as u64,
    })
}

fn is_adjacent(m: &Match, linenum: i32, x: i32, y: i32) -> bool {
    if linenum >= y - 1 && linenum <= y + 1 {
        if x >= m.start() as i32 - 1 && x <= m.end() as i32 {
            return true;
        }
    }
    return false;
}
