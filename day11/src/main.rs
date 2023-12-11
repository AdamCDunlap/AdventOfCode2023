use std::{fmt::Display, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
enum AocError {
    InvalidMapEntry,
}

#[derive(Debug, PartialEq, Eq)]
enum Point {
    Galaxy,
    Empty,
}

#[derive(Debug)]
struct StarMap {
    points: Vec<Vec<Point>>,
    galaxies: Vec<(usize, usize)>,
    do_rows_have_galaxies: Vec<bool>,
    do_cols_have_galaxies: Vec<bool>,
}

impl FromStr for StarMap {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_points(
            s.lines()
                .map(|l| {
                    l.chars()
                        .map(|ch| {
                            Ok(match ch {
                                '.' => Point::Empty,
                                '#' => Point::Galaxy,
                                _ => return Err(AocError::InvalidMapEntry),
                            })
                        })
                        .collect()
                })
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

impl Display for StarMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        for row in self.points.iter() {
            for pt in row {
                f.write_char(match pt {
                    Point::Empty => '.',
                    Point::Galaxy => '#',
                })?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl StarMap {
    fn from_points(points: Vec<Vec<Point>>) -> StarMap {
        let nrows = points.len();
        if nrows == 0 {
            return StarMap {
                points,
                do_cols_have_galaxies: vec![],
                do_rows_have_galaxies: vec![],
                galaxies: vec![],
            };
        }
        let ncols = points[0].len();

        let mut do_rows_have_galaxies: Vec<bool> = vec![false; nrows];
        let mut do_cols_have_galaxies: Vec<bool> = vec![false; ncols];
        let mut galaxies = vec![];
        for row in 0..nrows {
            for col in 0..ncols {
                match &points[row][col] {
                    &Point::Galaxy => {
                        do_rows_have_galaxies[row] = true;
                        do_cols_have_galaxies[col] = true;
                        galaxies.push((col, row));
                    }
                    &Point::Empty => (),
                }
            }
        }
        StarMap {
            points,
            do_cols_have_galaxies,
            do_rows_have_galaxies,
            galaxies,
        }
    }

    fn get_distance(
        &self,
        p1: (usize, usize),
        p2: (usize, usize),
        expansion_coefficient: usize,
    ) -> usize {
        let min_x = std::cmp::min(p1.0, p2.0);
        let max_x = std::cmp::max(p1.0, p2.0);
        let min_y = std::cmp::min(p1.1, p2.1);
        let max_y = std::cmp::max(p1.1, p2.1);
        (min_x..max_x)
            .map(|col| {
                if self.do_cols_have_galaxies[col] {
                    1
                } else {
                    expansion_coefficient
                }
            })
            .sum::<usize>()
            + (min_y..max_y)
                .map(|row| {
                    if self.do_rows_have_galaxies[row] {
                        1
                    } else {
                        expansion_coefficient
                    }
                })
                .sum::<usize>()
    }

    fn galaxy_distance_sum(&self, expansion_coefficient: usize) -> usize {
        (0..self.galaxies.len())
            .map(|i| {
                (i + 1..self.galaxies.len())
                    .map(|j| {
                        self.get_distance(self.galaxies[i], self.galaxies[j], expansion_coefficient)
                    })
                    .sum::<usize>()
            })
            .sum()
    }
}

#[test]
fn test_get_distance() {
    let map = TEST_STR.parse::<StarMap>().unwrap();
    assert_eq!(map.get_distance(map.galaxies[0], map.galaxies[6], 2), 15);
    assert_eq!(map.get_distance(map.galaxies[2], map.galaxies[5], 2), 17);
    assert_eq!(map.get_distance(map.galaxies[7], map.galaxies[8], 2), 5);
}

fn part1(input: &str) -> usize {
    input.parse::<StarMap>().unwrap().galaxy_distance_sum(2)
}

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_STR), 374);
}

fn part2(input: &str) -> usize {
    input
        .parse::<StarMap>()
        .unwrap()
        .galaxy_distance_sum(1000000)
}

#[test]
fn test_part2() {
    let map = TEST_STR.parse::<StarMap>().unwrap();

    assert_eq!(map.galaxy_distance_sum(10), 1030);
    assert_eq!(map.galaxy_distance_sum(100), 8410);
}

fn main() {
    let input = include_str!("real_input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

const TEST_STR: &str = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;
