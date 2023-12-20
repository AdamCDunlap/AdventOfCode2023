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
    len: isize,
}

#[derive(Debug, Clone)]
struct BoundaryLine {
    top_left: Coord,
    bottom_right: Coord,
}

impl BoundaryLine {
    fn is_horizontal(&self) -> bool {
        self.top_left.y == self.bottom_right.y
    }
}

#[derive(Debug)]
struct Map {
    boundaries: Vec<BoundaryLine>,
    width: usize,
    height: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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

    fn is_valid(&self) -> bool {
        self.top_left.x <= self.bottom_right.x && self.top_left.y <= self.bottom_right.y
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Map {} x {}", self.width, self.height)?;
        let mut grid = vec![vec!['.'; self.width]; self.height];
        for BoundaryLine {
            top_left: Coord { x: left, y: top },
            bottom_right: Coord {
                x: right,
                y: bottom,
            },
        } in self.boundaries.iter()
        {
            for y in *top..=*bottom {
                for x in *left..=*right {
                    grid[y as usize][x as usize] = '#';
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
            let len = isize::from_str_radix(&line[line.len() - 7..line.len() - 2], 16).unwrap();
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

        let mut boundaries: Vec<BoundaryLine> = lines
            .filter_map(|UnrootedLine { dir, len }| {
                let prev_coord = current_coord.clone();
                current_coord += dir.to_coord() * len;

                min_x = std::cmp::min(min_x, current_coord.x);
                min_y = std::cmp::min(min_y, current_coord.y);
                max_x = std::cmp::max(max_x, current_coord.x);
                max_y = std::cmp::max(max_y, current_coord.y);

                let top_left = Coord {
                    x: std::cmp::min(prev_coord.x, current_coord.x),
                    y: std::cmp::min(prev_coord.y, current_coord.y),
                };
                let bottom_right = Coord {
                    x: std::cmp::max(prev_coord.x, current_coord.x),
                    y: std::cmp::max(prev_coord.y, current_coord.y),
                };

                Some(BoundaryLine {
                    top_left,
                    bottom_right,
                })
            })
            .collect();

        assert_eq!(current_coord, Coord { x: 0, y: 0 });

        // println!("Orig:  {horizontal_lines:?}");

        // This is probably unnecessary
        boundaries.iter_mut().for_each(
            |BoundaryLine {
                 top_left,
                 bottom_right,
             }| {
                top_left.y -= min_y;
                top_left.x -= min_x;
                bottom_right.y -= min_y;
                bottom_right.x -= min_x;
            },
        );
        // println!("Fixed: {horizontal_lines:?}\nmin_x: {min_x} min_y: {min_y}");

        Self {
            boundaries,
            width: (max_x - min_x + 1) as usize,
            height: (max_y - min_y + 1) as usize,
        }
    }

    fn get_possible_rectangles(&self) -> Vec<Rectangle> {
        let mut rectangles = vec![Rectangle {
            top_left: Coord { x: 0, y: 0 },
            bottom_right: Coord {
                x: self.width as isize - 1,
                y: self.height as isize - 1,
            },
        }];

        for boundary in self.boundaries.iter() {
            rectangles = rectangles
                .into_iter()
                .flat_map(|orig| {
                    // Optimization: if orig does not at all intersect boundary, do not split
                    if !(orig.top_left.x <= boundary.bottom_right.x
                        && orig.bottom_right.x >= boundary.top_left.x
                        && orig.top_left.y <= boundary.bottom_right.y
                        && orig.bottom_right.y >= boundary.top_left.y)
                    {
                        return vec![orig];
                    }
                    let split_candidates = if boundary.is_horizontal() {
                        [
                            // Above the line
                            Rectangle {
                                top_left: orig.top_left,
                                bottom_right: Coord {
                                    x: orig.bottom_right.x,
                                    y: boundary.top_left.y - 1,
                                },
                            },
                            // Overlapping the line
                            Rectangle {
                                top_left: Coord {
                                    x: orig.top_left.x,
                                    y: boundary.top_left.y,
                                },
                                bottom_right: Coord {
                                    x: orig.bottom_right.x,
                                    y: boundary.bottom_right.y,
                                },
                            },
                            // Below the line
                            Rectangle {
                                top_left: Coord {
                                    x: orig.top_left.x,
                                    y: boundary.bottom_right.y + 1,
                                },
                                bottom_right: orig.bottom_right,
                            },
                        ]
                    } else {
                        [
                            // Left of the line
                            Rectangle {
                                top_left: orig.top_left,
                                bottom_right: Coord {
                                    x: boundary.top_left.x - 1,
                                    y: orig.bottom_right.y,
                                },
                            },
                            // Overlapping the line
                            Rectangle {
                                top_left: Coord {
                                    x: boundary.top_left.x,
                                    y: orig.top_left.y,
                                },
                                bottom_right: Coord {
                                    x: boundary.bottom_right.x,
                                    y: orig.bottom_right.y,
                                },
                            },
                            // Right of line
                            Rectangle {
                                top_left: Coord {
                                    x: boundary.bottom_right.x + 1,
                                    y: orig.top_left.y,
                                },
                                bottom_right: orig.bottom_right,
                            },
                        ]
                    };

                    split_candidates
                        .into_iter()
                        .map(|r| Rectangle {
                            top_left: Coord {
                                x: std::cmp::max(r.top_left.x, orig.top_left.x),
                                y: std::cmp::max(r.top_left.y, orig.top_left.y),
                            },
                            bottom_right: Coord {
                                x: std::cmp::min(r.bottom_right.x, orig.bottom_right.x),
                                y: std::cmp::min(r.bottom_right.y, orig.bottom_right.y),
                            },
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
            if coord.x >= boundary.top_left.x
                && coord.x <= boundary.bottom_right.x
                && coord.y >= boundary.top_left.y
                && coord.y <= boundary.bottom_right.y
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
                && boundary.top_left.x <= coord.x
                && boundary.bottom_right.x > coord.x
                && boundary.top_left.y <= coord.y
            {
                assert!(boundary.top_left.y != coord.y);
                seems_enclosed = !seems_enclosed;
            }
        }
        seems_enclosed
    }

    fn get_enclosed_rectangles(&self) -> Vec<Rectangle> {
        self.get_possible_rectangles()
            .into_iter()
            .filter(|r| self.is_point_enclosed(&r.top_left))
            .collect()
    }

    fn draw_rectangles(&self, rectangles: &[Rectangle]) {
        let mut grid = vec![vec!['.'; self.width]; self.height];

        let mut assign = |y: isize, x: isize, ch: char| {
            let y: usize = y.try_into().unwrap();
            let x: usize = x.try_into().unwrap();
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
            top_left,
            bottom_right,
        } in rectangles
        {
            let width_1 = top_left.x == bottom_right.x;
            let height_1 = top_left.y == bottom_right.y;

            if width_1 && height_1 {
                assign(top_left.y, bottom_right.x, '▫');
            } else if width_1 {
                assign(top_left.y, top_left.x, '╓');
                for y in top_left.y + 1..bottom_right.y {
                    assign(y, top_left.x, '║');
                }
                assign(bottom_right.y, bottom_right.x, '╙');
            } else if height_1 {
                assign(top_left.y, top_left.x, '╘');
                for x in top_left.x + 1..bottom_right.x {
                    assign(top_left.y, x, '═');
                }
                assign(bottom_right.y, bottom_right.x, '╛');
            } else {
                assign(top_left.y, top_left.x, '┌');
                assign(top_left.y, bottom_right.x, '┐');
                assign(bottom_right.y, top_left.x, '└');
                assign(bottom_right.y, bottom_right.x, '┘');
                for x in top_left.x + 1..bottom_right.x {
                    assign(top_left.y, x, '─');
                    assign(bottom_right.y, x, '─');
                }
                for y in top_left.y + 1..bottom_right.y {
                    assign(y, top_left.x, '│');
                    assign(y, bottom_right.x, '│');
                }

                for x in top_left.x + 1..bottom_right.x {
                    for y in top_left.y + 1..bottom_right.y {
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
    let map = Map::from_part1_str(input);

    println!("{map}");
    let possible = map.get_possible_rectangles();
    map.draw_rectangles(&possible);
    let rectangles: Vec<Rectangle> = possible
        .into_iter()
        .filter(|r| map.is_point_enclosed(&r.top_left))
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

fn part2(input: &str) -> u64 {
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
