use anyhow::Result;
use aoc2023::days::*;
use std::time::Instant;

macro_rules! execute_day {
    ($day:ident) => {
        let start = Instant::now();
        let answer = $day::run()?;
        let elapsed = start.elapsed();

        println!(
            "{}: pt1={}, pt2={}, elapsed={:?}",
            stringify!($day),
            answer.pt1,
            answer.pt2,
            elapsed
        );
    };
}

fn main() -> Result<()> {
    execute_day!(day01);
    execute_day!(day02);
    execute_day!(day03);
    execute_day!(day04);
    execute_day!(day05);
    execute_day!(day06);
    Ok(())
}
