#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
use std::{
    collections::VecDeque,
    ops::{Add, AddAssign, Mul},
    str::FromStr,
};

use Dir::*;

#[derive(Clone)]
struct Line {
    dir: Dir,
    len: usize,
    start: Coord,
}

impl Line {
    fn left_side(&self) -> isize {
        match self.dir {
            Up | Down => panic!("can only be called on a horizontal line"),
            Left => self.start.x - self.len as isize,
            Right => self.start.x,
        }
    }
    fn right_side(&self) -> isize {
        match self.dir {
            Up | Down => panic!("can only be called on a horizontal line"),
            Left => self.start.x,
            Right => self.start.x + self.len as isize,
        }
    }
}

struct Directions {
    lines: Vec<Line>,
    width: usize,
    height: usize,
}

impl Directions {
    fn from_part1_str(input: &str) -> Self {
        Self::from_line_segments(
            input
                .lines()
                .map(|l| {
                    let strs: [&str; 3] = l
                        .split_whitespace()
                        .collect::<Vec<&str>>()
                        .try_into()
                        .unwrap();
                    let dir: Dir = strs[0].parse().unwrap();
                    let len: usize = strs[1].parse().unwrap();
                    Line {
                        dir,
                        len,
                        start: Coord {
                            x: isize::MIN,
                            y: isize::MIN,
                        },
                    }
                })
                .collect(),
        )
    }
    fn from_part2_str(input: &str) -> Self {
        Self::from_line_segments(
            input
                .lines()
                .map(|line| {
                    let dir_code = line.as_bytes()[line.len() - 2];
                    let len =
                        usize::from_str_radix(&line[line.len() - 7..line.len() - 2], 16).unwrap();
                    let dir = match dir_code {
                        b'0' => Right,
                        b'1' => Down,
                        b'2' => Left,
                        b'3' => Up,
                        _ => panic!("Invalid dir code {}", dir_code),
                    };
                    Line {
                        dir,
                        len,
                        start: Coord {
                            x: isize::MIN,
                            y: isize::MIN,
                        },
                    }
                })
                .collect(),
        )
    }

    fn from_line_segments(mut lines: Vec<Line>) -> Self {
        let mut current_coord = Coord { x: 0, y: 0 };
        let mut max_x = 0;
        let mut min_x = 0;
        let mut max_y = 0;
        let mut min_y = 0;

        for Line { dir, len, start: _ } in lines.iter() {
            current_coord += dir.to_coord() * *len as isize;
            min_x = std::cmp::min(min_x, current_coord.x);
            min_y = std::cmp::min(min_y, current_coord.y);
            max_x = std::cmp::max(max_x, current_coord.x);
            max_y = std::cmp::max(max_y, current_coord.y);
        }

        assert_eq!(current_coord, Coord { x: 0, y: 0 });

        let mut current_coord = Coord {
            x: -min_x,
            y: -min_y,
        };

        for Line { dir, len, start } in lines.iter_mut() {
            *start = current_coord.clone();
            current_coord += dir.to_coord() * *len as isize;
        }

        Self {
            lines,
            width: (max_x - min_x + 1) as usize,
            height: (max_y - min_y + 1) as usize,
        }
    }

    // fn calculate_dimensions(&self) -> Coord {
    //     let mut current_coord = Coord { x: 0, y: 0 };
    //     let mut max_x = 0;
    //     let mut min_x = 0;
    //     let mut max_y = 0;
    //     let mut min_y = 0;

    //     for LineSegment { dir, len } in self.0.iter() {
    //         current_coord += dir.to_coord() * *len as isize;
    //         min_x = std::cmp::min(min_x, current_coord.x);
    //         min_y = std::cmp::min(min_y, current_coord.y);
    //         max_x = std::cmp::max(max_x, current_coord.x);
    //         max_y = std::cmp::max(max_y, current_coord.y);
    //     }

    //     assert_eq!(current_coord, Coord { x: 0, y: 0 });

    //     Coord {
    //         x: max_x - min_x + 1,
    //         y: max_y - min_y + 1,
    //     }
    // }

    fn always_turns(&self) -> bool {
        self.lines.windows(2).all(|win| win[0].dir != win[1].dir)
    }

    fn horizontal_lines_crossing_x(&self, x: isize) -> impl Iterator<Item = &Line> {
        self.lines.iter().filter(move |line| match line.dir {
            Up | Down => false,
            Left | Right => line.left_side() <= x && x < line.right_side(),
        })
    }

    fn get_enclosed_area(&self) -> usize {
        let mut segments_to_consider = self.lines.clone();
        let mut area = 0;
        while let Some(segment_to_consider) = segments_to_consider.pop() {
            let start_x = segment_to_consider.start.x;

            let mut num_above = 0;
            let mut first_below: Option<Line> = None;

            self.horizontal_lines_crossing_x(start_x)
                .for_each(|other_line| {
                    if other_line.start.y < segment_to_consider.start.y {
                        num_above += 1;
                    } else {
                        if let Some(prev_highest) = &first_below {
                            if other_line.start.y < prev_highest.start.y {
                                first_below = Some(other_line.clone());
                            }
                        } else {
                            first_below = Some(other_line.clone());
                        }
                    }
                });
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

fn part1(input: &str) -> usize {
    let directions = Directions::from_part1_str(input);
    directions.get_enclosed_area()
}

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 62);
}

fn part2(input: &str) -> usize {
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
