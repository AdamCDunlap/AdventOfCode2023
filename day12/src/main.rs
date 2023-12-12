use rayon::prelude::*;
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
enum AocError {
    InvalidLine,
    InvalidSpringType,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Spring {
    Broken,
    Operational,
}

#[derive(Debug, PartialEq, Eq)]
struct Record {
    springs: Vec<Option<Spring>>,
    group_lens: Vec<usize>,
}

impl FromStr for Record {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');

        let springs: Vec<Option<Spring>> = parts
            .next()
            .ok_or(AocError::InvalidLine)?
            .chars()
            .map(|ch| {
                Ok(match ch {
                    '#' => Some(Spring::Broken),
                    '.' => Some(Spring::Operational),
                    '?' => None,
                    _ => return Err(AocError::InvalidSpringType),
                })
            })
            .collect::<Result<Vec<Option<Spring>>, AocError>>()?;

        let group_lens = parts
            .next()
            .ok_or(AocError::InvalidLine)?
            .split(',')
            .filter_map(|n| n.parse().ok())
            .collect();

        Ok(Record {
            springs,
            group_lens,
        })
    }
}

fn could_work(group_lens: &Vec<usize>, springs: &mut Vec<Option<Spring>>) -> Option<bool> {
    let mut group_num = 0;
    let mut group_len = None;
    for sp in springs.iter() {
        match (&sp, &mut group_len) {
            (Some(Spring::Broken), Some(len)) => {
                *len += 1;
            }
            (Some(Spring::Broken), None) => group_len = Some(1),
            (Some(Spring::Operational), None) => (),
            (Some(Spring::Operational), Some(len)) => {
                if group_lens.get(group_num) != Some(&len) {
                    return Some(false);
                } else {
                    group_len = None;
                    group_num += 1;
                }
            }
            (None, Some(cur_len)) => {
                let Some(&expected_len) = group_lens.get(group_num) else {
                    return Some(false);
                };
                if expected_len < *cur_len {
                    return Some(false);
                } else {
                    return None;
                }
            }
            (None, None) => return None,
        }
    }

    if let Some(len) = group_len {
        if group_lens.get(group_num) != Some(&len) {
            return Some(false);
        }
        group_num += 1;
    }

    if group_lens.len() != group_num {
        return Some(false);
    }

    Some(true)
}

impl Record {
    fn num_working(mut self) -> usize {
        fn inner(group_lens: &Vec<usize>, springs: &mut Vec<Option<Spring>>) -> usize {
            if let Some(first_unknown_pos) = springs.iter_mut().position(|s| s.is_none()) {
                let mut num: usize = 0;
                springs[first_unknown_pos] = Some(Spring::Broken);

                if !matches!(could_work(group_lens, springs), Some(false)) {
                    num += inner(group_lens, springs);
                }
                springs[first_unknown_pos] = Some(Spring::Operational);
                if !matches!(could_work(group_lens, springs), Some(false)) {
                    num += inner(group_lens, springs);
                }
                springs[first_unknown_pos] = None;
                num
            } else {
                if could_work(group_lens, springs).unwrap() {
                    1
                } else {
                    0
                }
            }
        }
        inner(&self.group_lens, &mut self.springs)
    }

    fn repeat(self, times: usize) -> Self {
        assert!(times > 0);
        let Record {
            springs,
            group_lens,
        } = self;
        let mut new_springs = springs.clone();
        for _ in 0..times - 1 {
            new_springs.push(None);
            new_springs.append(&mut springs.clone());
        }

        let group_lens_len = group_lens.len();

        Record {
            springs: new_springs,
            group_lens: group_lens
                .into_iter()
                .cycle()
                .take(group_lens_len * times)
                .collect(),
        }
    }
}

