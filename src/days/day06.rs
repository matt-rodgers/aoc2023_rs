use crate::util::answer::*;
use crate::util::input;
use anyhow::Result;

pub fn run() -> Result<Answer> {
    let lines = input::get_lines("inputs/06.in")?;

    let times: Vec<u64> = lines[0]
        .split(' ')
        .skip(1)
        .filter(|item| !item.is_empty())
        .map(|item| item.parse().expect("couldn't parse number"))
        .collect();
    let distances: Vec<u64> = lines[1]
        .split(' ')
        .skip(1)
        .filter(|item| !item.is_empty())
        .map(|item| item.parse().expect("couldn't parse number"))
        .collect();
    let races: Vec<(u64, u64)> = times.into_iter().zip(distances).collect();

    let pt1: u64 = races
        .iter()
        .map(|(time, distance)| count_wins(*time, *distance))
        .product();

    /* Part 2 */
    let time: u64 = lines[0]
        .split(' ')
        .skip(1)
        .filter(|item| !item.is_empty())
        .collect::<Vec<&str>>()
        .join("")
        .parse()
        .expect("couldn't parse string");
    let distance: u64 = lines[1]
        .split(' ')
        .skip(1)
        .filter(|item| !item.is_empty())
        .collect::<Vec<&str>>()
        .join("")
        .parse()
        .expect("couldn't parse string");
    println!("Time: {}, Distance: {}", time, distance);

    let pt2 = count_wins(time, distance);

    Ok(Answer {
        pt1: pt1 as u64,
        pt2: pt2 as u64,
    })
}

fn count_wins(time: u64, distance: u64) -> u64 {
    (1..time - 1)
        .map(|i| i * (time - i))
        .filter(|d| *d > distance)
        .count() as u64
}
