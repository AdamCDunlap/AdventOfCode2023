use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
enum AocError {
    NoSeedsLine,
    InvalidMapName,
    InvalidMapLine,
    DataBeforeMaps,
    NoMapFrom(String),
}

#[derive(Debug, PartialEq, Eq)]
struct MapEntry {
    dst_start: u64,
    src_start: u64,
    len: u64,
}

impl MapEntry {
    fn contains(&self, src: u64) -> bool {
        src >= self.src_start && src < self.src_start + self.len
    }
    fn translate(&self, src: u64) -> Option<u64> {
        if self.contains(src) {
            Some(src - self.src_start + self.dst_start)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Range {
    first: u64,
    len: u64,
}

#[derive(Debug, PartialEq, Eq)]
struct Map {
    src_name: String,
    dst_name: String,
    // invariant: Entries' source ranges do not overlap and are sorted by the source range.
    entries: Vec<MapEntry>,
}

#[derive(Debug, PartialEq, Eq)]
struct TranslatePartialResult {
    translated: Range,
    remaining: Range,
}

impl TranslatePartialResult {
    fn new(full_range: &Range, first_result: u64, max_to_translate: Option<u64>) -> Self {
        let len_translated = min_with_none(full_range.len, max_to_translate);

        TranslatePartialResult {
            translated: Range {
                first: first_result,
                len: len_translated,
            },
            remaining: Range {
                first: full_range.first + len_translated,
                len: full_range.len - len_translated,
            },
        }
    }
}

fn min_with_none(a: u64, b: Option<u64>) -> u64 {
    if b.is_none() {
        a
    } else {
        std::cmp::min(a, b.unwrap())
    }
}

impl Map {
    fn translate(&self, src: u64) -> u64 {
        self.entries
            .iter()
            .find_map(|ent| ent.translate(src))
            .unwrap_or(src) // If no matching entries, identity map
    }

    fn translate_partial_identity(
        to_translate: Range,
        next_entry: Option<&MapEntry>,
    ) -> TranslatePartialResult {
        TranslatePartialResult::new(
            &to_translate,
            to_translate.first,
            next_entry.map(|e| e.src_start - to_translate.first),
        )
    }

    /// Translates the first part of `to_translate`, returning one translated range and the rest of the range that was not translated.
    fn translate_partial(&self, to_translate: Range) -> TranslatePartialResult {
        let location = self
            .entries
            .partition_point(|ent| ent.src_start <= to_translate.first);
        if location == 0 {
            // to_translate.first is before any of the map entries, so identity map it up to the start of the first entry.
            Map::translate_partial_identity(to_translate, self.entries.first())
        } else {
            let prev = &self.entries[location - 1];
            if prev.contains(to_translate.first) {
                let len_to_translate = prev.src_start + prev.len - to_translate.first;

                TranslatePartialResult::new(
                    &to_translate,
                    prev.translate(to_translate.first).unwrap(),
                    Some(len_to_translate),
                )
            } else {
                // Identity map up to the next entry, if there is one
                Map::translate_partial_identity(to_translate, self.entries.get(location))
            }
        }
    }

    fn translate_range(&self, to_translate: &Range) -> Vec<Range> {
        let mut remaining = to_translate.clone();
        let mut translated = vec![];
        while remaining.len > 0 {
            let partial = self.translate_partial(remaining);
            translated.push(partial.translated);
            remaining = partial.remaining.clone();
        }
        translated
    }
}

#[cfg(test)]
fn get_test_map() -> Map {
    Map {
        src_name: "".into(),
        dst_name: "".into(),
        entries: [
            MapEntry {
                src_start: 5,
                dst_start: 105,
                len: 10,
            },
            MapEntry {
                src_start: 15,
                dst_start: 215,
                len: 10,
            },
            MapEntry {
                src_start: 30,
                dst_start: 330,
                len: 20,
            },
        ]
        .into(),
    }
}

#[test]
fn translate_partial_range_completely_before_first_test() {
    assert_eq!(
        get_test_map().translate_partial(Range { first: 1, len: 2 }),
        TranslatePartialResult {
            translated: Range { first: 1, len: 2 },
            remaining: Range { first: 3, len: 0 },
        }
    );
}

#[test]
fn translate_partial_range_partially_before_first_test() {
    assert_eq!(
        get_test_map().translate_partial(Range { first: 1, len: 50 }),
        TranslatePartialResult {
            translated: Range { first: 1, len: 4 },
            remaining: Range { first: 5, len: 46 },
        }
    );
}

#[test]
fn translate_partial_range_starting_at_first_test() {
    assert_eq!(
        get_test_map().translate_partial(Range { first: 5, len: 50 }),
        TranslatePartialResult {
            translated: Range {
                first: 105,
                len: 10
            },
            remaining: Range { first: 15, len: 40 },
        }
    );
}

#[test]
fn translate_partial_range_starting_within_first_test() {
    assert_eq!(
        get_test_map().translate_partial(Range { first: 6, len: 50 }),
        TranslatePartialResult {
            translated: Range { first: 106, len: 9 },
            remaining: Range { first: 15, len: 41 },
        }
    );
}

#[test]
fn translate_partial_range_starting_at_break_point() {
    assert_eq!(
        get_test_map().translate_partial(Range { first: 15, len: 50 }),
        TranslatePartialResult {
            translated: Range {
                first: 215,
                len: 10
            },
            remaining: Range { first: 25, len: 40 },
        }
    );
}

#[test]
fn translate_partial_range_starting_in_empty_range() {
    assert_eq!(
        get_test_map().translate_partial(Range { first: 27, len: 50 }),
        TranslatePartialResult {
            translated: Range { first: 27, len: 3 },
            remaining: Range { first: 30, len: 47 },
        }
    );
}

#[test]
fn translate_range_test() {
    assert_eq!(
        get_test_map().translate_range(&Range { first: 10, len: 30 }),
        [
            Range { first: 110, len: 5 },
            Range {
                first: 215,
                len: 10
            },
            Range { first: 25, len: 5 },
            Range {
                first: 330,
                len: 10
            }
        ]
        .to_vec()
    )
}

fn parse_numbers(num_list: &str) -> Vec<u64> {
    num_list.split(' ').filter_map(|s| s.parse().ok()).collect()
}

#[derive(Debug, PartialEq, Eq)]
struct Almanac {
    seeds: Vec<Range>,
    maps: Vec<Map>,
}

impl FromStr for Almanac {
    type Err = AocError;

    fn from_str(input: &str) -> Result<Almanac, AocError> {
        let mut lines = input.lines();

        let seeds_line = lines.next().ok_or(AocError::NoSeedsLine)?;
        let seeds_nums = parse_numbers(
            &seeds_line
                .strip_prefix("seeds: ")
                .ok_or(AocError::NoSeedsLine)?,
        );
        let seeds = seeds_nums
            .chunks(2)
            .map(|vals| Range {
                first: vals[0],
                len: vals[1],
            })
            .collect();

        let mut almanac = Almanac {
            seeds,
            maps: vec![],
        };

        for line in lines {
            if line.trim().is_empty() {
                continue;
            }
            // If the line ends with map:, then create a new map
            if let Some(map_name) = line.strip_suffix(" map:") {
                let name_parts: Vec<&str> = map_name.trim().split('-').collect();
                if name_parts.len() != 3 {
                    return Err(AocError::InvalidMapName);
                }
                if name_parts[1] != "to" {
                    return Err(AocError::InvalidMapName);
                }
                almanac.maps.push(Map {
                    src_name: name_parts[0].into(),
                    dst_name: name_parts[2].into(),
                    entries: vec![],
                });
            } else {
                // Otherwise, add to the entries of the last map
                let nums = parse_numbers(line);
                if nums.len() != 3 {
                    dbg!(line);
                    dbg!(nums);
                    return Err(AocError::InvalidMapLine);
                }
                almanac
                    .maps
                    .last_mut()
                    .ok_or(AocError::DataBeforeMaps)?
                    .entries
                    .push(MapEntry {
                        dst_start: nums[0],
                        src_start: nums[1],
                        len: nums[2],
                    });
            }
        }

        // Sort entries in each map
        for map in almanac.maps.iter_mut() {
            map.entries.sort_unstable_by_key(|me| me.src_start);
        }

        Ok(almanac)
    }
}

#[test]
fn test_parse_almanac() {
    assert_eq!(
        r#"seeds: 1 2

        seed-to-soil map:
        3 4 5
        6 7 8
        
        soil-to-fertilizer map:
        9 10 11"#
            .parse(),
        Ok(Almanac {
            seeds: vec![Range { first: 1, len: 2 }],
            maps: [
                Map {
                    src_name: "seed".into(),
                    dst_name: "soil".into(),
                    entries: [
                        MapEntry {
                            dst_start: 3,
                            src_start: 4,
                            len: 5
                        },
                        MapEntry {
                            dst_start: 6,
                            src_start: 7,
                            len: 8
                        },
                    ]
                    .into()
                },
                Map {
                    src_name: "soil".into(),
                    dst_name: "fertilizer".into(),
                    entries: [MapEntry {
                        dst_start: 9,
                        src_start: 10,
                        len: 11
                    }]
                    .into()
                },
            ]
            .into()
        })
    )
}

impl Almanac {
    fn find_map(&self, from_type: &str) -> Result<&Map, AocError> {
        self.maps
            .iter()
            .find(|m| m.src_name == from_type)
            .ok_or_else(|| AocError::NoMapFrom(from_type.into()))
    }

    fn translate(
        &self,
        from_type: &str,
        to_type: &str,
        initial_range: Range,
    ) -> Result<Vec<Range>, AocError> {
        let mut cur_type = from_type;
        let mut cur_ranges = vec![initial_range];
        while cur_type != to_type {
            let map = self.find_map(cur_type)?;
            cur_ranges = cur_ranges
                .iter()
                .flat_map(|range| map.translate_range(range))
                .collect();
            cur_type = &map.dst_name;
        }

        Ok(cur_ranges)
    }
}

const TEST_INPUT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

#[test]
fn test_almanac_translate() {
    assert_eq!(
        TEST_INPUT.parse::<Almanac>().unwrap().translate(
            "seed",
            "soil",
            Range { first: 79, len: 1 }
        ),
        Ok(vec![Range { first: 81, len: 1 }])
    );
    assert_eq!(
        TEST_INPUT.parse::<Almanac>().unwrap().translate(
            "seed",
            "location",
            Range { first: 79, len: 1 }
        ),
        Ok(vec![Range { first: 82, len: 1 }])
    );
    assert_eq!(
        TEST_INPUT.parse::<Almanac>().unwrap().translate(
            "seed",
            "location",
            Range { first: 14, len: 1 }
        ),
        Ok(vec![Range { first: 43, len: 1 }])
    );
    assert_eq!(
        TEST_INPUT.parse::<Almanac>().unwrap().translate(
            "seed",
            "location",
            Range { first: 55, len: 1 }
        ),
        Ok(vec![Range { first: 86, len: 1 }])
    );
    assert_eq!(
        TEST_INPUT.parse::<Almanac>().unwrap().translate(
            "seed",
            "location",
            Range { first: 13, len: 1 }
        ),
        Ok(vec![Range { first: 35, len: 1 }])
    );
}

// fn part1(input: &str) -> u64 {
//     let almanac: Almanac = input.parse().unwrap();

//     almanac
//         .seeds
//         .iter()
//         .map(|seed| almanac.translate("seed", "location", *seed).unwrap())
//         .min()
//         .unwrap()
// }

// #[test]
// fn test_part1() {
//     assert_eq!(part1(TEST_INPUT), 35);
// }

fn part2(input: &str) -> u64 {
    let almanac: Almanac = input.parse().unwrap();

    almanac
        .seeds
        .iter()
        .flat_map(|seed| almanac.translate("seed", "location", seed.clone()).unwrap())
        .map(|loc_range| loc_range.first)
        .min()
        .unwrap()
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 46);
}

fn main() {
    let input = &std::fs::read_to_string("input.txt").expect("input.txt should exist");
    println!("part 2: {}", part2(input));
}