#[test]
fn test_num_working_basecase() {
    assert_eq!(
        "####.#...#... 4,1,1"
            .parse::<Record>()
            .unwrap()
            .num_working(),
        1
    );
    assert_eq!(
        "#....######..#####. 1,6,5"
            .parse::<Record>()
            .unwrap()
            .num_working(),
        1
    );
    assert_eq!(
        "#....######..##### 1,6,5"
            .parse::<Record>()
            .unwrap()
            .num_working(),
        1
    );
    assert_eq!(
        "#....######..##### 1,6,4"
            .parse::<Record>()
            .unwrap()
            .num_working(),
        0
    );
    assert_eq!(
        ".###.##....# 3,2,1"
            .parse::<Record>()
            .unwrap()
            .num_working(),
        1
    );
    assert_eq!(
        ".###.##....# 3,2,1,2"
            .parse::<Record>()
            .unwrap()
            .num_working(),
        0
    );
    assert_eq!(
        "####.#...#... 4,1,1,1"
            .parse::<Record>()
            .unwrap()
            .num_working(),
        0
    );
}

#[test]
fn test_num_working() {
    assert_eq!("???.### 1,1,3".parse::<Record>().unwrap().num_working(), 1);
    assert_eq!(
        ".??..??...?##. 1,1,3"
            .parse::<Record>()
            .unwrap()
            .num_working(),
        4
    );
    assert_eq!(
        "?#?#?#?#?#?#?#? 1,3,1,6"
            .parse::<Record>()
            .unwrap()
            .num_working(),
        1
    );
    assert_eq!(
        "????.#...#... 4,1,1"
            .parse::<Record>()
            .unwrap()
            .num_working(),
        1
    );
    assert_eq!(
        "????.######..#####. 1,6,5"
            .parse::<Record>()
            .unwrap()
            .num_working(),
        4
    );
    assert_eq!(
        "?###???????? 3,2,1"
            .parse::<Record>()
            .unwrap()
            .num_working(),
        10
    );
}

#[test]
fn test_repeat() {
    assert_eq!(
        "???.### 1,1,3".parse::<Record>().unwrap().repeat(2),
        "???.###????.### 1,1,3,1,1,3".parse::<Record>().unwrap()
    );
}

#[test]
fn test_p2() {
    assert_eq!(
        "?###???????? 3,2,1"
            .parse::<Record>()
            .unwrap()
            .repeat(5)
            .num_working(),
        506250
    );
}

fn get_known_answers(prev_output: &str) -> HashMap<usize, usize> {
    prev_output
        .lines()
        .filter(|l| l.starts_with("Finished"))
        .map(|l| {
            let nums = l
                .split_whitespace()
                .filter_map(|word| word.trim_end_matches(':').parse().ok())
                .collect::<Vec<usize>>();
            (nums[0], nums[1])
        })
        .collect()
}

fn process_all(input: &str, times: usize, known_answers: &HashMap<usize, usize>) -> usize {
    let lines: Vec<_> = input.lines().collect();
    lines
        .par_iter()
        .enumerate()
        .map(|(i, l)| {
            if let Some(&known_ans) = known_answers.get(&i) {
                return known_ans;
            }
            let record = l.parse::<Record>().unwrap().repeat(times);
            println!(
                "Processing record {}/{}. {} springs, {} groups, {} unknowns",
                i,
                lines.len(),
                record.springs.len(),
                record.group_lens.len(),
                record.springs.iter().filter(|s| s.is_none()).count(),
            );

            let n = record.num_working();
            println!("Finished {}: {}", i, n);
            n
        })
        .sum()
}

fn part1(input: &str) -> usize {
    process_all(input, 1, &HashMap::new())
}

fn part2(input: &str, known_p2_answers: &HashMap<usize, usize>) -> usize {
    process_all(input, 5, known_p2_answers)
}

fn main() {
    let known_p2_answers = get_known_answers(include_str!("known_p2_answers.txt"));
    let input = include_str!("real_input.txt");
    // println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input, &known_p2_answers));
}
