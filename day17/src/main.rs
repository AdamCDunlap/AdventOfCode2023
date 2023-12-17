use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
    fs,
    ops::{Add, Index, IndexMut},
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Dir {
    North,
    South,
    East,
    West,
}

use Dir::*;

impl Dir {
    fn left(self) -> Self {
        match self {
            North => East,
            South => West,
            East => North,
            West => South,
        }
    }
    fn right(self) -> Self {
        match self {
            North => West,
            South => East,
            East => South,
            West => North,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct TileType {
    dir: Dir,
    steps_in_dir: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Coord {
    x: isize,
    y: isize,
}

struct Tile {
    heat_loss: u8,
    // Minimum loss from (0,0) to this tile found so far when entering from each direction.
    total_loss: HashMap<TileType, u64>,
}

struct Map(Vec<Vec<Tile>>);

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(|l| {
                    l.bytes()
                        .map(|ch| Tile {
                            heat_loss: ch - b'0',
                            total_loss: HashMap::new(),
                        })
                        .collect()
                })
                .collect(),
        ))
    }
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

impl Index<Coord> for Map {
    type Output = Tile;

    fn index(&self, index: Coord) -> &Self::Output {
        &self.0[index.y as usize][index.x as usize]
    }
}

impl IndexMut<Coord> for Map {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        &mut self.0[index.y as usize][index.x as usize]
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Map {
    fn height(&self) -> usize {
        self.0.len()
    }
    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn is_in_bounds(&self, coord: Coord) -> bool {
        coord.x >= 0
            && coord.y >= 0
            && (coord.x as usize) < self.width()
            && (coord.y as usize) < self.height()
    }

    fn find_min_basic(&mut self) -> Option<u64> {
        self.find_min(0, 3)
    }

    fn find_min(&mut self, min_dist: u8, max_dist: u8) -> Option<u64> {
        let mut to_examine: VecDeque<(Coord, TileType)> = VecDeque::new();
        let start_tiletype = TileType {
            dir: East,
            steps_in_dir: 0,
        };
        to_examine.push_back((Coord { x: 0, y: 0 }, start_tiletype));
        self[Coord { x: 0, y: 0 }].total_loss = HashMap::from([(start_tiletype, 0)]);

        while let Some((
            coord,
            prev_tt @ TileType {
                dir: incoming_dir,
                steps_in_dir,
            },
        )) = to_examine.pop_front()
        {
            for next_dir in [incoming_dir, incoming_dir.left(), incoming_dir.right()] {
                let this_loss = self[coord].total_loss[&prev_tt];

                let next_coord = coord + next_dir;
                if !self.is_in_bounds(next_coord) {
                    continue;
                }
                let is_staight = next_dir == incoming_dir;
                if !is_staight && steps_in_dir < min_dist {
                    continue;
                }
                let next_steps = if is_staight { steps_in_dir + 1 } else { 1 };
                if next_steps > max_dist {
                    continue;
                }

                let next = &mut self[next_coord];
                let loss = this_loss + next.heat_loss as u64;
                let tt = TileType {
                    dir: next_dir,
                    steps_in_dir: next_steps,
                };
                let mut changed = false;
                next.total_loss
                    .entry(tt)
                    .and_modify(|prev_loss| {
                        if loss < *prev_loss {
                            *prev_loss = loss;
                            changed = true
                        }
                    })
                    .or_insert_with(|| {
                        changed = true;
                        loss
                    });
                if changed {
                    to_examine.push_back((next_coord, tt));
                }

                // println!(
                //     "{}->{} loss: {} tt: {:?}. this_loss: {}, next loss: {}",
                //     coord, next_coord, loss, tt, this_loss, next.heat_loss
                // );
            }
        }

        // for line in self.0.iter() {
        //     for dir in [North, South, East, West] {
        //         print!("{:5?}:  ", dir);
        //         for tile in line.iter() {
        //             let best = (1..4)
        //                 .filter_map(|steps_in_dir| {
        //                     tile.total_loss.get(&TileType { dir, steps_in_dir })
        //                 })
        //                 .min()
        //                 .expect("There should be a path to every tile");
        //             print!("{:3} | ", best);
        //         }
        //         println!();
        //     }
        // }

        self[Coord {
            x: (self.width() - 1) as isize,
            y: (self.height() - 1) as isize,
        }]
        .total_loss
        .iter()
        .filter_map(|(tt, loss)| {
            if tt.steps_in_dir < min_dist {
                None
            } else {
                Some(loss)
            }
        })
        .min()
        .cloned()
    }
}

fn part1(input: &str) -> u64 {
    let mut map: Map = input.parse().unwrap();

    map.find_min_basic().unwrap()
}

#[test]
fn test_find_min() {
    // assert_eq!("1234".parse::<Map>().unwrap().find_min(), Some(9));
    // assert_eq!("12345".parse::<Map>().unwrap().find_min(), None);
    assert_eq!(
        "191\n111\n991".parse::<Map>().unwrap().find_min_basic(),
        Some(4)
    );
}

#[test]
fn test_part1() {
    assert_eq!(
        part1(
            r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"
        ),
        102
    );
}

fn part2(input: &str) -> u64 {
    let mut map: Map = input.parse().unwrap();

    map.find_min(4, 10).unwrap()
}

#[test]
fn test_part2() {
    assert_eq!(
        part2(
            r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"
        ),
        94
    );

    assert_eq!(
        part2(
            r"111111111111
999999999991
999999999991
999999999991
999999999991"
        ),
        71
    );
}

fn main() {
    let input = &fs::read_to_string("input.txt").expect("input.txt should exist");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
