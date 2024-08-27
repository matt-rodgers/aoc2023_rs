use crate::util::answer::*;
use crate::util::input;
use anyhow::Result;

fn predict_line(line: &str) -> (i64, i64) {
    let mut pt1 = 0;
    let mut first_elements: Vec<i64> = Vec::new();
    let mut nums: Vec<i64> = line
        .split(' ')
        .map(|n| n.parse().expect("couldn't parse number"))
        .collect();

    loop {
        pt1 += nums.last().expect("couldn't get last vector element");
        first_elements.push(*nums.first().expect("couldn't get first element"));

        let next_nums: Vec<i64> = nums
            .iter()
            .enumerate()
            .skip(1)
            .map(|(j, num)| *num - nums[j - 1])
            .collect();

        if next_nums.iter().all(|n| *n == 0) {
            break;
        }

        nums = next_nums;
    }

    let mut new_first: i64 = *first_elements.last().expect("Couldn't get last element");
    for first in first_elements.iter().rev().skip(1) {
        /* first - x = new_first --> x = first - new_first
         * then set: new_first = x, and run next iteration
         */
        let x = first - new_first;
        new_first = x;
    }

    let pt2 = new_first;

    return (pt1, pt2);
}

pub fn run(input_path: &str) -> Result<Answer> {
    let lines = input::get_lines(input_path)?;

    let (pt1, pt2): (i64, i64) = lines
        .iter()
        .map(|line| predict_line(line))
        .fold((0, 0), |accum, element| {
            (accum.0 + element.0, accum.1 + element.1)
        });

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
        let answer = run("inputs/09.ex1").unwrap();
        assert_eq!(answer.pt1, 114);
        assert_eq!(answer.pt2, 2);
    }
}
