use crate::util::answer::*;
use crate::util::input;
use anyhow::Result;
use itertools::Itertools;
use num::abs;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn distance(&self, other: &Point) -> isize {
        abs(self.x - other.x) + abs(self.y - other.y)
    }
}

pub fn run(input_path: &str) -> Result<Answer> {
    let pt2 = 0;

    let lines = input::get_lines(input_path)?;

    let width = lines[0].len();
    let height = lines.len();

    let mut galaxies: Vec<Point> = Vec::new();
    let mut galaxy_rows: HashSet<isize> = HashSet::new();
    let mut galaxy_cols: HashSet<isize> = HashSet::new();

    for (i, line) in lines.iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if ch == '#' {
                galaxy_rows.insert(i as isize);
                galaxy_cols.insert(j as isize);
                galaxies.push(Point {
                    x: j as isize,
                    y: i as isize,
                });
            }
        }
    }

    let mut row_mapping: HashMap<isize, isize> = HashMap::new();
    let mut col_mapping: HashMap<isize, isize> = HashMap::new();

    let mut extra: isize = 0;
    for i in 0..height {
        let n = i as isize;
        if !galaxy_rows.contains(&n) {
            extra += 1;
        }

        row_mapping.insert(n, n + extra);
    }

    extra = 0;
    for i in 0..width {
        let n = i as isize;
        if !galaxy_cols.contains(&n) {
            extra += 1;
        }

        col_mapping.insert(n, n + extra);
    }

    /* Update galaxies with new co-ordinates */
    for galaxy in galaxies.iter_mut() {
        galaxy.x = *col_mapping.get(&galaxy.x).expect("col mapping not found");
        galaxy.y = *row_mapping.get(&galaxy.y).expect("row mapping not found");
    }

    let pt1: isize = galaxies
        .iter()
        .combinations(2)
        .map(|pair| pair[0].distance(pair[1]))
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
        let answer = run("inputs/11.ex1").unwrap();
        assert_eq!(answer.pt1, 374);
    }
}
