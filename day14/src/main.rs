use std::{
    collections::HashMap,
    fmt::{Debug, Display, Write},
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq)]
enum AocError {
    UnknownSquare,
    NotRectangular,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Square {
    Ground,
    Rounded,
    Cube,
}

#[derive(PartialEq, Eq, Clone, Hash)]
struct Grid(Vec<Vec<Square>>);

impl FromStr for Grid {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let squares = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(|ch| {
                        Ok(match ch {
                            'O' => Square::Rounded,
                            '#' => Square::Cube,
                            '.' => Square::Ground,
                            _ => return Err(AocError::UnknownSquare),
                        })
                    })
                    .collect::<Result<Vec<Square>, AocError>>()
            })
            .collect::<Result<Vec<Vec<Square>>, AocError>>()?;

        let width = squares[0].len();
        if squares.iter().any(|line| line.len() != width) {
            return Err(AocError::NotRectangular);
        }
        Ok(Self(squares))
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.0.iter() {
            for col in line.iter() {
                f.write_char(match col {
                    Square::Ground => '.',
                    Square::Cube => '#',
                    Square::Rounded => 'O',
                })?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Grid {
    fn width(&self) -> usize {
        self.0[0].len()
    }
    fn height(&self) -> usize {
        self.0.len()
    }

    fn slide_piece_north(&mut self, row: usize, col: usize) {
        assert_eq!(self.0[row][col], Square::Rounded);

        let stop_point = (0..row)
            .rev()
            .find(|search_row| self.0[*search_row][col] != Square::Ground);
        let new_row = stop_point.map(|r| r + 1).unwrap_or(0);
        self.0[row][col] = Square::Ground;
        self.0[new_row][col] = Square::Rounded;
    }

    fn slide_piece_south(&mut self, row: usize, col: usize) {
        assert_eq!(self.0[row][col], Square::Rounded);

        let stop_point =
            (row + 1..self.height()).find(|search_row| self.0[*search_row][col] != Square::Ground);
        let new_row = stop_point.map(|r| r - 1).unwrap_or(self.height() - 1);
        self.0[row][col] = Square::Ground;
        self.0[new_row][col] = Square::Rounded;
    }

    fn slide_piece_west(&mut self, row: usize, col: usize) {
        assert_eq!(self.0[row][col], Square::Rounded);

        let stop_point = (0..col)
            .rev()
            .find(|search_col| self.0[row][*search_col] != Square::Ground);
        let new_col = stop_point.map(|r| r + 1).unwrap_or(0);
        self.0[row][col] = Square::Ground;
        self.0[row][new_col] = Square::Rounded;
    }

    fn slide_piece_east(&mut self, row: usize, col: usize) {
        assert_eq!(self.0[row][col], Square::Rounded);

        let stop_point =
            (col + 1..self.width()).find(|search_col| self.0[row][*search_col] != Square::Ground);
        let new_col = stop_point.map(|c| c - 1).unwrap_or(self.width() - 1);
        self.0[row][col] = Square::Ground;
        self.0[row][new_col] = Square::Rounded;
    }

    fn slide_north(&mut self) {
        for row in 0..self.height() {
            for col in 0..self.width() {
                if self.0[row][col] == Square::Rounded {
                    self.slide_piece_north(row, col);
                }
            }
        }
    }

    fn slide_south(&mut self) {
        for row in (0..self.height()).rev() {
            for col in 0..self.width() {
                if self.0[row][col] == Square::Rounded {
                    self.slide_piece_south(row, col);
                }
            }
        }
    }

    fn slide_west(&mut self) {
        for row in 0..self.height() {
            for col in 0..self.width() {
                if self.0[row][col] == Square::Rounded {
                    self.slide_piece_west(row, col);
                }
            }
        }
    }

    fn slide_east(&mut self) {
        for row in 0..self.height() {
            for col in (0..self.width()).rev() {
                if self.0[row][col] == Square::Rounded {
                    self.slide_piece_east(row, col);
                }
            }
        }
    }

    fn slide_cycle(&mut self) {
        self.slide_north();
        self.slide_west();
        self.slide_south();
        self.slide_east();
    }

    fn slide_cycle_many(&mut self, iters: usize) {
        // Maps Grids to the iteration on which it was seen
        let mut seen: HashMap<Grid, usize> = HashMap::new();
        for i in 0..iters {
            if let Some(prev_idx) = seen.get(self) {
                // This grid was seen before! It was seen after modifying it prev_idx times and also i times. This means that there is a cycle of length (i-prev_idx).
                let idx_of_result = *prev_idx + (iters - *prev_idx) % (i - *prev_idx);
                println!(
                    "Found a cycle! idxs {} and {} are the same. Returning {}",
                    i, *prev_idx, idx_of_result
                );
                *self = seen
                    .iter()
                    .find_map(|(grid, idx)| {
                        if *idx == idx_of_result {
                            Some(grid)
                        } else {
                            None
                        }
                    })
                    .unwrap()
                    .clone();
                return;
            }
            seen.insert(self.clone(), i);
            self.slide_cycle();
        }
    }

    fn get_north_load(&self) -> usize {
        (0..self.height())
            .map(|row| {
                (0..self.width())
                    .map(|col| match self.0[row][col] {
                        Square::Rounded => self.height() - row,
                        _ => 0,
                    })
                    .sum::<usize>()
            })
            .sum()
    }
}

#[test]
fn test_slide_north() {
    let mut grid: Grid = TEST_STR.parse().unwrap();

    let expected: Grid = r"OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#...."
        .parse()
        .unwrap();

    grid.slide_north();

    assert_eq!(grid, expected);
}

#[test]
fn test_slide_cycle() {
    let mut grid: Grid = TEST_STR.parse().unwrap();

    grid.slide_cycle();
    assert_eq!(
        grid,
        r".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#...."
            .parse()
            .unwrap()
    );
    grid.slide_cycle();
    assert_eq!(
        grid,
        r".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O"
            .parse()
            .unwrap()
    );
    grid.slide_cycle();
    assert_eq!(
        grid,
        r".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O"
            .parse()
            .unwrap()
    );
}

fn part1(input: &str) -> usize {
    let mut grid: Grid = input.parse().unwrap();
    grid.slide_north();
    grid.get_north_load()
}

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_STR), 136);
}

fn part2(input: &str) -> usize {
    let mut grid: Grid = input.parse().unwrap();

    grid.slide_cycle_many(1000000000);
    grid.get_north_load()
}

fn main() {
    println!("part 1: {}", part1(include_str!("input.txt")));
    println!("part 2: {}", part2(include_str!("input.txt")));
}

const TEST_STR: &str = r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
