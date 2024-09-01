use crate::util::answer::*;
use crate::util::input;
use anyhow::bail;
use anyhow::Result;

// Co-ordinate system: +x --> right, +y --> down
#[derive(Debug, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

// Co-ordinate system: +x --> right, +y --> down
#[derive(Debug, Copy, Clone)]
struct Dir {
    x: isize,
    y: isize,
}

impl std::ops::Add<Dir> for Point {
    type Output = Point;
    fn add(self, rhs: Dir) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn parse_input(input_path: &str) -> Result<Vec<Vec<char>>> {
    let lines = input::get_lines(input_path)?;

    let grid = lines.iter().map(|line| line.chars().collect()).collect();

    Ok(grid)
}

fn find_start(grid: &Vec<Vec<char>>) -> Result<Point> {
    for (i, line) in grid.iter().enumerate() {
        for (j, ch) in line.iter().enumerate() {
            if *ch == 'S' {
                return Ok(Point {
                    x: j as isize,
                    y: i as isize,
                });
            }
        }
    }

    bail!("Could not find start position")
}

fn char_to_direction(ch: char, entry: Dir) -> Option<Dir> {
    match (ch, entry) {
        ('|', Dir { x: 0, y: 1 }) => Some(Dir { x: 0, y: 1 }),
        ('|', Dir { x: 0, y: -1 }) => Some(Dir { x: 0, y: -1 }),
        ('|', _) => None,

        ('-', Dir { x: 1, y: 0 }) => Some(Dir { x: 1, y: 0 }),
        ('-', Dir { x: -1, y: 0 }) => Some(Dir { x: -1, y: 0 }),
        ('-', _) => None,

        ('L', Dir { x: 0, y: 1 }) => Some(Dir { x: 1, y: 0 }),
        ('L', Dir { x: -1, y: 0 }) => Some(Dir { x: 0, y: -1 }),
        ('L', _) => None,

        ('J', Dir { x: 0, y: 1 }) => Some(Dir { x: -1, y: 0 }),
        ('J', Dir { x: 1, y: 0 }) => Some(Dir { x: 0, y: -1 }),
        ('J', _) => None,

        ('7', Dir { x: 0, y: -1 }) => Some(Dir { x: -1, y: 0 }),
        ('7', Dir { x: 1, y: 0 }) => Some(Dir { x: 0, y: 1 }),
        ('7', _) => None,

        ('F', Dir { x: 0, y: -1 }) => Some(Dir { x: 1, y: 0 }),
        ('F', Dir { x: -1, y: 0 }) => Some(Dir { x: 0, y: 1 }),
        ('F', _) => None,

        ('.', _) => None,

        ('S', _) => None,

        _ => None,
    }
}

fn char_from_point(grid: &Vec<Vec<char>>, pt: &Point) -> Option<char> {
    if pt.x < 0 || pt.y < 0 {
        return None;
    }

    let line = grid.get(pt.y as usize)?;
    let ch = line.get(pt.x as usize)?;

    Some(*ch)
}

fn find_loop(grid: &Vec<Vec<char>>, start: &Point, initial_dir: Dir) -> Option<u32> {
    let mut loop_length: u32 = 0;
    let mut current_pos: Point = start.clone();
    let mut current_dir: Dir = initial_dir.clone();

    loop {
        loop_length += 1;

        let next_pos = current_pos + current_dir;
        let ch = char_from_point(grid, &next_pos)?;
        if ch == 'S' {
            return Some(loop_length);
        }

        current_dir = char_to_direction(ch, current_dir)?;
        current_pos = next_pos;
    }
}

pub fn run(input_path: &str) -> Result<Answer> {
    let mut pt1 = 0;
    let pt2 = 0;

    let grid = parse_input(input_path)?;
    let start = find_start(&grid)?;

    let initial_directions = vec![
        Dir { x: 0, y: 1 },
        Dir { x: 0, y: -1 },
        Dir { x: 1, y: 0 },
        Dir { x: -1, y: 0 },
    ];

    for dir in initial_directions {
        let loop_len = find_loop(&grid, &start, dir);
        if let Some(len) = loop_len {
            pt1 = len / 2;
            break;
        }
    }

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
        let answer = run("inputs/10.ex1").unwrap();
        assert_eq!(answer.pt1, 4);
        assert_eq!(answer.pt2, 0);
    }

    #[test]
    fn test_ex2() {
        let answer = run("inputs/10.ex2").unwrap();
        assert_eq!(answer.pt1, 8);
        assert_eq!(answer.pt2, 0);
    }
}
