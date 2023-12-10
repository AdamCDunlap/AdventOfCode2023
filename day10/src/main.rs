use std::{ops::Add, str::FromStr};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum AocError {
    InvalidPuzzleChar(char),
    NoStart,
    StartDoesntConnect,
    PipeWentOffEdge,
    PipeHitNonPipe,
    PipeHitNonconnectingPipe,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    fn reverse(self) -> Self {
        use Dir::*;
        match self {
            North => South,
            South => North,
            East => West,
            West => East,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Square {
    Pipe(Dir, Dir),
    Ground,
    InsideLoop,
    OutsideLoop,
    Start,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Coord(isize, isize);

impl Add<Dir> for Coord {
    type Output = Self;
    fn add(self, rhs: Dir) -> Self::Output {
        use Dir::*;
        match rhs {
            North => Coord(self.0, self.1 - 1),
            South => Coord(self.0, self.1 + 1),
            East => Coord(self.0 + 1, self.1),
            West => Coord(self.0 - 1, self.1),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Puzzle(Vec<Vec<Square>>);

impl FromStr for Puzzle {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Dir::*;
        use Square::*;
        Ok(Puzzle(
            s.lines()
                .map(|l| {
                    l.chars()
                        .map(|ch| match ch {
                            '|' => Ok(Pipe(North, South)),
                            '-' => Ok(Pipe(East, West)),
                            'L' => Ok(Pipe(North, East)),
                            'J' => Ok(Pipe(North, West)),
                            '7' => Ok(Pipe(South, West)),
                            'F' => Ok(Pipe(South, East)),
                            '.' => Ok(Ground),
                            'S' => Ok(Start),
                            'I' => Ok(InsideLoop),
                            'O' => Ok(OutsideLoop),
                            other => Err(AocError::InvalidPuzzleChar(other)),
                        })
                        .collect()
                })
                .collect::<Result<Vec<Vec<Square>>, AocError>>()?,
        ))
    }
}

impl Puzzle {
    fn get(&self, index: Coord) -> Option<&Square> {
        if index.0 < 0 || index.1 < 0 {
            return None;
        }
        self.0
            .get(index.1 as usize)
            .and_then(|row| row.get(index.0 as usize))
    }

    fn find_pipe_loop(&self) -> Result<Pipe, AocError> {
        let start_pos = self
            .0
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                Some(Coord(
                    row.iter().position(|pp| *pp == Square::Start)? as isize,
                    y as isize,
                ))
            })
            .ok_or(AocError::NoStart)?;

        let mut start_dirs = vec![];
        for dir in [Dir::North, Dir::South, Dir::East, Dir::West] {
            if let Some(Square::Pipe(other_d1, other_d2)) = self.get(start_pos + dir) {
                if *other_d1 == dir.reverse() || *other_d2 == dir.reverse() {
                    start_dirs.push(dir);
                }
            }
        }

        let start_dirs = start_dirs;

        if start_dirs.len() != 2 {
            return Err(AocError::StartDoesntConnect);
        }

        let mut cur_dir = start_dirs[0];
        let mut path = vec![start_pos];
        let mut cur_pos: Coord = start_pos;

        loop {
            cur_pos = cur_pos + cur_dir;
            if cur_pos == start_pos {
                break;
            }
            path.push(cur_pos);
            let cur_sq = self.get(cur_pos).ok_or(AocError::PipeWentOffEdge)?;
            let Square::Pipe(d1, d2) = cur_sq else {
                return Err(AocError::PipeHitNonPipe);
            };
            if *d1 == cur_dir.reverse() {
                cur_dir = *d2;
            } else if *d2 == cur_dir.reverse() {
                cur_dir = *d1;
            } else {
                return Err(AocError::PipeHitNonconnectingPipe);
            }
        }

        Ok(Pipe {
            puzzle: &self,
            path,
            start_dirs: (start_dirs[0], start_dirs[1]),
        })
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Pipe<'a> {
    puzzle: &'a Puzzle,
    path: Vec<Coord>,
    start_dirs: (Dir, Dir),
}

impl<'a> Pipe<'a> {
    fn max_dist(&self) -> usize {
        (self.path.len() + 1) / 2
    }

    fn is_on_path(&self, pt: &Coord) -> bool {
        self.path.contains(pt)
    }

    fn is_point_inside(&self, pt: &Coord) -> bool {
        if self.is_on_path(pt) {
            // Points on the pipe itself are not inside the pipe area
            return false;
        }

        // The way this works is that we start at the top at pt's x coordinate
        // and check every square up to pt's y coordinate. If the square contains
        // a west-facing edge, then we invert seems_inside. Since west coordinates
        // are always the second part of direction tuples, it's simple to check.

        let mut seems_inside = false;
        for y in 0..pt.1 {
            let coord = Coord(pt.0, y);
            if self.is_on_path(&coord) {
                let sq = *self.puzzle.get(coord).unwrap();
                use Dir::*;
                let invert = match sq {
                    Square::Pipe(_, West) => true,
                    Square::Start if self.start_dirs.1 == West => true,
                    _ => false,
                };

                if invert {
                    seems_inside = !seems_inside;
                }
            }
        }
        seems_inside
    }

    fn area(&self) -> usize {
        self.puzzle
            .0
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, _)| self.is_point_inside(&Coord(x as isize, y as isize)) as usize)
                    .sum::<usize>()
            })
            .sum()
    }
}

fn check_is_point_inside(input: &str) {
    let puzzle = input.parse::<Puzzle>().unwrap();
    let pipe = puzzle.find_pipe_loop().unwrap();
    pipe.puzzle.0.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, _)| {
            let coord = Coord(x as isize, y as isize);
            let pp = pipe.puzzle.get(coord).unwrap();
            match *pp {
                Square::InsideLoop => assert!(
                    pipe.is_point_inside(&coord),
                    "Expected {:?} to be inside",
                    coord
                ),
                Square::OutsideLoop => assert!(
                    !pipe.is_point_inside(&coord),
                    "Expected {:?} to be outside",
                    coord
                ),
                Square::Ground => (),
                _ => assert!(
                    !pipe.is_point_inside(&coord),
                    "Expected {:?} to be on the pipe",
                    coord
                ),
            }
        });
    });
}

