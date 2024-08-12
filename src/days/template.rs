use crate::util::answer::*;
use crate::util::input;
use anyhow::Result;

pub fn run(input_path: &str) -> Result<Answer> {
    let pt1 = 0;
    let pt2 = 0;

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
        let answer = run("inputs/00.ex1").unwrap();
        assert_eq!(answer.pt1, 0);
        assert_eq!(answer.pt2, 0);
    }
}
