#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
use std::{
    collections::VecDeque,
    fmt::Write,
    ops::{Add, Index, IndexMut},
    str::FromStr,
};

use Dir::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Coord {
    x: isize,
    y: isize,
}

impl Add<Dir> for Coord {
    type Output = Coord;

    fn add(self, rhs: Dir) -> Self::Output {
        match rhs {
            Up => Self {
                x: self.x,
                y: self.y - 1,
            },
            Down => Self {
                x: self.x,
                y: self.y + 1,
            },
            Right => Self {
                x: self.x + 1,
                y: self.y,
            },
            Left => Self {
                x: self.x - 1,
                y: self.y,
            },
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

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Square {
    Edge { color: String },
    Inside,
    Outside,
}

struct Lake {
    squares: VecDeque<VecDeque<Square>>,
    x_off: isize,
    y_off: isize,
}

impl FromStr for Lake {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Lake::from_directions(s.lines().map(|l| {
            let strs: [&str; 3] = l
                .split_whitespace()
                .collect::<Vec<&str>>()
                .try_into()
                .unwrap();
            let dir: Dir = strs[0].parse().unwrap();
            let len: u64 = strs[1].parse().unwrap();
            let color = &strs[2][1..strs[2].len() - 1];
            (dir, len, color)
        })))
    }
}

impl Lake {
    fn new() -> Self {
        Self {
            squares: VecDeque::new(),
            x_off: 0,
            y_off: 0,
        }
    }

    fn from_directions<'a>(it: impl Iterator<Item = (Dir, u64, &'a str)>) -> Self {
        let mut lake = Lake::new();
        let mut current_coord = Coord { x: 0, y: 0 };

        for (dir, len, color) in it {
            for _ in 0..len {
                current_coord = current_coord + dir;
                lake.make_room_for_coord(current_coord);
                assert_eq!(lake[current_coord], Square::Outside);
                lake[current_coord] = Square::Edge {
                    color: color.to_string(),
                }
            }
        }

        lake
    }

    fn height(&self) -> isize {
        self.squares.len() as isize
    }
    fn width(&self) -> isize {
        if self.squares.is_empty() {
            0
        } else {
            self.squares[0].len() as isize
        }
    }

    fn make_room_for_coord(&mut self, coord: Coord) {
        let mut new_row = VecDeque::new();
        new_row.resize(self.width() as usize, Square::Outside);
        while coord.y + self.y_off < 0 {
            self.y_off += 1;
            self.squares.push_front(new_row.clone());
        }

        while coord.x + self.x_off < 0 {
            self.x_off += 1;
            for line in self.squares.iter_mut() {
                line.push_front(Square::Outside);
            }
        }

        let necessary_width = coord.x + self.x_off;
        let necessary_height = coord.y + self.y_off;
        if necessary_height >= self.height() {
            self.squares.resize(necessary_height as usize + 1, new_row);
        }
        if necessary_width >= self.width() {
            for line in self.squares.iter_mut() {
                line.resize(necessary_width as usize + 1, Square::Outside);
            }
        }
    }

    fn is_inside(&self, x: usize, y: usize) -> bool {
        if matches!(self.squares[y][x], Square::Edge { .. }) {
            return false;
        }
        if x <= 0 {
            return false;
        }
        let mut seems_inside = false;
        for test_y in 0..y {
            if matches!(self.squares[test_y][x], Square::Edge { .. })
                && matches!(self.squares[test_y][x - 1], Square::Edge { .. })
            {
                seems_inside = !seems_inside;
            }
        }

        seems_inside
    }

    fn dig_interior(&mut self) {
        for y in 0..self.height() as usize {
            for x in 0..self.width() as usize {
                if self.is_inside(x, y) {
                    self.squares[y][x] = Square::Inside;
                }
            }
        }
    }

    fn count_edge_or_inside(&self) -> usize {
        (0..self.height() as usize)
            .map(|y| {
                (0..self.width() as usize)
                    .map(|x| match self.squares[y][x] {
                        Square::Edge { .. } | Square::Inside => 1,
                        Square::Outside => 0,
                    })
                    .sum::<usize>()
            })
            .sum()
    }
}

#[test]
fn test_make_room_for_coord() {
    let mut lake = Lake::new();
    lake.make_room_for_coord(Coord { x: 0, y: 0 });
    assert_eq!(lake.width(), 1);
    assert_eq!(lake.height(), 1);
    assert_eq!(lake.x_off, 0);
    assert_eq!(lake.y_off, 0);

    let mut lake = Lake::new();
    lake.make_room_for_coord(Coord { x: -1, y: -1 });
    assert_eq!(lake.width(), 1);
    assert_eq!(lake.height(), 1);
    assert_eq!(lake.x_off, 1);
    assert_eq!(lake.y_off, 1);

    let mut lake = Lake::new();
    lake.make_room_for_coord(Coord { x: -50, y: -20 });
    assert_eq!(lake.width(), 50);
    assert_eq!(lake.height(), 20);
    assert_eq!(lake.x_off, 50);
    assert_eq!(lake.y_off, 20);

    lake.make_room_for_coord(Coord { x: 20, y: 10 });
    assert_eq!(lake.width(), 71);
    assert_eq!(lake.height(), 31);
    assert_eq!(lake.x_off, 50);
    assert_eq!(lake.y_off, 20);
}

#[test]
fn test_from_directions() {
    let lake = Lake::from_directions([(Left, 2, "")].into_iter());
    assert_eq!(lake.width(), 2);
    assert_eq!(lake.height(), 1);
    assert_eq!(lake.x_off, 2);
    assert_eq!(lake.y_off, 0);
}

impl Index<Coord> for Lake {
    type Output = Square;

    fn index(&self, index: Coord) -> &Self::Output {
        &self.squares[(index.y + self.y_off) as usize][(index.x + self.x_off) as usize]
    }
}

impl IndexMut<Coord> for Lake {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        &mut self.squares[(index.y + self.y_off) as usize][(index.x + self.x_off) as usize]
    }
}

impl std::fmt::Display for Lake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // output 0 x point
        for _ in 0..self.x_off + 1 {
            f.write_char(' ')?;
        }
        f.write_str("0\n")?;
        for (y, row) in self.squares.iter().enumerate() {
            f.write_char(if y as isize == self.y_off { '0' } else { ' ' })?;
            for sq in row.iter() {
                f.write_char(match sq {
                    Square::Edge { .. } => '#',
                    Square::Inside => '@',
                    Square::Outside => '.',
                })?;
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

fn part1(input: &str) -> usize {
    let mut lake: Lake = input.parse().unwrap();
    println!("before digging: \n{}", lake);
    lake.dig_interior();
    println!("after digging: \n{}", lake);
    lake.count_edge_or_inside()
}

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 62);
}

fn main() {
    let input = &std::fs::read_to_string("input.txt").expect("input.txt should exist");
    println!("part 1: {}", part1(input));
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
