use anyhow::Result;
use aoc2023::days::*;
use std::time::Instant;

macro_rules! execute_day {
    ($day:ident, $file:literal, $timer:ident) => {
        let start = Instant::now();
        let answer = $day::run(concat!("inputs/", $file))?;
        let elapsed = start.elapsed();
        $timer += elapsed;

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
    let mut total_elapsed = std::time::Duration::from_micros(0);

    execute_day!(day01, "01.in", total_elapsed);
    execute_day!(day02, "02.in", total_elapsed);
    execute_day!(day03, "03.in", total_elapsed);
    execute_day!(day04, "04.in", total_elapsed);
    execute_day!(day05, "05.in", total_elapsed);
    execute_day!(day06, "06.in", total_elapsed);

    println!("Total elapsed time: {:?}", total_elapsed);
    Ok(())
}
