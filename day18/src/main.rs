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
struct UnrootedLine {
    dir: Dir,
    len: i64,
}

#[derive(Debug, Clone)]
struct BoundaryLine {
    left: i64,
    right: i64,
    top: i64,
    bottom: i64,
}

impl BoundaryLine {
    fn is_horizontal(&self) -> bool {
        self.top == self.bottom
    }
}

#[derive(Debug)]
struct Map {
    boundaries: Vec<BoundaryLine>,
    bounds: Rectangle,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Rectangle {
    left: i64,
    right: i64,
    top: i64,
    bottom: i64,
}

impl Rectangle {
    fn area(&self) -> i64 {
        self.width() * self.height()
    }

    fn is_valid(&self) -> bool {
        self.left <= self.right && self.top <= self.bottom
    }

    fn width(&self) -> i64 {
        assert!(self.is_valid());
        self.right - self.left + 1
    }
    fn height(&self) -> i64 {
        assert!(self.is_valid());
        self.bottom - self.top + 1
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Map {} x {} {:?}",
            self.bounds.width(),
            self.bounds.height(),
            self.bounds
        )?;
        let mut grid = vec![vec!['.'; self.bounds.width() as usize]; self.bounds.height() as usize];
        for BoundaryLine {
            left,
            right,
            top,
            bottom,
        } in self.boundaries.iter()
        {
            for y in *top..=*bottom {
                for x in *left..=*right {
                    grid[(y - self.bounds.top) as usize][(x - self.bounds.left) as usize] = '#';
                }
            }
        }

        for line in grid {
            writeln!(f, "{}", String::from_iter(&line))?;
        }

        Ok(())
    }
}

impl Map {
    fn from_part1_str(input: &str) -> Self {
        Self::from_lines(input.lines().map(|l| {
            let mut split = l.split_whitespace();
            let dir = split.next().unwrap().parse().unwrap();
            let len = split.next().unwrap().parse().unwrap();
            UnrootedLine { dir, len }
        }))
    }
    fn from_part2_str(input: &str) -> Self {
        Self::from_lines(input.lines().map(|line| {
            let dir_code = line.as_bytes()[line.len() - 2];
            let len = i64::from_str_radix(&line[line.len() - 7..line.len() - 2], 16).unwrap();
            let dir = match dir_code {
                b'0' => Right,
                b'1' => Down,
                b'2' => Left,
                b'3' => Up,
                _ => panic!("Invalid dir code {}", dir_code),
            };
            UnrootedLine { dir, len }
        }))
    }

    fn from_lines(lines: impl Iterator<Item = UnrootedLine>) -> Self {
        let mut current_coord = Coord { x: 0, y: 0 };
        let mut max_x = 0;
        let mut min_x = 0;
        let mut max_y = 0;
        let mut min_y = 0;

        let boundaries: Vec<BoundaryLine> = lines
            .filter_map(|UnrootedLine { dir, len }| {
                let prev_coord = current_coord.clone();
                current_coord += dir.to_coord() * len;

                min_x = std::cmp::min(min_x, current_coord.x);
                min_y = std::cmp::min(min_y, current_coord.y);
                max_x = std::cmp::max(max_x, current_coord.x);
                max_y = std::cmp::max(max_y, current_coord.y);

                Some(BoundaryLine {
                    left: std::cmp::min(prev_coord.x, current_coord.x),
                    right: std::cmp::max(prev_coord.x, current_coord.x),
                    top: std::cmp::min(prev_coord.y, current_coord.y),
                    bottom: std::cmp::max(prev_coord.y, current_coord.y),
                })
            })
            .collect();

        assert_eq!(current_coord, Coord { x: 0, y: 0 });

        Self {
            boundaries,
            bounds: Rectangle {
                left: min_x,
                right: max_x,
                top: min_y,
                bottom: max_y,
            },
        }
    }

    fn get_possible_rectangles(&self) -> Vec<Rectangle> {
        let mut rectangles = vec![self.bounds.clone()];

        for boundary in self.boundaries.iter() {
            rectangles = rectangles
                .into_iter()
                .flat_map(|orig| {
                    // Optimization: if orig does not at all intersect boundary, do not split
                    if !(orig.left <= boundary.right
                        && orig.right >= boundary.left
                        && orig.top <= boundary.bottom
                        && orig.bottom >= boundary.top)
                    {
                        return vec![orig];
                    }
                    let split_candidates = if boundary.is_horizontal() {
                        [
                            // Above the line
                            Rectangle {
                                bottom: boundary.top - 1,
                                ..orig
                            },
                            // Overlapping the line
                            Rectangle {
                                top: boundary.top,
                                bottom: boundary.bottom,
                                ..orig
                            },
                            // Below the line
                            Rectangle {
                                top: boundary.bottom + 1,
                                ..orig
                            },
                        ]
                    } else {
                        [
                            // Left of the line
                            Rectangle {
                                right: boundary.left - 1,
                                ..orig
                            },
                            // Overlapping the line
                            Rectangle {
                                left: boundary.left,
                                right: boundary.right,
                                ..orig
                            },
                            // Right of line
                            Rectangle {
                                left: boundary.right + 1,
                                ..orig
                            },
                        ]
                    };

                    split_candidates
                        .into_iter()
                        .map(|r| Rectangle {
                            left: std::cmp::max(r.left, orig.left),
                            top: std::cmp::max(r.top, orig.top),
                            right: std::cmp::min(r.right, orig.right),
                            bottom: std::cmp::min(r.bottom, orig.bottom),
                        })
                        .filter(|r| r.is_valid())
                        .collect()
                })
                .filter(Rectangle::is_valid)
                .collect()
        }

        rectangles
    }

