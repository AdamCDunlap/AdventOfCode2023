#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Mul},
    str::FromStr,
};

use Dir::*;

#[derive(Debug, Clone)]
struct Line {
    dir: Dir,
    len: isize,
}

#[derive(Debug, Clone)]
struct HorizontalLine {
    left: Coord,
    len: isize,
}

impl HorizontalLine {
    fn y(&self) -> isize {
        self.left.y
    }
    fn right_x(&self) -> isize {
        self.left.x + self.len
    }
    fn crosses_x(&self, x: isize) -> bool {
        self.left.x <= x && x < self.right_x()
    }
}

#[derive(Debug)]
struct Directions {
    horizontal_lines: Vec<HorizontalLine>,
    width: usize,
    height: usize,
}

impl Display for Directions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Directions {} x {}", self.width, self.height)?;
        let mut grid = vec![vec![b'.'; self.width]; self.height];
        for HorizontalLine { left, len } in self.horizontal_lines.iter() {
            for i in 0..*len {
                grid[left.y as usize][(left.x + i) as usize] = b'#';
            }
        }

        for line in grid {
            writeln!(f, "{}", std::str::from_utf8(&line).unwrap())?;
        }

        Ok(())
    }
}

impl Directions {
    fn from_part1_str(input: &str) -> Self {
        Self::from_lines(input.lines().map(|l| {
            let mut split = l.split_whitespace();
            let dir = split.next().unwrap().parse().unwrap();
            let len = split.next().unwrap().parse().unwrap();
            Line { dir, len }
        }))
    }
    fn from_part2_str(input: &str) -> Self {
        Self::from_lines(input.lines().map(|line| {
            let dir_code = line.as_bytes()[line.len() - 2];
            let len = isize::from_str_radix(&line[line.len() - 7..line.len() - 2], 16).unwrap();
            let dir = match dir_code {
                b'0' => Right,
                b'1' => Down,
                b'2' => Left,
                b'3' => Up,
                _ => panic!("Invalid dir code {}", dir_code),
            };
            Line { dir, len }
        }))
    }

    fn from_lines(lines: impl Iterator<Item = Line>) -> Self {
        let mut current_coord = Coord { x: 0, y: 0 };
        let mut max_x = 0;
        let mut min_x = 0;
        let mut max_y = 0;
        let mut min_y = 0;

        let mut horizontal_lines: Vec<HorizontalLine> = lines
            .filter_map(|Line { dir, len }| {
                current_coord += dir.to_coord() * len;

                min_x = std::cmp::min(min_x, current_coord.x);
                min_y = std::cmp::min(min_y, current_coord.y);
                max_x = std::cmp::max(max_x, current_coord.x);
                max_y = std::cmp::max(max_y, current_coord.y);

                Some(HorizontalLine {
                    len,
                    left: Coord {
                        y: current_coord.y,
                        x: match dir {
                            Up | Down => return None,
                            Left => current_coord.x,
                            Right => current_coord.x - len,
                        },
                    },
                })
            })
            .collect();

        assert_eq!(current_coord, Coord { x: 0, y: 0 });

        println!("Orig:  {horizontal_lines:?}");

        horizontal_lines
            .iter_mut()
            .for_each(|HorizontalLine { left, .. }| {
                *left += Coord {
                    x: -min_x,
                    y: -min_y,
                }
            });
        println!("Fixed: {horizontal_lines:?}\nmin_x: {min_x} min_y: {min_y}");

        Self {
            horizontal_lines,
            width: (max_x - min_x + 1) as usize,
            height: (max_y - min_y + 1) as usize,
        }
    }

