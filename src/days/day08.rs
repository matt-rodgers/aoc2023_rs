use crate::util::answer::*;
use crate::util::input;
use anyhow::Result;
use std::collections::HashMap;
use num::integer::lcm;

pub fn run(input_path: &str) -> Result<Answer> {
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

    if nodes.contains_key(current_node) {
        /* Condition needed for testcase without AAA node for pt2 */
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
    }

    /* Part 2 */
    let start_nodes: Vec<&str> = nodes
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| *k)
        .collect();

    let loop_lengths: Vec<u64> = start_nodes.iter().map(|s| {
            let mut j = 0;
            let mut cur = *s;
            
            while !cur.ends_with('Z') {
                let dir = directions[j % directions.len()];
                
                match nodes.get(cur) {
                    Some(node) => { cur = node[dir] },
                    None       => { panic!("Failed to get node {:?} from hashmap", cur) },
                }
                
                j += 1;
            }
            
            j as u64
    }).collect();

    let mut pt2 = loop_lengths[0];
    for ll in loop_lengths.iter().skip(1) {
        pt2 = lcm(pt2, *ll);
    }

    Ok(Answer {
        pt1: i as u64,
        pt2: pt2,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1() {
        let answer = run("inputs/08.ex1").unwrap();
        assert_eq!(answer.pt1, 2);
    }

    #[test]
    fn test_ex2() {
        let answer = run("inputs/08.ex2").unwrap();
        assert_eq!(answer.pt1, 6);
    }

    #[test]
    fn test_ex3() {
        let answer = run("inputs/08.ex3").unwrap();
        assert_eq!(answer.pt2, 6);
    }
}
