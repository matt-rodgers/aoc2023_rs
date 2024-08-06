use anyhow::Result;
use aoc2023::days::*;
use std::time::Instant;

fn main() -> Result<()> {
    let start = Instant::now();
    let answer = day01::run()?;
    let elapsed = start.elapsed();

    println!("Part 1: {}", answer.pt1);
    println!("Part 2: {}", answer.pt2);
    println!("Elapsed time: {:?}", elapsed);
    Ok(())
}
