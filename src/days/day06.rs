use crate::util::answer::*;
use crate::util::input;
use anyhow::Result;

pub fn run() -> Result<Answer> {
    let pt2 = 0;

    let lines = input::get_lines("inputs/06.in")?;

    let times: Vec<u32> = lines[0]
        .split(' ')
        .skip(1)
        .filter(|item| !item.is_empty())
        .map(|item| item.parse().expect("couldn't parse number"))
        .collect();
    let distances: Vec<u32> = lines[1]
        .split(' ')
        .skip(1)
        .filter(|item| !item.is_empty())
        .map(|item| item.parse().expect("couldn't parse number"))
        .collect();
    let races: Vec<(u32, u32)> = times.into_iter().zip(distances).collect();

    let pt1: usize = races
        .iter()
        .map(|(time, distance)| {
            (1..time - 1)
                .map(|i| i * (time - i))
                .filter(|d| d > distance)
                .count()
        })
        .product();

    Ok(Answer {
        pt1: pt1 as u64,
        pt2: pt2 as u64,
    })
}