    fn is_point_on_boundary(&self, coord: &Coord) -> bool {
        for boundary in self.boundaries.iter() {
            if coord.x >= boundary.left
                && coord.x <= boundary.right
                && coord.y >= boundary.top
                && coord.y <= boundary.bottom
            {
                return true;
            }
        }
        false
    }

    fn is_point_enclosed(&self, coord: &Coord) -> bool {
        if self.is_point_on_boundary(coord) {
            return true;
        }

        let mut seems_enclosed = false;

        for boundary in self.boundaries.iter() {
            if boundary.is_horizontal()
                && boundary.left <= coord.x
                && boundary.right > coord.x
                && boundary.top <= coord.y
            {
                assert!(boundary.top != coord.y);
                seems_enclosed = !seems_enclosed;
            }
        }
        seems_enclosed
    }

    fn get_enclosed_rectangles(&self) -> Vec<Rectangle> {
        self.get_possible_rectangles()
            .into_iter()
            .filter(|r| {
                self.is_point_enclosed(&Coord {
                    x: r.left,
                    y: r.top,
                })
            })
            .collect()
    }

    fn draw_rectangles(&self, rectangles: &[Rectangle]) {
        let mut grid = vec![vec!['.'; self.bounds.width() as usize]; self.bounds.height() as usize];

        let mut assign = |y: i64, x: i64, ch: char| {
            let y: usize = (y - self.bounds.top).try_into().unwrap();
            let x: usize = (x - self.bounds.left).try_into().unwrap();
            if grid[y][x] != '.' {
                println!(
                    "Coordinate ({},{}) is double-assigned. Was {}",
                    x, y, grid[y][x]
                );
                grid[y][x] = 'x';
            } else {
                grid[y][x] = ch;
            }
        };

        for Rectangle {
            left,
            right,
            top,
            bottom,
        } in rectangles.iter().cloned()
        {
            let width_1 = left == right;
            let height_1 = top == bottom;

            if width_1 && height_1 {
                assign(top, right, '▫');
            } else if width_1 {
                assign(top, left, '╓');
                for y in top + 1..bottom {
                    assign(y, left, '║');
                }
                assign(bottom, right, '╙');
            } else if height_1 {
                assign(top, left, '╘');
                for x in left + 1..right {
                    assign(top, x, '═');
                }
                assign(bottom, right, '╛');
            } else {
                assign(top, left, '┌');
                assign(top, right, '┐');
                assign(bottom, left, '└');
                assign(bottom, right, '┘');
                for x in left + 1..right {
                    assign(top, x, '─');
                    assign(bottom, x, '─');
                }
                for y in top + 1..bottom {
                    assign(y, left, '│');
                    assign(y, right, '│');
                }

                for x in left + 1..right {
                    for y in top + 1..bottom {
                        assign(y, x, '█');
                    }
                }
            }
        }

        for line in grid {
            println!("{}", String::from_iter(&line));
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Coord {
    x: i64,
    y: i64,
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

impl Mul<i64> for Coord {
    type Output = Coord;

    fn mul(self, rhs: i64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

fn part1(input: &str) -> i64 {
    let map = Map::from_part1_str(input);

    println!("{map}");
    let possible = map.get_possible_rectangles();
    map.draw_rectangles(&possible);
    let rectangles: Vec<Rectangle> = possible
        .into_iter()
        .filter(|r| {
            map.is_point_enclosed(&Coord {
                x: r.left,
                y: r.top,
            })
        })
        .collect();
    println!("=========");

    map.draw_rectangles(&rectangles);

    rectangles.iter().map(Rectangle::area).sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1("D 10000\nR 10\nU 10000\nL 10"), 10001 * 11);

    assert_eq!(part1("D 8\nR 4\nU 2\nL 1\nU 3\nR 1\nU 3\nL 4"), 5 * 9 - 2);

    assert_eq!(part1(TEST_INPUT), 62);
}

fn part2(input: &str) -> i64 {
    let map = Map::from_part2_str(input);
    let rectangles = map.get_enclosed_rectangles();

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

#[cfg(test)]
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
