use crate::util::answer::*;
use crate::util::input;
use anyhow::Result;

#[derive(Default, Debug)]
struct Rgb {
    r: u32,
    g: u32,
    b: u32,
}

pub fn run(input_path: &str) -> Result<Answer> {
    let lines = input::get_lines(input_path)?;

    let games: Vec<Rgb> = lines
        .iter()
        .map(|line| {
            // Discard first part of string
            let ref text = line[line.find(":").expect("no colon in string") + 2..];
            let mut rgb = Rgb::default();
            let sets: Vec<&str> = text.split("; ").collect();
            for set in sets {
                let colours: Vec<&str> = set.split(", ").collect();
                for colour in colours {
                    let parts: Vec<&str> = colour.split(" ").collect();
                    let count: u32 = parts
                        .first()
                        .expect("no first part")
                        .parse()
                        .expect("couldn't parse string to number");
                    match *parts.last().expect("no last part") {
                        "red" => {
                            if count > rgb.r {
                                rgb.r = count
                            }
                        }
                        "green" => {
                            if count > rgb.g {
                                rgb.g = count
                            }
                        }
                        "blue" => {
                            if count > rgb.b {
                                rgb.b = count
                            }
                        }
                        _ => {
                            panic!("string is not red, green or blue")
                        }
                    }
                }
            }
            rgb
        })
        .collect();

    let mut pt1 = 0;
    for (i, g) in games.iter().enumerate() {
        if g.r <= 12 && g.g <= 13 && g.b <= 14 {
            pt1 += 1 + i;
        }
    }

    let pt2: u32 = games.iter().map(|g| g.r * g.g * g.b).sum();

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
        let answer = run("inputs/02.ex1").unwrap();
        assert_eq!(answer.pt1, 8);
        assert_eq!(answer.pt2, 2286);
    }
}
