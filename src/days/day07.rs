use crate::util::answer::*;
use crate::util::input;
use anyhow::Result;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<char>,
    bid: u64,
    ht: HandType,
}

fn hand_type(cards: &Vec<char>) -> HandType {
    let mut hm: HashMap<char, u32> = HashMap::new();
    let mut wc = 0; // Wildcard count

    for item in cards.iter() {
        if *item == '_' {
            wc += 1;
        } else {
            *hm.entry(*item).or_insert(0) += 1;
        }
    }

    let max_count = *hm.values().max().unwrap_or(&0) + wc;

    let ht = match hm.len() {
        0 => HandType::FiveOfAKind,
        1 => HandType::FiveOfAKind,
        2 => {
            if max_count == 4 {
                HandType::FourOfAKind
            } else {
                HandType::FullHouse
            }
        }
        3 => {
            if max_count == 3 {
                HandType::ThreeOfAKind
            } else {
                HandType::TwoPair
            }
        }
        4 => HandType::OnePair,
        _ => HandType::HighCard,
    };

    ht
}

impl From<(&String, bool)> for Hand {
    fn from(value: (&String, bool)) -> Self {
        let parts: Vec<&str> = value.0.split(' ').collect();
        let cards = parts[0]
            .chars()
            .map(|c| {
                /* Replace J with _ if pt2 flag is true */
                if value.1 && c == 'J' {
                    '_'
                } else {
                    c
                }
            })
            .collect::<Vec<char>>();
        let bid = parts[1].parse().expect("couldn't parse int");
        let ht = hand_type(&cards);
        Hand {
            cards: cards,
            bid: bid,
            ht: ht,
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        /* If all cards are equal, hands are equal */
        if self == other {
            return Ordering::Equal;
        }

        /* Next compare hand types */
        if self.ht > other.ht {
            return Ordering::Greater;
        } else if self.ht < other.ht {
            return Ordering::Less;
        }

        /* If same hand types, compare each card in turn starting with first */
        for i in 0..self.cards.len() {
            let self_card_rank = card_rank(self.cards[i]);
            let other_card_rank = card_rank(other.cards[i]);

            if self_card_rank > other_card_rank {
                return Ordering::Greater;
            } else if self_card_rank < other_card_rank {
                return Ordering::Less;
            }
        }

        return Ordering::Equal;
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn card_rank(card: char) -> u32 {
    match card {
        '_' => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'J' => 10,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => {
            panic!("Unexpected card {}", card)
        }
    }
}

fn run_part(lines: &Vec<String>, pt2: bool) -> u64 {
    let mut hands: Vec<Hand> = lines.iter().map(|line| Hand::from((line, pt2))).collect();
    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) as u64 * h.bid)
        .sum()
}

pub fn run(input_path: &str) -> Result<Answer> {
    let lines = input::get_lines(input_path)?;

    let pt1 = run_part(&lines, false);
    let pt2 = run_part(&lines, true);

    Ok(Answer { pt1: pt1, pt2: pt2 })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1() {
        let answer = run("inputs/07.ex1").unwrap();
        assert_eq!(answer.pt1, 6440);
        assert_eq!(answer.pt2, 5905);
    }

    #[test]
    fn test_ex2() {
        let answer = run("inputs/07.ex2").unwrap();
        assert_eq!(answer.pt1, 6592);
        assert_eq!(answer.pt2, 6839);
    }

    #[test]
    fn test_hands_eq() {
        let h1 = Hand::from((&"AKQJT 123".to_string(), false));
        let h2 = Hand::from((&"AKQJT 123".to_string(), false));
        let h3 = Hand::from((&"AKQJT 456".to_string(), false));

        assert_eq!(h1, h2);
        assert_eq!(h1, h3);
    }

    #[test]
    fn test_hand_rank() {
        let h = Hand::from((&"32T3K 765".to_string(), false));
        assert_eq!(h.ht, HandType::OnePair);

        let h = Hand::from((&"T55J5 684".to_string(), false));
        assert_eq!(h.ht, HandType::ThreeOfAKind);

        let h = Hand::from((&"KK677 28".to_string(), false));
        assert_eq!(h.ht, HandType::TwoPair);

        let h = Hand::from((&"KTJJT 220".to_string(), false));
        assert_eq!(h.ht, HandType::TwoPair);

        let h = Hand::from((&"QQQJA 483".to_string(), false));
        assert_eq!(h.ht, HandType::ThreeOfAKind);

        /* pt2  */
        let h = Hand::from((&"32T3K 765".to_string(), true));
        assert_eq!(h.ht, HandType::OnePair);

        let h = Hand::from((&"T55J5 684".to_string(), true));
        assert_eq!(h.ht, HandType::FourOfAKind);

        let h = Hand::from((&"KK677 28".to_string(), true));
        assert_eq!(h.ht, HandType::TwoPair);

        let h = Hand::from((&"KTJJT 220".to_string(), true));
        assert_eq!(h.ht, HandType::FourOfAKind);

        let h = Hand::from((&"QQQJA 483".to_string(), true));
        assert_eq!(h.ht, HandType::FourOfAKind);
    }

    #[test]
    fn test_hand_compare() {
        let h1 = Hand::from((&"T55J5 684".to_string(), false));
        let h2 = Hand::from((&"QQQJA 483".to_string(), false));
        assert!(h2 > h1);
        assert!(h1 < h2);

        let h1 = Hand::from((&"KK677 28".to_string(), false));
        let h2 = Hand::from((&"KTJJT 220".to_string(), false));
        assert!(h1 > h2);
        assert!(h2 < h1);

        let h1 = Hand::from((&"T55J5 684".to_string(), false));
        let h2 = Hand::from((&"KTJJT 220".to_string(), false));
        assert!(h1 > h2);
        assert!(h2 < h1);
    }
}
