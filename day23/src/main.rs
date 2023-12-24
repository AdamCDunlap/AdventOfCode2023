use std::{
    collections::{HashSet, VecDeque},
    ops::Index,
    str::FromStr,
};

struct Maze {
    maze: Vec<Vec<u8>>,
}

impl FromStr for Maze {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            maze: s.trim().lines().map(|line| line.trim().into()).collect(),
        })
    }
}

struct FoundTiles {
    longest_path_to: Vec<Vec<Vec<HashSet<Coord>>>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Coord(isize, isize);

impl Maze {
    fn width(&self) -> isize {
        self.maze[0].len() as isize
    }
    fn height(&self) -> isize {
        self.maze.len() as isize
    }

    fn is_blocked(&self, coord: &Coord) -> bool {
        if coord.0 < 0 || coord.0 >= self.width() || coord.1 < 0 || coord.1 >= self.height() {
            return true;
        }
        match self[*coord] {
            b'#' => true,
            _ => false,
        }
    }

    fn possible_next_steps(&self, coord: &Coord) -> Vec<Coord> {
        let left = Coord(coord.0 - 1, coord.1);
        let right = Coord(coord.0 + 1, coord.1);
        let above = Coord(coord.0, coord.1 - 1);
        let below = Coord(coord.0, coord.1 + 1);
        let steps = match self[*coord] {
            b'>' => vec![right],
            b'<' => vec![left],
            b'^' => vec![above],
            b'v' => vec![below],
            b'.' => vec![left, right, above, below],
            _ => unreachable!(),
        };

        steps
            .into_iter()
            .filter(|step| !self.is_blocked(step))
            .collect()
    }

    fn print_found_tiles(&self, found_tiles: &FoundTiles) {
        for y in 0..self.height() as usize {
            let mut boundary_line = String::new();
            let mut numbered_line = String::new();
            for x in 0..self.width() as usize {
                let ch = std::char::from_u32(self.maze[y][x] as u32).unwrap();
                let five_ch = format!("{ch}{ch}{ch}{ch}{ch}");
                if self.maze[y][x] == b'#' {
                    boundary_line.push_str(&five_ch);
                    numbered_line.push_str(&five_ch);
                // } else if let Some(ref path) = found_tiles.longest_path_to[y][x] {
                //     let path_len_str = format!("{}", path.len());
                //     // let last_char = &path_len_str[path_len_str.len() - 1..];
                //     numbered_line.push_str(&format!("{ch}{path_len_str:.^3.3}{ch}"));
                //     boundary_line.push_str(&five_ch);
                } else {
                    numbered_line.push_str(&format!("{ch}xxx{ch}"));
                    boundary_line.push_str(&five_ch);
                }
            }
            println!("{boundary_line}\n{numbered_line}\n{boundary_line}");
        }
    }
    fn print_path(&self, found_tiles: &FoundTiles, longest_path: &HashSet<Coord>) {
        println!();
        for y in 0..self.height() {
            let mut boundary_line = String::new();
            let mut numbered_line = String::new();
            for x in 0..self.width() {
                let coord = Coord(x, y);

                let ch = std::char::from_u32(self[coord] as u32).unwrap();
                let five_ch = format!("{ch}{ch}{ch}{ch}{ch}");
                if self[coord] == b'#' {
                    assert!(!longest_path.contains(&coord));
                    boundary_line.push_str(&five_ch);
                    numbered_line.push_str(&five_ch);
                } else if longest_path.contains(&coord) {
                    // let path = found_tiles.longest_path_to[y as usize][x as usize]
                    //     .as_ref()
                    //     .unwrap();
                    // let path_len_str = format!("{}", path.len());

                    // numbered_line.push_str(&format!("{ch}{path_len_str:.^3.3}{ch}"));
                    numbered_line.push_str(&format!("{ch}{ch}O{ch}{ch}"));
                    boundary_line.push_str(&five_ch);
                } else {
                    boundary_line.push_str(&five_ch);
                    numbered_line.push_str(&five_ch);
                }
            }
            println!("{boundary_line}\n{numbered_line}\n{boundary_line}");
        }
        println!();
    }

    fn max_path(&self) -> usize {
        let mut found_tiles = FoundTiles {
            longest_path_to: vec![vec![vec![]; self.width() as usize]; self.height() as usize],
        };

        let start_coord = Coord(1, 0);
        let mut to_examine: VecDeque<(Coord, usize)> = VecDeque::from([(start_coord, 0)]);
        found_tiles.longest_path_to[0][1] = vec![HashSet::new()];

        while let Some((here, path_idx)) = to_examine.pop_front() {
            for next in self.possible_next_steps(&here) {
                let path_to_here =
                    &found_tiles.longest_path_to[here.1 as usize][here.0 as usize][path_idx];
                if path_to_here.contains(&next) {
                    // Already saw this coordinate
                    continue;
                }
                // for path_to_here in
                //     found_tiles.longest_path_to[here.1 as usize][here.0 as usize].iter().cloned()
                // {

                // if let Some(existing) = &longest_path_to[next.1 as usize][next.0 as usize] {
                //     if existing.len() >= path_to_here.len() + 1 {
                //         continue;
                //     }
                // }

                let mut path_to_next = path_to_here.clone();
                path_to_next.insert(here.clone());

                // self.print_path(&found_tiles, &path_to_next);

                let longest_path_to_next =
                    &mut found_tiles.longest_path_to[next.1 as usize][next.0 as usize];

                to_examine.push_back((next, longest_path_to_next.len()));
                longest_path_to_next.push(path_to_next);
                // self.print_found_tiles(&found_tiles);
            }
        }

        // println!();

        found_tiles.longest_path_to[self.height() as usize - 1][self.width() as usize - 2]
            .iter()
            .map(|p| p.len())
            .max()
            .unwrap()
    }

    // fn max_dist
}

impl Index<Coord> for Maze {
    type Output = u8;

    fn index(&self, index: Coord) -> &Self::Output {
        &self.maze[index.1 as usize][index.0 as usize]
    }
}

fn part1(input: &str) -> usize {
    input.parse::<Maze>().unwrap().max_path()
}

// #[test]
// fn test_tiny() {
//     assert_eq!(part2("#.#\n#.#\n#.#"), 2);
//     assert_eq!(part2("#.#\n#..\n#.#"), 2);
//     assert_eq!(
//         part2(
//             r"
//     #.##
//     ....
//     .#..
//     .#.#"
//         ),
//         5
//     );
// }

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_STR), 94);
}

fn part2(input: &str) -> usize {
    input
        .replace(">", ".")
        .replace("<", ".")
        .replace("v", ".")
        .replace("^", ".")
        .parse::<Maze>()
        .unwrap()
        .max_path()
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_STR), 154);
}

fn main() {
//     part2(
//         r"
// #.##
// ....
// .#..
// .#.#",
//     );

    let input = &std::fs::read_to_string("input.txt").expect("input.txt should exist");
    println!("Part 1: {}", part1(input));
    // println!("Part 2: {}", part2(input));
}

const TEST_STR: &str = r"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
