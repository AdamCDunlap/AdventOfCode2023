use core::num;
use std::{cmp::Ordering, collections::HashMap, str::FromStr};

fn card_to_value(card: u8) -> u8 {
    match card {
        b'2'..=b'9' => card - b'0',
        b'T' => 10,
        b'J' => 1, // Part 2 says J is Joker and only worth 1
        b'Q' => 12,
        b'K' => 13,
        b'A' => 14,
        _ => unreachable!(),
    }
}

fn cmp_cards(a: u8, b: u8) -> std::cmp::Ordering {
    card_to_value(a).cmp(&card_to_value(b))
}

struct CamelCards {
    hands: Vec<Hand>,
}

impl FromStr for CamelCards {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(CamelCards {
            hands: s
                .lines()
                .map(Hand::from_str)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    hand: String,
    bid: u32,
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let hand = parts.next().ok_or(())?.to_string();
        assert_eq!(hand.len(), 5);
        let bid = parts.next().ok_or(())?.parse().map_err(|_| ())?;
        if parts.next().is_some() {
            return Err(());
        }
        Ok(Hand { hand, bid })
    }
}

#[derive(PartialEq, Eq, Debug, PartialOrd, Ord)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn find_type(&self) -> Type {
        let mut counts: HashMap<u8, u32> = HashMap::new();
        let mut num_jokers = 0;
        for ch in self.hand.bytes() {
            if ch == b'J' {
                num_jokers += 1;
            } else {
                *counts.entry(ch).or_default() += 1;
            }
        }
        if counts.len() <= 1 {
            return Type::FiveOfAKind;
        }

        let mut best_counts: Vec<u32> = counts.values().cloned().collect();
        // Sort so highest numbers are at the start
        best_counts.sort_by(|a, b| b.cmp(a));
        match (best_counts[0], best_counts[1]) {
            (4, 1) => Type::FourOfAKind,
            (3, 1) => {
                if num_jokers == 1 {
                    Type::FourOfAKind
                } else {
                    Type::ThreeOfAKind
                }
            }
            (3, 2) => Type::FullHouse,
            (2, 2) => {
                if num_jokers == 1 {
                    Type::FullHouse
                } else {
                    Type::TwoPair
                }
            }
            (2, 1) => match num_jokers {
                0 => Type::OnePair,
                1 => Type::ThreeOfAKind,
                2 => Type::FourOfAKind,
                _ => unreachable!(),
            },
            (1, 1) => match num_jokers {
                0 => Type::HighCard,
                1 => Type::OnePair,
                2 => Type::ThreeOfAKind,
                3 => Type::FourOfAKind,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

#[test]
fn test_find_type() {
    assert_eq!(
        Hand {
            hand: "32T3K".to_string(),
            bid: 0
        }
        .find_type(),
        Type::OnePair
    );
    assert_eq!(
        Hand {
            hand: "T55J5".to_string(),
            bid: 0
        }
        .find_type(),
        Type::FourOfAKind
    );
    assert_eq!(
        Hand {
            hand: "KK677".to_string(),
            bid: 0
        }
        .find_type(),
        Type::TwoPair
    );
    assert_eq!(
        Hand {
            hand: "KTJJT".to_string(),
            bid: 0
        }
        .find_type(),
        Type::FourOfAKind
    );
    assert_eq!(
        Hand {
            hand: "QQQJA".to_string(),
            bid: 0
        }
        .find_type(),
        Type::FourOfAKind
    );
    assert_eq!(
        Hand {
            hand: "AAAAJ".to_string(),
            bid: 0
        }
        .find_type(),
        Type::FiveOfAKind
    );
    assert_eq!(
        Hand {
            hand: "33333".to_string(),
            bid: 0
        }
        .find_type(),
        Type::FiveOfAKind
    );
    assert_eq!(
        Hand {
            hand: "12345".to_string(),
            bid: 0
        }
        .find_type(),
        Type::HighCard
    );
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.find_type().cmp(&other.find_type()).then_with(|| {
            for i in 0..self.hand.len() {
                match cmp_cards(self.hand.as_bytes()[i], other.hand.as_bytes()[i]) {
                    Ordering::Equal => (),
                    non_eq => return non_eq,
                }
            }
            Ordering::Equal
        })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[test]
#[ignore = "Only works for part 1"]
fn test_hand_ordering() {
    let mut cards: CamelCards = TEST_INPUT.parse().unwrap();
    cards.hands.sort();
    assert_eq!(cards.hands[0].hand, "32T3K");
    assert_eq!(cards.hands[1].hand, "KTJJT");
    assert_eq!(cards.hands[2].hand, "KK677");
    assert_eq!(cards.hands[3].hand, "T55J5");
    assert_eq!(cards.hands[4].hand, "QQQJA");
}

fn part2(s: &str) -> u32 {
    let mut cards: CamelCards = s.parse().unwrap();
    cards.hands.sort();
    cards
        .hands
        .iter_mut()
        .enumerate()
        .map(|(rank, hand)| (rank as u32 + 1) * hand.bid)
        .sum()
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 5905);
}

fn main() {
    let input = &std::fs::read_to_string("input.txt").expect("input.txt should exist");
    println!("part 2: {}", part2(input));
}

const TEST_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
