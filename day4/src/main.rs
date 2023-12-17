use std::collections::HashSet;

#[cfg(test)]
const TEST_INPUT: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

#[derive(PartialEq, Eq, Debug)]
enum AocError {
    LineHasNoColon,
    LineHasNoBar,
}

#[derive(Default, PartialEq, Eq, Debug)]
struct Card {
    got: HashSet<u32>,
    winners: HashSet<u32>,
}

fn parse_numbers(num_list: &str) -> HashSet<u32> {
    num_list.split(' ').filter_map(|s| s.parse().ok()).collect()
}

#[test]
fn test_parse_numbers() {
    assert_eq!(parse_numbers("1 2 3"), [1, 2, 3].into());
}

fn parse_card(line: &str) -> Result<Card, AocError> {
    let colon_pos = line.find(':').ok_or(AocError::LineHasNoColon)?;
    let line = &line[colon_pos..];

    let mut bar_iter = line.trim().split('|');

    let c = Card {
        winners: parse_numbers(bar_iter.next().ok_or(AocError::LineHasNoBar)?),
        got: parse_numbers(bar_iter.next().ok_or(AocError::LineHasNoBar)?),
    };
    if bar_iter.next().is_some() {
        return Err(AocError::LineHasNoBar);
    }

    Ok(c)
}

#[test]
fn test_parse_card() {
    assert_eq!(
        parse_card("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"),
        Ok(Card {
            winners: [87, 83, 26, 28, 32].into(),
            got: [88, 30, 70, 12, 93, 22, 82, 36].into(),
        })
    );
}

fn count_winners(c: Card) -> usize {
    c.got.intersection(&c.winners).count()
}

#[test]
fn test_count_winners() {
    assert_eq!(
        count_winners(Card {
            winners: [41, 48, 83, 86, 17].into(),
            got: [83, 86, 6, 31, 17, 9, 48, 53].into(),
        }),
        4
    );
}

fn score_part1(num_winners: usize) -> u32 {
    if num_winners == 0 {
        0
    } else {
        1 << num_winners - 1
    }
}

#[test]
fn test_score() {
    assert_eq!(score_part1(0), 0);
    assert_eq!(score_part1(1), 1);
    assert_eq!(score_part1(4), 8);
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| parse_card(line).ok())
        .map(count_winners)
        .map(score_part1)
        .sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 13);
}

fn part2(input: &str) -> usize {
    let card_winners: Vec<usize> = input
        .lines()
        .filter_map(|line| parse_card(line).ok())
        .map(count_winners)
        .collect();
    // Start with 1 of each card
    let mut card_counts = vec![1; card_winners.len()];

    for (card_num, num_wins) in card_winners.iter().cloned().enumerate() {
        // Look ahead the number of cards that this card won and increment those card counts by the card count of the current card.
        for i in 0..num_wins {
            // The problem specifies that we won't walk off the end of the array, so just leave it to panic if it does
            card_counts[card_num + i + 1] += card_counts[card_num];
        }
    }

    card_counts.iter().sum()
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 30);
}

fn main() {
    let input = &std::fs::read_to_string("input.txt").expect("input.txt should exist");
    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}
