use std::{collections::HashSet, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    fn around(&self) -> [Coord; 4] {
        [
            Coord {
                x: self.x,
                y: self.y - 1,
            },
            Coord {
                x: self.x,
                y: self.y + 1,
            },
            Coord {
                x: self.x - 1,
                y: self.y,
            },
            Coord {
                x: self.x + 1,
                y: self.y,
            },
        ]
    }
}

// enum Square {
//     Plot,
//     Rock,
// }

struct Garden {
    map: Vec<Vec<u8>>,
    start: Coord,
}

impl Garden {
    fn width(&self) -> i64 {
        self.map[0].len() as i64
    }
    fn height(&self) -> i64 {
        self.map.len() as i64
    }
    fn is_in_bounds(&self, coord: &Coord) -> bool {
        coord.x >= 0 && coord.y >= 0 && coord.x < self.width() && coord.y < self.height()
    }
    fn plots_around(&self, coord: &Coord) -> Vec<Coord> {
        coord
            .around()
            .into_iter()
            .filter(|c| self.is_in_bounds(c))
            .filter(|c| self.map[c.y as usize][c.x as usize] == b'.')
            .collect()
    }

    fn reachable_after_steps(&self, steps: i64) -> HashSet<Coord> {
        let mut coords = HashSet::from([self.start.clone()]);
        for _ in 0..steps {
            coords = coords
                .into_iter()
                .flat_map(|c| self.plots_around(&c))
                .collect()
        }
        coords
    }
}

impl FromStr for Garden {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map: Vec<Vec<u8>> = s.lines().map(|l| l.into()).collect();
        let mut start = None;
        for (y, line) in map.iter_mut().enumerate() {
            for (x, ch) in line.iter_mut().enumerate() {
                if *ch == b'S' {
                    *ch = b'.';
                    assert!(start.is_none());
                    start = Some(Coord {
                        x: x as i64,
                        y: y as i64,
                    });
                }
            }
        }
        Ok(Garden {
            map,
            start: start.unwrap(),
        })
    }
}

fn part1(input: &str) -> usize {
    let garden: Garden = input.parse().unwrap();
    garden.reachable_after_steps(64).len()
}

#[test]
fn test_reachable_after_steps() {
    let garden: Garden = TEST_STR.parse().unwrap();
    assert_eq!(garden.reachable_after_steps(1).len(), 2);
    assert_eq!(garden.reachable_after_steps(2).len(), 4);
    assert_eq!(garden.reachable_after_steps(3).len(), 6);
    assert_eq!(garden.reachable_after_steps(6).len(), 16);
}

fn main() {
    let input = &std::fs::read_to_string("input.txt").expect("input.txt should exist");
    println!("part 1: {}", part1(input));
    // println!("part 2: {}", part2(input));
}

const TEST_STR: &str = r"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
