use crate::util::answer::*;
use crate::util::input;
use anyhow::Result;
use std::collections::HashMap;

pub fn run(input_path: &str) -> Result<Answer> {
    let lines = input::get_lines(input_path)?;

    let pt1: u32 = lines
        .iter()
        .map(|line| {
            let digits: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();
            let first = digits.first().expect("No first digit");
            let last = digits.last().expect("No last digit");
            first.to_digit(10).expect("not a digit") * 10 + last.to_digit(10).expect("not a digit")
        })
        .sum();

    let digit_names = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let pt2: u32 = lines
        .iter()
        .map(|line| {
            let mut first: Option<u32> = None;
            let mut last: Option<u32> = None;
            for (i, c) in line.chars().enumerate() {
                // First check if character is a digit
                if c.is_digit(10) {
                    last = c.to_digit(10);
                    if first == None {
                        first = last;
                    }
                }

                // Then check if it matches a digit name
                for (key, value) in digit_names.iter() {
                    if line[i..].starts_with(key) {
                        last = Some(*value);
                        if first == None {
                            first = last;
                        }
                    }
                }
            }

            // We should now have first and last digits
            10 * first.expect("no first digit") + last.expect("no last digit")
        })
        .sum();

    Ok(Answer {
        pt1: pt1 as u64,
        pt2: pt2 as u64,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1() {
        let answer = run("inputs/01.ex1").unwrap();
        assert_eq!(answer.pt1, 142);
    }

    #[test]
    fn test_ex2() {
        let answer = run("inputs/01.ex2").unwrap();
        assert_eq!(answer.pt2, 281);
    }
}