    fn get_enclosed_area(&self) -> u64 {
        let mut segments_to_consider = self.horizontal_lines.clone();
        let mut area = 0;
        while let Some(mut segment_to_consider) = segments_to_consider.pop() {
            println!("Considering {segment_to_consider:?}");
            let start_x = segment_to_consider.left.x;

            let mut num_above = 0;
            let mut first_below: Option<HorizontalLine> = None;

            self.horizontal_lines.iter().for_each(|other_line| {
                let crosses_left = other_line.crosses_x(start_x);
                let crosses_right = other_line.crosses_x(segment_to_consider.right_x());

                if other_line.left.y < segment_to_consider.left.y {
                    if crosses_left {
                        num_above += 1;
                    }
                } else if (crosses_left || crosses_right)
                    && other_line.left.y != segment_to_consider.left.y
                {
                    if let Some(prev_highest) = &first_below {
                        if other_line.left.y < prev_highest.left.y {
                            first_below = Some(other_line.clone());
                        }
                    } else {
                        first_below = Some(other_line.clone());
                    }
                }
            });

            println!("num_above: {num_above}");

            if num_above % 2 == 1 {
                // Odd number of lines above this line means that the area below this line is not in bounds
                continue;
            }
            let first_below = first_below.expect("Not a loop");

            // let this_right = start_x + segment_to_consider.len;
            // let below_right = first_below.left.x + first_below.len;
            if first_below.left.x > start_x {
                let right = HorizontalLine {
                    left: Coord {
                        x: first_below.left.x,
                        y: segment_to_consider.y(),
                    },
                    len: first_below.left.x - start_x - 1,
                };
                let below = HorizontalLine {
                    left: Coord {
                        x: start_x,
                        y: first_below.y(),
                    },
                    len: segment_to_consider.len,
                };
                let prev = segment_to_consider.clone();
                segment_to_consider.len -= right.len + 1;
                println!(
                    "Splitting {:?} into {:?}, {:?}, and {:?}",
                    prev, segment_to_consider, below, right
                );

                segments_to_consider.push(below);
                segments_to_consider.push(right);
            } else if first_below.right_x() < segment_to_consider.right_x() {
                assert!(first_below.right_x() > start_x);
                let other = HorizontalLine {
                    left: Coord {
                        x: first_below.right_x(),
                        y: segment_to_consider.left.y,
                    },
                    len: segment_to_consider.right_x() - first_below.right_x(),
                };
                let prev = segment_to_consider.clone();
                segment_to_consider.len -= other.len + 1;
                println!(
                    "Splitting {:?} into {:?} and {:?}",
                    prev, segment_to_consider, other
                );

                segments_to_consider.push(other);
            }

            let width: u64 = (segment_to_consider.right_x() - start_x + 1)
                .try_into()
                .unwrap();
            let height: u64 = (first_below.y() - segment_to_consider.y() + 1)
                .try_into()
                .unwrap();
            println!("width: {width} height: {height}");

            area += width * height;
        }
        area
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Coord {
    x: isize,
    y: isize,
}

impl Add<Dir> for Coord {
    type Output = Coord;

    fn add(self, rhs: Dir) -> Self::Output {
        rhs.to_coord() + self
    }
}

impl Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Dir {
    fn to_coord(self) -> Coord {
        match self {
            Up => Coord { x: 0, y: -1 },
            Down => Coord { x: 0, y: 1 },
            Right => Coord { x: 1, y: 0 },
            Left => Coord { x: -1, y: 0 },
        }
    }
}

impl FromStr for Dir {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "R" => Right,
            "L" => Left,
            "U" => Up,
            "D" => Down,
            _ => return Err(()),
        })
    }
}

impl Mul<isize> for Coord {
    type Output = Coord;

    fn mul(self, rhs: isize) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

fn part1(input: &str) -> u64 {
    let directions = Directions::from_part1_str(input);
    // println!("{directions}");
    directions.get_enclosed_area()
}

#[test]
fn test_part1() {
    
    assert_eq!(part1("D 10000\nR 10\nU 10000\nL 10"), 10001 * 11);
    
    assert_eq!(part1("D 8\nR 4\nU 2\nL 1\nU 3\nR 1\nU 3\nL 4"), 5*9 - 2);

    assert_eq!(part1(TEST_INPUT), 62);
}

fn part2(input: &str) -> u64 {
    let directions = Directions::from_part2_str(input);
    directions.get_enclosed_area()
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 952408144115);
}

fn main() {
    let input = &std::fs::read_to_string("input.txt").expect("input.txt should exist");
    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

const TEST_INPUT: &str = r"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
