use std::{
    collections::HashSet,
    fmt::{Display, Write},
    ops::Add,
    ops::Index,
    ops::IndexMut,
    str::FromStr,
};

struct Grid {
    tiles: Vec<String>,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum Dir {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Coord {
    x: isize,
    y: isize,
}

impl Add<Dir> for Coord {
    type Output = Coord;

    fn add(self, rhs: Dir) -> Self::Output {
        match rhs {
            Dir::North => Self {
                x: self.x,
                y: self.y - 1,
            },
            Dir::South => Self {
                x: self.x,
                y: self.y + 1,
            },
            Dir::East => Self {
                x: self.x + 1,
                y: self.y,
            },
            Dir::West => Self {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

impl Index<&Coord> for Grid {
    type Output = u8;

    fn index(&self, index: &Coord) -> &Self::Output {
        &self.tiles[index.y as usize].as_bytes()[index.x as usize]
    }
}

#[derive(Default)]
struct EnergizedMap(Vec<Vec<HashSet<Dir>>>);

impl Index<&Coord> for EnergizedMap {
    type Output = HashSet<Dir>;

    fn index(&self, index: &Coord) -> &Self::Output {
        &self.0[index.y as usize][index.x as usize]
    }
}

impl IndexMut<&Coord> for EnergizedMap {
    fn index_mut(&mut self, index: &Coord) -> &mut Self::Output {
        &mut self.0[index.y as usize][index.x as usize]
    }
}

impl EnergizedMap {
    fn count(&self) -> usize {
        self.0
            .iter()
            .map(|l| {
                l.iter()
                    .map(|t| if t.is_empty() { 0 } else { 1 })
                    .sum::<usize>()
            })
            .sum()
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.tiles.iter() {
            f.write_str(line)?;
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Display for EnergizedMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.0.iter() {
            for tile in line.iter() {
                f.write_char(match tile.len() {
                    0 => '.',
                    1 => match tile.iter().next().unwrap() {
                        Dir::North => '^',
                        Dir::South => 'v',
                        Dir::East => '>',
                        Dir::West => '<',
                    },
                    0..=9 => format!("{}", tile.len()).chars().next().unwrap(),
                    _ => '@',
                })?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            tiles: s.lines().map(|l| l.to_string()).collect(),
        })
    }
}

impl Grid {
    fn height(&self) -> usize {
        self.tiles.len()
    }
    fn width(&self) -> usize {
        self.tiles[0].len()
    }
    fn is_in_bounds(&self, coord: &Coord) -> bool {
        coord.x >= 0
            && coord.y >= 0
            && (coord.x as usize) < self.width()
            && (coord.y as usize) < self.height()
    }

    fn get_energized_map(&self, start_coord: Coord, start_dir: Dir) -> EnergizedMap {
        use Dir::*;
        let mut energized = EnergizedMap(vec![vec![HashSet::new(); self.width()]; self.height()]);
        let mut modified_tiles = vec![(start_coord, start_dir)];

        while let Some((prev, dir)) = modified_tiles.pop() {
            let cur = prev + dir;
            if !self.is_in_bounds(&cur) {
                continue;
            }
            // Insert incoming direction into the set. If it was already there, don't do anything else.
            if !energized[&cur].insert(dir) {
                continue;
            }
            // Push the next directions to check
            match self[&cur] {
                b'.' => modified_tiles.push((cur, dir)),
                b'/' => modified_tiles.push((
                    cur,
                    match dir {
                        North => East,
                        South => West,
                        East => North,
                        West => South,
                    },
                )),
                b'\\' => modified_tiles.push((
                    cur,
                    match dir {
                        North => West,
                        South => East,
                        East => South,
                        West => North,
                    },
                )),
                b'|' => match dir {
                    North | South => modified_tiles.push((cur, dir)),
                    East | West => {
                        modified_tiles.push((cur, North));
                        modified_tiles.push((cur, South));
                    }
                },
                b'-' => match dir {
                    East | West => modified_tiles.push((cur, dir)),
                    North | South => {
                        modified_tiles.push((cur, East));
                        modified_tiles.push((cur, West));
                    }
                },
                ch => panic!("Unexpected grid element {}", ch),
            }
        }
        energized
    }
}

const TEST_INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

#[test]
fn test_count_energized() {
    let grid = TEST_INPUT.parse::<Grid>().unwrap();

    println!("{}", grid);

    let energized = grid.get_energized_map(Coord { x: -1, y: 0 }, Dir::East);
    println!("{}", energized);
    assert_eq!(energized.count(), 46);
}

fn part1(input: &str) -> usize {
    let grid: Grid = input.parse().unwrap();
    grid.get_energized_map(Coord { x: -1, y: 0 }, Dir::East)
        .count()
}

fn part2(input: &str) -> usize {
    let grid: Grid = input.parse().unwrap();

    (0..grid.width() as isize)
        .flat_map(|i| {
            [
                (
                    Coord {
                        x: i,
                        y: grid.height() as isize,
                    },
                    Dir::North,
                ),
                (Coord { x: i, y: -1 }, Dir::South),
            ]
        })
        .chain((0..grid.height() as isize).flat_map(|i| {
            [
                (
                    Coord {
                        x: grid.width() as isize,
                        y: i,
                    },
                    Dir::West,
                ),
                (Coord { x: -1, y: i }, Dir::East),
            ]
        }))
        .map(|(coord, dir)| grid.get_energized_map(coord, dir).count())
        .max()
        .unwrap()
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 51);
}

fn main() {
    let input = include_str!("input.txt");
    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input))
}
