use crate::util::answer::*;
use crate::util::input;
use anyhow::Result;
use std::collections::HashSet;

pub fn run() -> Result<Answer> {
    let pt2 = 0;

    let lines = input::get_lines("inputs/04.in")?;

    let pt1: u64 = lines.iter().map(|line| {
        let tmp: Vec<&str> = line.split([':', '|']).collect();
        let winning: HashSet<&str> = tmp[1].split(' ').filter(|num| !num.is_empty()).collect();
        let ours: HashSet<&str> = tmp[2].split(' ').filter(|num| !num.is_empty()).collect();
        
        let win_count: u32 = ours.iter()
        .filter(|&num| winning.contains(num))
        .count() as u32;
        
        2u64.pow(win_count - 1)
    }).sum();

    Ok(Answer {
        pt1: pt1,
        pt2: pt2 as u64,
    })
}
