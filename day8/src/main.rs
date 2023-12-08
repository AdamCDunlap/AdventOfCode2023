use num::integer::lcm;
use std::{collections::{HashMap, HashSet}, str::FromStr};

#[derive(PartialEq, Eq, Debug)]
struct Map {
    left: String,
    right: String,
}

#[derive(PartialEq, Eq, Debug)]
struct Maps {
    directions: String,
    maps: HashMap<String, Map>,
}

impl FromStr for Maps {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let directions = lines.next().ok_or(())?.to_string();
        lines.next(); // skip blank line
        Ok(Maps {
            directions,
            maps: lines
                .map(|l| {
                    let mut eqsplit = l.split('=');
                    let src = eqsplit.next().ok_or(())?.trim().to_string();
                    let dst = eqsplit.next().ok_or(())?.trim();
                    let mut commasplit = dst.split(',');
                    let left = commasplit.next().ok_or(())?.trim()[1..].to_string();
                    let right = commasplit.next().ok_or(())?.trim();
                    let right = right[..right.len() - 1].to_string();

                    Ok((src, Map { left, right }))
                })
                .collect::<Result<HashMap<_, _>, _>>()?,
        })
    }
}

#[test]
fn test_maps_parse() {
    assert_eq!(
        TEST_INPUT2.parse(),
        Ok(Maps {
            directions: "LLR".to_string(),
            maps: [
                (
                    "AAA".to_string(),
                    Map {
                        left: "BBB".to_string(),
                        right: "BBB".to_string()
                    }
                ),
                (
                    "BBB".to_string(),
                    Map {
                        left: "AAA".to_string(),
                        right: "ZZZ".to_string()
                    }
                ),
                (
                    "ZZZ".to_string(),
                    Map {
                        left: "ZZZ".to_string(),
                        right: "ZZZ".to_string()
                    }
                ),
            ]
            .into()
        })
    );
}

impl Maps {
    fn count_steps_from_to(&self, from: &str, to: &HashSet<&str>) -> usize {
        // println!("Counting steps from {} to {}", from, to);
        let mut location = from;
        //let mut visited = HashSet::new();
        for (iteration, dir) in self.directions.chars().cycle().enumerate() {
            //assert!(visited.insert(location));
            let map = self.maps.get(location).unwrap();
            match dir {
                'R' => location = &map.right,
                'L' => location = &map.left,
                _ => unreachable!(),
            }
            if to.contains(location) {
                return iteration + 1;
            }
        }
        unreachable!()
    }

    fn count_steps(&self) -> usize {
        self.count_steps_from_to("AAA", &["ZZZ"].into())
    }

    fn count_ghost_steps(&self) -> usize {
        let start_locations: Vec<&String> = self
            .maps
            .keys()
            .filter(|k| k.as_bytes()[2] == b'A')
            .collect();
        let end_locations = self
            .maps
            .keys()
            .filter(|k| k.as_bytes()[2] == b'Z')
            .map(|s| s.as_str())
            .collect();

        start_locations
            .iter()
            .map(|start| self.count_steps_from_to(start, &end_locations))
            .fold(1, |a, b| lcm(a, b))
    }

    fn count_ghost_steps_naive(&self) -> usize {
        assert!(!self.directions.is_empty());

        let mut locations: Vec<&String> = self
            .maps
            .keys()
            .filter(|k| k.as_bytes()[2] == b'A')
            .collect();
        for (iteration, dir) in self.directions.chars().cycle().enumerate() {
            for loc in locations.iter_mut() {
                let map = self.maps.get(*loc).unwrap();
                match dir {
                    'R' => *loc = &map.right,
                    'L' => *loc = &map.left,
                    _ => unreachable!(),
                }
            }

            if locations.iter().all(|k| k.as_bytes()[2] == b'Z') {
                return iteration + 1;
            }
        }
        unreachable!()
    }
}

fn part1(input: &str) -> usize {
    input.parse::<Maps>().unwrap().count_steps()
}

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT1), 2);
    assert_eq!(part1(TEST_INPUT2), 6);
}
fn part2(input: &str) -> usize {
    input.parse::<Maps>().unwrap().count_ghost_steps()
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT3), 6);
}

fn main() {
    println!("Part 1: {}", part1(REAL_INPUT));
    println!("Part 2: {}", part2(REAL_INPUT));
}

const TEST_INPUT1: &str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;

const TEST_INPUT2: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;

const TEST_INPUT3: &str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;

const REAL_INPUT: &str = include_str!("real_input.txt");
