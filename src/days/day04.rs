use crate::util::answer::*;
use crate::util::input;
use anyhow::Result;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn run(input_path: &str) -> Result<Answer> {
    let lines = input::get_lines(input_path)?;
    let mut card_count: HashMap<usize, u64> = (0..lines.len()).map(|i| (i, 1)).collect();

    let pt1: u64 = lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            let tmp: Vec<&str> = line.split([':', '|']).collect();
            let winning: HashSet<&str> = tmp[1].split(' ').filter(|num| !num.is_empty()).collect();
            let ours: HashSet<&str> = tmp[2].split(' ').filter(|num| !num.is_empty()).collect();

            let win_count = ours.intersection(&winning).count();

            /* Part 2: count up the won cards */
            for j in 0..win_count {
                let won_card = i + j + 1;
                match card_count.get(&won_card) {
                    Some(count) => {
                        card_count.insert(
                            won_card,
                            count + card_count.get(&i).expect("index not in card_count"),
                        );
                    }
                    None => {
                        panic!("index not in card_count")
                    }
                }
            }

            if 0 == win_count {
                0
            } else {
                2u64.pow(win_count as u32 - 1)
            }
        })
        .sum();

    let pt2: u64 = card_count.iter().map(|(_, v)| v).sum();

    Ok(Answer {
        pt1: pt1,
        pt2: pt2 as u64,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1() {
        let answer = run("inputs/04.ex1").unwrap();
        assert_eq!(answer.pt1, 13);
        assert_eq!(answer.pt2, 30);
    }
}
