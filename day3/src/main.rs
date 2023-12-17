
#[cfg(test)]
const TEST_INPUT: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

#[derive(Debug, PartialEq, Eq)]
struct PartNumber {
    num: u32,
    row: usize,
    start_col: usize,
    end_col: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct Point {
    row: usize,
    col: usize,
}

impl PartNumber {
    fn is_adjacent_to(&self, p: &Point) -> bool {
        p.row >= self.row.saturating_sub(1)
            && p.row <= self.row.saturating_add(1)
            && p.col >= self.start_col.saturating_sub(1)
            && p.col <= self.end_col
    }
}

#[test]
fn test_is_adjacent_to() {
    let pn = PartNumber {
        num: 42,
        row: 3,
        start_col: 5,
        end_col: 8,
    };
    assert!(pn.is_adjacent_to(&Point { row: 2, col: 4 }));
    assert!(pn.is_adjacent_to(&Point { row: 3, col: 4 }));
    assert!(pn.is_adjacent_to(&Point { row: 4, col: 4 }));
    assert!(pn.is_adjacent_to(&Point { row: 2, col: 5 }));
    assert!(pn.is_adjacent_to(&Point { row: 4, col: 6 }));
    assert!(pn.is_adjacent_to(&Point { row: 4, col: 8 }));

    assert!(!pn.is_adjacent_to(&Point { row: 5, col: 6 }));
    assert!(!pn.is_adjacent_to(&Point { row: 1, col: 6 }));

    assert!(!pn.is_adjacent_to(&Point { row: 4, col: 3 }));
    assert!(!pn.is_adjacent_to(&Point { row: 4, col: 9 }));

    let pn = PartNumber {
        num: 42,
        row: 3,
        start_col: 0,
        end_col: 5,
    };
    assert!(pn.is_adjacent_to(&Point { row: 4, col: 0 }));
    assert!(!pn.is_adjacent_to(&Point { row: 4, col: 6 }));
}

fn finish_number(
    line: &str,
    row: usize,
    end_col: usize,
    start_col: &mut Option<usize>,
) -> PartNumber {
    let start_col = start_col.take().unwrap();
    PartNumber {
        num: line[start_col..end_col].parse().unwrap(),
        row,
        start_col,
        end_col,
    }
}

fn extract_part_numbers(schematic: &str) -> Vec<PartNumber> {
    let mut part_numbers = Vec::new();
    for (row, line) in schematic.lines().enumerate() {
        let mut num_start: Option<usize> = None;
        for (col, ch) in line.bytes().enumerate() {
            match (ch.is_ascii_digit(), num_start.is_some()) {
                (true, true) => {}   // Number is continuing
                (false, false) => {} // Non-number is continuing
                (true, false) => {
                    num_start = Some(col); // Number is starting
                }
                (false, true) => {
                    part_numbers.push(finish_number(line, row, col, &mut num_start));
                }
            }
        }
        if num_start.is_some() {
            part_numbers.push(finish_number(line, row, line.len(), &mut num_start));
        }
    }
    part_numbers
}

#[test]
fn test_extract_part_numbers() {
    assert_eq!(
        extract_part_numbers(TEST_INPUT)[..3],
        [
            PartNumber {
                num: 467,
                row: 0,
                start_col: 0,
                end_col: 3,
            },
            PartNumber {
                num: 114,
                row: 0,
                start_col: 5,
                end_col: 8,
            },
            PartNumber {
                num: 35,
                row: 2,
                start_col: 2,
                end_col: 4,
            },
        ]
    );

    assert_eq!(
        extract_part_numbers("1.2\n3.4"),
        [
            PartNumber {
                num: 1,
                row: 0,
                start_col: 0,
                end_col: 1,
            },
            PartNumber {
                num: 2,
                row: 0,
                start_col: 2,
                end_col: 3,
            },
            PartNumber {
                num: 3,
                row: 1,
                start_col: 0,
                end_col: 1,
            },
            PartNumber {
                num: 4,
                row: 1,
                start_col: 2,
                end_col: 3,
            },
        ]
    );
}

fn is_symbol(ch: u8) -> bool {
    ch != b'.' && !ch.is_ascii_digit()
}

fn is_gear_symbol(ch: u8) -> bool {
    ch == b'*'
}

fn get_valid_parts(schematic: &str) -> Vec<u32> {
    let mut parts = extract_part_numbers(schematic);
    let mut are_valid = vec![false; parts.len()];
    for (row, line) in schematic.lines().enumerate() {
        for (col, ch) in line.bytes().enumerate() {
            let pt = &Point { row, col };
            if is_symbol(ch) {
                for (valid, pn) in Iterator::zip(are_valid.iter_mut(), parts.iter()) {
                    if *valid {
                        continue;
                    }
                    if !pn.is_adjacent_to(pt) {
                        continue;
                    }
                    *valid = true;
                }
            }
        }
    }
    parts
        .drain(..)
        .enumerate()
        .filter(|(idx, _)| are_valid[*idx])
        .map(|(_, pn)| pn.num)
        .collect()
}

#[test]
fn test_get_valid_parts() {
    assert_eq!(
        get_valid_parts(TEST_INPUT),
        [467, 35, 633, 617, 592, 755, 664, 598,]
    );
}

fn part1(schematic: &str) -> u32 {
    get_valid_parts(schematic).into_iter().sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 4361);
}

fn get_gears(schematic: &str) -> Vec<(u32, u32)> {
    let parts = extract_part_numbers(schematic);
    let mut gears = Vec::new();
    for (row, line) in schematic.lines().enumerate() {
        'chloop: for (col, ch) in line.bytes().enumerate() {
            let pt = &Point { row, col };
            if !is_gear_symbol(ch) {
                continue;
            }
            let mut adjacent_parts = Vec::new();
            for pn in parts.iter() {
                if !pn.is_adjacent_to(pt) {
                    continue;
                }
                if adjacent_parts.len() >= 2 {
                    break 'chloop
                }
                adjacent_parts.push(pn.num);
            }
            if adjacent_parts.len() == 2 {
                gears.push((adjacent_parts[0], adjacent_parts[1]));
            }
        }
    }
    gears
}

#[test]
fn test_get_gears() {
    assert_eq!(
        get_gears(TEST_INPUT),
        [(467, 35), (755, 598)]
    );
}

fn part2(schematic: &str) -> u32 {
    get_gears(schematic).into_iter().map(|(a, b)| a * b).sum()
}

#[test]
fn test_part2() {
    assert_eq!(
        part2(TEST_INPUT),
        467835
    );
}


fn main() {
    let input = &std::fs::read_to_string("input.txt").expect("input.txt should exist");
    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}
