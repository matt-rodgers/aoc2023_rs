use crate::util::answer::*;
use crate::util::input;
use anyhow::Result;
use std::collections::HashMap;

pub fn run(input_path: &str) -> Result<Answer> {
    let pt2 = 0;

    let lines = input::get_lines(input_path)?;

    let directions: Vec<usize> = lines[0]
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => panic!("Direction must be 'L' or 'R'"),
        })
        .collect();

    let nodes: HashMap<&str, [&str; 2]> = lines
        .iter()
        .skip(2)
        .map(|line| {
            assert_eq!(line.len(), 16);
            let parts: Vec<&str> = line
                .split([' ', '=', '(', ')', ','])
                .filter(|item| !item.is_empty())
                .collect();
            assert_eq!(parts.len(), 3);
            (parts[0], [parts[1], parts[2]])
        })
        .collect();

    let mut i = 0;
    let mut current_node = "AAA";

    while current_node != "ZZZ" {
        let dir = directions[i % directions.len()];
        i += 1;

        match nodes.get(current_node) {
            Some(node) => current_node = node[dir],
            None => {
                panic!("Failed to get node {:?} from hashmap", current_node)
            }
        }
    }

    Ok(Answer {
        pt1: i as u64,
        pt2: pt2 as u64,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1() {
        let answer = run("inputs/08.ex1").unwrap();
        assert_eq!(answer.pt1, 2);
        assert_eq!(answer.pt2, 0);
    }

    #[test]
    fn test_ex2() {
        let answer = run("inputs/08.ex2").unwrap();
        assert_eq!(answer.pt1, 6);
        assert_eq!(answer.pt2, 0);
    }
}
