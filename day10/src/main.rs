use std::{
    ops::{Add, Sub},
    str::FromStr,
};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum AocError {
    InvalidPuzzleChar(char),
    NoStart,
    PipeEnded,
    PipeWentOffEdge,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum PipePiece {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Coord(isize, isize);

impl Add for Coord {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl Sub for Coord {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Coord(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl PipePiece {
    fn follow(self, pos: Coord, from: Coord) -> Option<Coord> {
        use PipePiece::*;
        match (self, from - pos) {
            (NS, Coord(0, -1)) => Some(pos + Coord(0, 1)),
            (NS, Coord(0, 1)) => Some(pos + Coord(0, -1)),
            (EW, Coord(-1, 0)) => Some(pos + Coord(1, 0)),
            (EW, Coord(1, 0)) => Some(pos + Coord(-1, 0)),
            (NE, Coord(1, 0)) => Some(pos + Coord(0, -1)),
            (NE, Coord(0, -1)) => Some(pos + Coord(1, 0)),
            (NW, Coord(-1, 0)) => Some(pos + Coord(0, -1)),
            (NW, Coord(0, -1)) => Some(pos + Coord(-1, 0)),
            (SW, Coord(0, 1)) => Some(pos + Coord(-1, 0)),
            (SW, Coord(-1, 0)) => Some(pos + Coord(0, 1)),
            (SE, Coord(1, 0)) => Some(pos + Coord(0, 1)),
            (SE, Coord(0, 1)) => Some(pos + Coord(1, 0)),
            _ => None,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Puzzle(Vec<Vec<PipePiece>>);

impl FromStr for Puzzle {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PipePiece::*;
        Ok(Puzzle(
            s.lines()
                .map(|l| {
                    l.chars()
                        .map(|ch| match ch {
                            '|' => Ok(NS),
                            '-' => Ok(EW),
                            'L' => Ok(NE),
                            'J' => Ok(NW),
                            '7' => Ok(SW),
                            'F' => Ok(SE),
                            '.' => Ok(Ground),
                            'S' => Ok(Start),
                            other => Err(AocError::InvalidPuzzleChar(other)),
                        })
                        .collect()
                })
                .collect::<Result<Vec<Vec<PipePiece>>, AocError>>()?,
        ))
    }
}

impl Puzzle {
    fn get(&self, index: Coord) -> Option<&PipePiece> {
        if index.0 < 0 || index.1 < 0 {
            return None;
        }
        self.0
            .get(index.1 as usize)
            .and_then(|row| row.get(index.0 as usize))
    }

    fn find_loop_length(&self) -> Result<u32, AocError> {
        use PipePiece::*;

        let start_pos = self
            .0
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                Some(Coord(
                    row.iter().position(|pp| *pp == Start)? as isize,
                    y as isize,
                ))
            })
            .ok_or(AocError::NoStart)?;

        // Find first direction
        let dirs_to_try = [
            (Coord(-1, 0), [EW, NE, SE]),
            (Coord(0, -1), [NS, SW, SE]),
            (Coord(0, 1), [EW, NW, SW]),
            (Coord(1, 0), [EW, NW, SW]),
        ];
        let mut second_pos = None;
        'outer: for (diff, ok_pipes) in dirs_to_try {
            if let Some(ch) = self.get(start_pos + diff) {
                if ok_pipes.contains(ch) {
                    second_pos = Some(start_pos + diff);
                    break 'outer;
                }
            }
        }

        let mut length = 1;
        let mut cur_pos = second_pos.unwrap();
        let mut prev_pos = start_pos;

        // Follow around
        while cur_pos != start_pos {
            let next = self
                .get(cur_pos)
                .ok_or(AocError::PipeWentOffEdge)?
                .follow(cur_pos, prev_pos)
                .ok_or(AocError::PipeEnded)?;
            prev_pos = cur_pos;
            cur_pos = next;

            length += 1;
        }

        Ok(length)
    }
}

fn part1(input: &str) -> Result<u32, AocError> {
    Ok((input.parse::<Puzzle>()?.find_loop_length()? + 1) / 2)
}

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT1), Ok(4));
    assert_eq!(part1(TEST_INPUT2), Ok(8));
}

fn main() {
    println!("part 1: {:?}", part1(include_str!("real_input.txt")));
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
