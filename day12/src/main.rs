use rayon::prelude::*;
use std::{collections::HashMap, fmt::Display, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
enum AocError {
    InvalidLine,
    InvalidSpringType,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Spring {
    Broken,
    Operational,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Record {
    springs: Vec<Option<Spring>>,
    group_lens: Vec<usize>,
    ends_in_group: bool,
}

impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        f.write_char('"')?;
        for sp in self.springs.iter() {
            f.write_char(match sp {
                Some(Spring::Broken) => '#',
                Some(Spring::Operational) => '.',
                None => '?',
            })?;
        }

        f.write_char(' ')?;

        for (i, l) in self.group_lens.iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }
            f.write_fmt(format_args!("{}", l))?;
        }

        if self.ends_in_group {
            f.write_char('+')?;
        }

        f.write_char('"')?;

        Ok(())
    }
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
            ends_in_group: false,
        })
    }
}

impl Record {
    fn num_working_uncached(mut self, cache: &mut HashMap<Record, usize>, depth: usize) -> usize {
        // println!("{}Starting with {}", " ".repeat(depth), self);
        // Trim non-unknowns off the end
        loop {
            // print!("{}Looping. cur: {}", " ".repeat(depth), self);
            match self.springs.pop() {
                Some(Some(Spring::Operational)) => {
                    // A 0 means that the last group was "used up" but not "ended" by seeing
                    // another operational spring. If we see that, then pop it off since the
                    // group has ended upon seeing this.
                    if self.group_lens.last() == Some(&0) {
                        if !self.ends_in_group {
                            // println!("-> failure, not in a group, but last was 0");
                            return 0;
                        }
                        self.group_lens.pop().unwrap();
                        self.ends_in_group = false;
                    } else if self.ends_in_group {
                        // println!("-> failure, in a group, but last was not 0");
                        return 0;
                    }
                }
                Some(Some(Spring::Broken)) => {
                    let Some(last_len) = self.group_lens.last_mut() else {
                        // There's a broken spring, but no group lens left, so this configuration is impossible.
                        // println!(" -> failure since group_lens is empty");
                        return 0;
                    };
                    if *last_len == 0 {
                        // A 0 means that the last group was "used up" but not "ended" by seeing another operational spring.
                        // println!(" -> failure since group_lens.last == 0");
                        return 0;
                    }
                    self.ends_in_group = true;
                    *last_len -= 1;
                }
                Some(None) => {
                    // The last item popped was an unknown. Continue below the loop to limit indentation
                    break;
                }
                None => {
                    // We hit the end of the springs list without hitting any unknowns, so we can give an immediate answer.

                    return if self.group_lens.is_empty()
                        || (self.group_lens.len() == 1 && self.group_lens[0] == 0)
                    {
                        // println!("-> success");
                        1
                    } else {
                        // println!("-> failure, out of springs");
                        0
                    };
                }
            }
            // println!();
        }

        // println!(" -> recursing");

        // The last item popped was an unknown.
        let mut result = 0;
        self.springs.push(Some(Spring::Operational));
        result += self.num_working_cached(cache, depth + 2);
        *self.springs.last_mut().unwrap() = Some(Spring::Broken);
        result += self.num_working_cached(cache, depth + 2);

        *self.springs.last_mut().unwrap() = None;
        // println!(
        //     "{}Done recursing; {} returned {}",
        //     " ".repeat(depth),
        //     self,
        //     result
        // );

        result
    }

    fn num_working_cached(&self, cache: &mut HashMap<Record, usize>, depth: usize) -> usize {
        if let Some(result) = cache.get(self) {
            // println!("{}Got from cache {} -> {}", " ".repeat(depth), self, result);
            return *result;
        }
        let result = self.clone().num_working_uncached(cache, depth + 2);
        cache.insert(self.clone(), result);
        result
    }

    fn num_working(&self) -> usize {
        self.num_working_cached(&mut HashMap::new(), 0)
    }

    fn repeat(self, times: usize) -> Self {
        assert!(times > 0);
        let Record {
            springs,
            group_lens,
            ends_in_group: _,
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
            ends_in_group: false,
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
fn test_num_working_full() {
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
    assert_eq!("??? 2".parse::<Record>().unwrap().num_working(), 2);
    assert_eq!("???? 2".parse::<Record>().unwrap().num_working(), 3);
    assert_eq!("?###????? 3,2".parse::<Record>().unwrap().num_working(), 3);
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
            //println!(
            //    "Processing record {}/{}. {} springs, {} groups, {} unknowns",
            //    i,
            //    lines.len(),
            //    record.springs.len(),
            //    record.group_lens.len(),
            //    record.springs.iter().filter(|s| s.is_none()).count(),
            //);

            let n = record.num_working();
            //println!("Finished {}: {}", i, n);
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
    let input = &std::fs::read_to_string("input.txt").expect("input.txt should exist");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input, &HashMap::new()));
}