#[test]
fn test_is_point_inside() {
    // check_is_point_inside(TEST_INPUT5);
    check_is_point_inside(TEST_INPUT6);
}

fn part1(input: &str) -> Result<usize, AocError> {
    Ok(input.parse::<Puzzle>()?.find_pipe_loop()?.max_dist())
}

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT1), Ok(4));
    assert_eq!(part1(TEST_INPUT2), Ok(8));
}

fn part2(input: &str) -> Result<usize, AocError> {
    Ok(input.parse::<Puzzle>()?.find_pipe_loop()?.area())
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT3), Ok(4));
    assert_eq!(part2(TEST_INPUT4), Ok(8));
}

fn main() {
    println!(
        "part 1: {:?}",
        part1(include_str!("real_input.txt")).unwrap()
    );
    println!(
        "part 2: {:?}",
        part2(include_str!("real_input.txt")).unwrap()
    );
}

const TEST_INPUT1: &str = r#"-L|F7
7S-7|
L|7||
-L-J|
L|-JF"#;

const TEST_INPUT2: &str = r#"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"#;

const TEST_INPUT3: &str = r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."#;

const TEST_INPUT4: &str = r#".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."#;

const TEST_INPUT5: &str = r#"...........
.S-------7.
.|F-----7|.
.||OOOOO||.
.||OOOOO||.
.|L-7OF-J|.
.|II|O|II|.
.L--JOL--J.
.....O....."#;

const TEST_INPUT6: &str = r#"OF----7F7F7F7F-7OOOO
O|F--7||||||||FJOOOO
O||OFJ||||||||L7OOOO
FJL7L7LJLJ||LJIL-7OO
L--JOL7IIILJS7F-7L7O
OOOOF-JIIF7FJ|L7L7L7
OOOOL7IF7||L7|IL7L7|
OOOOO|FJLJ|FJ|F7|OLJ
OOOOFJL-7O||O||||OOO
OOOOL---JOLJOLJLJOOO"#;
