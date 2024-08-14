use crate::util::answer::*;
use crate::util::input;
use anyhow::Result;

pub fn run(input_path: &str) -> Result<Answer> {
    let lines = input::get_lines(input_path)?;

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

    let pt2 = count_wins(time, distance);

    Ok(Answer {
        pt1: pt1 as u64,
        pt2: pt2 as u64,
    })
}

fn count_wins(time: u64, distance: u64) -> u64 {
    /* d = i * (time - i)
     * d = i * time - i^2
     * i^2 - time.i + d = 0
     *
     * Use quadratic formula: ax^2 + bx + c ==> x = (-b +/-sqrt(b^2 - 4ac)) / 2a
     * where a = 1, b = -time, c = distance
     * we need an extra step where we find the closest integer to each root such that the distance is greater
     */

    let a: f64 = 1.0;
    let b: f64 = -(time as f64);
    let c: f64 = distance as f64;

    let i1: f64 = (-b + (b.powi(2) - 4.0 * a * c).sqrt()) / (2.0 * a);
    let i2: f64 = (-b - (b.powi(2) - 4.0 * a * c).sqrt()) / (2.0 * a);

    let (larger, smaller) = if i1 > i2 { (i1, i2) } else { (i2, i1) };

    let smaller_int: u64 = if smaller.ceil() == smaller {
        smaller.ceil() as u64 + 1
    } else {
        smaller.ceil() as u64
    };

    let larger_int: u64 = if larger.ceil() == larger {
        larger.floor() as u64 - 1
    } else {
        larger.floor() as u64
    };

    /* Inclusive range between the two values */
    larger_int - smaller_int + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1() {
        let answer = run("inputs/06.ex1").unwrap();
        assert_eq!(answer.pt1, 288);
        assert_eq!(answer.pt2, 71503);
    }
}
