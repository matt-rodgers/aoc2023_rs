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

fn expand(
    galaxies: &Vec<Point>,
    galaxy_rows: &HashSet<isize>,
    galaxy_cols: &HashSet<isize>,
    h: usize,
    w: usize,
    amount: isize,
) -> Vec<Point> {
    let mut row_mapping: HashMap<isize, isize> = HashMap::new();
    let mut col_mapping: HashMap<isize, isize> = HashMap::new();

    let mut extra: isize = 0;
    for i in 0..h {
        let n = i as isize;
        if !galaxy_rows.contains(&n) {
            extra += amount;
        }

        row_mapping.insert(n, n + extra);
    }

    extra = 0;
    for i in 0..w {
        let n = i as isize;
        if !galaxy_cols.contains(&n) {
            extra += amount;
        }

        col_mapping.insert(n, n + extra);
    }

    galaxies
        .iter()
        .map(|galaxy| Point {
            x: *col_mapping.get(&galaxy.x).expect("col mapping not found"),
            y: *row_mapping.get(&galaxy.y).expect("row mapping not found"),
        })
        .collect()
}

fn sum_distances(galaxies: &Vec<Point>) -> isize {
    galaxies
        .iter()
        .combinations(2)
        .map(|pair| pair[0].distance(pair[1]))
        .sum()
}

pub fn run(input_path: &str) -> Result<Answer> {
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

    let galaxies_pt1 = expand(&galaxies, &galaxy_rows, &galaxy_cols, height, width, 1);
    let galaxies_pt2 = expand(&galaxies, &galaxy_rows, &galaxy_cols, height, width, 999999);

    let pt1 = sum_distances(&galaxies_pt1);
    let pt2 = sum_distances(&galaxies_pt2);

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
