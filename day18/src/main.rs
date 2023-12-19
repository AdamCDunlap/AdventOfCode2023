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
    y: isize,
    left: isize,
    right: isize,
}

impl HorizontalLine {
    fn crosses_x(&self, x: isize) -> bool {
        self.left <= x && x <= self.right
    }
}

#[derive(Debug)]
struct Directions {
    horizontal_lines: Vec<HorizontalLine>,
    width: usize,
    height: usize,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Coord,
    bottom_right: Coord,
}

impl Rectangle {
    fn area(&self) -> u64 {
        let width = (self.bottom_right.x - self.top_left.x + 1) as u64;
        let height = (self.bottom_right.y - self.top_left.y + 1) as u64;
        width * height
    }
}

impl Display for Directions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Directions {} x {}", self.width, self.height)?;
        let mut grid = vec![vec![b'.'; self.width]; self.height];
        for HorizontalLine { y, left, right } in self.horizontal_lines.iter() {
            for x in *left..=*right {
                grid[*y as usize][x as usize] = b'#';
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
                let prev_coord = current_coord.clone();
                current_coord += dir.to_coord() * len;

                min_x = std::cmp::min(min_x, current_coord.x);
                min_y = std::cmp::min(min_y, current_coord.y);
                max_x = std::cmp::max(max_x, current_coord.x);
                max_y = std::cmp::max(max_y, current_coord.y);

                // println!("Considering line {dir:?} {len}: prev was {prev_coord:?}, current is {current_coord:?}");

                if prev_coord.y != current_coord.y {
                    return None;
                }

                let left = std::cmp::min(prev_coord.x, current_coord.x);
                let right = std::cmp::max(prev_coord.x, current_coord.x);

                Some(HorizontalLine {
                    y: current_coord.y,
                    left,
                    right,
                })
            })
            .collect();

        assert_eq!(current_coord, Coord { x: 0, y: 0 });

        println!("Orig:  {horizontal_lines:?}");

        horizontal_lines
            .iter_mut()
            .for_each(|HorizontalLine { y, left, right }| {
                *y -= min_y;
                *left -= min_x;
                *right -= min_x;
            });
        println!("Fixed: {horizontal_lines:?}\nmin_x: {min_x} min_y: {min_y}");

        Self {
            horizontal_lines,
            width: (max_x - min_x + 1) as usize,
            height: (max_y - min_y + 1) as usize,
        }
    }

    fn get_enclosed_rectangles(&self) -> Vec<Rectangle> {
        let mut segments_to_consider: Vec<_> = self
            .horizontal_lines
            .iter()
            .filter(|line| {
                let mut seems_like_top = true;
                self.horizontal_lines.iter().for_each(|other_line| {
                    if other_line.y < line.y && other_line.crosses_x(line.left) {
                        seems_like_top = !seems_like_top;
                    }
                });
                seems_like_top
            })
            .cloned()
            .collect();
        let mut rectangles = vec![];
        while let Some(mut top_line) = segments_to_consider.pop() {
            println!("Considering {top_line:?}");

            let first_below = self
                .horizontal_lines
                .iter()
                .filter(|other_line| {
                    other_line.y > top_line.y
                        && (other_line.crosses_x(top_line.left)
                            || other_line.crosses_x(top_line.right))
                })
                .min_by_key(|other_line| other_line.y)
                .expect("Not a loop");

            // let mut first_below: Option<HorizontalLine> = None;

            // self.horizontal_lines.iter().for_each(|other_line| {
            //     let crosses_left = other_line.crosses_x(top_line.left);
            //     let crosses_right = other_line.crosses_x(top_line.right);

            //     if (crosses_left || crosses_right) && other_line.y != top_line.y {
            //         if let Some(prev_highest) = &first_below {
            //             if other_line.y < prev_highest.y {
            //                 first_below = Some(other_line.clone());
            //             }
            //         } else {
            //             first_below = Some(other_line.clone());
            //         }
            //     }
            // });

            // println!("first_below: {first_below:?}");

            // let first_below = first_below.expect("Not a loop");

            if first_below.left > top_line.left {
                let right = HorizontalLine {
                    y: top_line.y,
                    left: first_below.left,
                    right: top_line.right,
                };
                let just_below = HorizontalLine {
                    y: first_below.y,
                    left: top_line.left,
                    right: first_below.left,
                };
                let prev = top_line.clone();
                top_line.right = first_below.left - 1;
                println!(
                    "Splitting {:?} into {:?}, {:?}, and {:?}",
                    prev, top_line, right, just_below
                );

                segments_to_consider.push(just_below);
                segments_to_consider.push(right);
            } else if first_below.right < top_line.right {
                assert!(first_below.right > top_line.right);
                let other = HorizontalLine {
                    y: top_line.y,
                    left: first_below.right + 1,
                    right: top_line.right,
                };
                let prev = top_line.clone();
                top_line.right = first_below.right - 1;
                println!("Splitting {:?} into {:?} and {:?}", prev, top_line, other);

                segments_to_consider.push(other);
            }

            rectangles.push(Rectangle {
                top_left: Coord {
                    x: top_line.left,
                    y: top_line.y,
                },
                bottom_right: Coord {
                    x: top_line.right,
                    y: first_below.y,
                },
            });
        }
        rectangles
    }

    fn draw_rectangles(&self, rectangles: &[Rectangle]) {
        let mut grid = vec![vec![b'.'; self.width]; self.height];

        let mut assign = |y: isize, x: isize, ch: u8| {
            let y: usize = y.try_into().unwrap();
            let x: usize = x.try_into().unwrap();
            if grid[y][x] != b'.' {
                println!(
                    "Coordinate ({},{}) is double-assigned. Was {}",
                    x,
                    y,
                    char::from_u32(grid[y][x] as u32).unwrap()
                );
                grid[y][x] = b'*';
            } else {
                grid[y][x] = ch;
            }
        };

        for Rectangle {
            top_left,
            bottom_right,
        } in rectangles
        {
            assign(top_left.y, top_left.x, b'+');
            assign(top_left.y, bottom_right.x, b'+');
            assign(bottom_right.y, top_left.x, b'+');
            assign(bottom_right.y, bottom_right.x, b'+');
            for x in top_left.x + 1..bottom_right.x {
                assign(top_left.y, x, b'-');
                assign(bottom_right.y, x, b'-');
            }
            for y in top_left.y + 1..bottom_right.y {
                assign(y, top_left.x, b'|');
                assign(y, bottom_right.x, b'|');
            }
        }

        for line in grid {
            println!("{}", std::str::from_utf8(&line).unwrap());
        }
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
    println!("{directions}");
    let rectangles = directions.get_enclosed_rectangles();
    for r in rectangles.windows(1) {
        println!();
        directions.draw_rectangles(r);
    }
    directions.draw_rectangles(&rectangles);
    println!("{:?}", rectangles);

    rectangles.iter().map(Rectangle::area).sum()
}

#[test]
fn test_part1() {
    // assert_eq!(part1("D 10000\nR 10\nU 10000\nL 10"), 10001 * 11);

    assert_eq!(part1("D 8\nR 4\nU 2\nL 1\nU 3\nR 1\nU 3\nL 4"), 5 * 9 - 2);

    // assert_eq!(part1(TEST_INPUT), 62);
}

fn part2(input: &str) -> u64 {
    let directions = Directions::from_part2_str(input);
    let rectangles = directions.get_enclosed_rectangles();

    rectangles.iter().map(Rectangle::area).sum()
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
