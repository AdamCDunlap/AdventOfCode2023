use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

struct Garden {
    map: Vec<Vec<u8>>,
    start: Coord,
    infinite: bool,
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
        if self.infinite {
            coord
                .around()
                .into_iter()
                .filter(|c| {
                    self.map[c.y.rem_euclid(self.height()) as usize]
                        [c.x.rem_euclid(self.width()) as usize]
                        == b'.'
                })
                .collect()
        } else {
            coord
                .around()
                .into_iter()
                .filter(|c| self.is_in_bounds(c))
                .filter(|c| self.map[c.y as usize][c.x as usize] == b'.')
                .collect()
        }
    }

    fn reachable_from<'a>(&self, prev_points: impl Iterator<Item = &'a Coord>) -> HashSet<Coord> {
        prev_points.flat_map(|c| self.plots_around(c)).collect()
    }

    fn reachable_from_start_after_steps(&self, steps: i64) -> HashSet<Coord> {
        let mut coords = HashSet::from([self.start.clone()]);
        for i in 0..steps {
            coords = self.reachable_from(coords.iter());
            // println!("Iteration {i}");
            // self.display_positions(&coords);
        }
        coords
    }

    fn from_str(input: &str, infinite: bool) -> Self {
        let mut map: Vec<Vec<u8>> = input.lines().map(|l| l.into()).collect();
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
        Garden {
            map,
            start: start.unwrap(),
            infinite,
        }
    }

    fn points_in_subgarden(
        &self,
        points: &HashSet<Coord>,
        subgarden_x: i64,
        subgarden_y: i64,
    ) -> BTreeSet<Coord> {
        let min_x = subgarden_x * self.width();
        let max_x = min_x + self.width();
        let min_y = subgarden_y * self.height();
        let max_y = min_y + self.height();
        points
            .iter()
            // .filter(|p| p.x >= min_x && p.x < max_x && p.y >= min_y && p.y < max_y)
            .map(|c| Coord {
                x: c.x - min_x,
                y: c.y - min_y,
            })
            .filter(|c| self.is_in_bounds(c))
            .collect()
    }

    fn finite_from_str(input: &str) -> Self {
        Self::from_str(input, false)
    }

    fn infinite_from_str(input: &str) -> Self {
        Self::from_str(input, true)
    }

    fn display_positions(&self, coords: &HashSet<Coord>) {
        let mut map = self.map.clone();

        for c in self.points_in_subgarden(coords, 0, 0) {
            map[c.y as usize][c.x as usize] = b'O';
        }

        // for c in coords.iter() {
        //     let c = &Coord {
        //         x: c.x - 3 * self.width(),
        //         y: c.y - self.width(),
        //     };
        //     if self.is_in_bounds(c) {
        //         map[c.y as usize][c.x as usize] = b'O';
        //     }
        // }
        for line in map {
            println!("{}", std::str::from_utf8(&line).unwrap());
        }
    }
}

#[test]
fn test_reachable_after_steps() {
    let garden = Garden::finite_from_str(TEST_STR);
    assert_eq!(garden.reachable_from_start_after_steps(1).len(), 2);
    assert_eq!(garden.reachable_from_start_after_steps(2).len(), 4);
    assert_eq!(garden.reachable_from_start_after_steps(3).len(), 6);
    assert_eq!(garden.reachable_from_start_after_steps(6).len(), 16);
}

fn part1(input: &str) -> usize {
    Garden::finite_from_str(input)
        .reachable_from_start_after_steps(64)
        .len()
}

#[test]
fn test_reachable_after_steps_infinite() {
    let garden = Garden::infinite_from_str(TEST_STR);
    assert_eq!(garden.reachable_from_start_after_steps(6).len(), 16);
    assert_eq!(garden.reachable_from_start_after_steps(10).len(), 50);
    assert_eq!(garden.reachable_from_start_after_steps(50).len(), 1594);
    assert_eq!(garden.reachable_from_start_after_steps(100).len(), 6536);
    assert_eq!(garden.reachable_from_start_after_steps(500).len(), 167004);
    assert_eq!(garden.reachable_from_start_after_steps(1000).len(), 668697);
    assert_eq!(
        garden.reachable_from_start_after_steps(5000).len(),
        16733044
    );
}

fn part2(input: &str) -> usize {
    Garden::infinite_from_str(input)
        .reachable_from_start_after_steps(26501365)
        .len()
}

fn play_with(input: &str) {
    let garden = Garden::infinite_from_str(input);

    for (start_point, name) in [
        (garden.start.clone(), "middle"),
        (Coord { x: 0, y: 0 }, "top left"),
        (
            Coord {
                x: garden.width() - 1,
                y: 0,
            },
            "top right",
        ),
        (
            Coord {
                x: 0,
                y: garden.height() - 1,
            },
            "bottm left",
        ),
        (
            Coord {
                x: garden.width() - 1,
                y: garden.height() - 1,
            },
            "bottom right",
        ),
        (
            Coord {
                x: garden.start.x,
                y: 0,
            },
            "top middle",
        ),
        (
            Coord {
                x: garden.start.x,
                y: garden.height() - 1,
            },
            "bottom middle",
        ),
        (
            Coord {
                x: 0,
                y: garden.start.y,
            },
            "middle left",
        ),
        (
            Coord {
                x: garden.width() - 1,
                y: garden.start.y,
            },
            "middle right",
        ),
    ] {
        let mut coords = HashSet::from([start_point]);
        for i in 0..131 {
            coords = garden.reachable_from(coords.iter());
        }
        println!("Starting from {name:15} gives {}", coords.len());
    }

    let start_iteration = 1400;
    let mut coords_after = HashMap::from([(
        start_iteration,
        garden.reachable_from_start_after_steps(start_iteration),
    )]);

    // let samples = [
    //     (0, 0, "start"),
    //     (0, 1, "below"),
    //     (0, 3, "far below"),
    //     (0, -1, "above"),
    //     (0, -3, "far above"),
    //     (1, 0, "right"),
    //     (3, 0, "far right"),
    //     (-1, 0, "left"),
    //     (-3, 0, "far left"),
    //     (2, 3, "bottom right"),
    //     (3, -4, "top right"),
    //     (-2, 1, "top left"),
    //     (-4, -2, "bottom left"),
    // ];

    let box_size = 5;
    let samples = (-box_size..=box_size).flat_map(|x| (-box_size..=box_size).map(move |y| (x, y)));

    for (x_off, y_off) in samples {
        // println!("Checking {name}");
        let mut cache: HashMap<BTreeSet<Coord>, Vec<i64>> = HashMap::new();

        for i in 0..100 {
            let iteration = start_iteration + i;

            let next = if let Some(next) = coords_after.get(&iteration) {
                next
            } else {
                let next =
                    garden.reachable_from(coords_after.get(&(iteration - 1)).unwrap().iter());
                coords_after.entry(iteration).or_insert(next)
            };

            let next_in_bounds = garden.points_in_subgarden(&next, x_off, y_off);

            // if next_in_bounds.is_empty() {
            //     println!("Don't have enough data yet for ({x_off:3},{y_off:3})");
            //     break;
            // }
            cache
                .entry(next_in_bounds)
                .and_modify(|iters| iters.push(iteration))
                .or_insert(vec![iteration]);

            //let next = coords_after.entry(start_iteration + i).or_insert_with
            // cache.entry();
        }
        let mut lens: Vec<(usize, &[i64])> = cache
            .iter()
            .map(|(set, iters)| (set.len(), &iters[0..(2.min(iters.len()))]))
            .collect();
        lens.sort_by_key(|(_set_len, iters)| *iters);
        println!(
            "Cache length {} for ({x_off:3},{y_off:3}). Lens are {lens:?}",
            cache.len()
        );

        // let n_in_bounds = garden.points_in_subgarden(&coords_after_n, x_off, y_off);
        // let np1_in_bounds = garden.points_in_subgarden(&coords_after_np1, x_off, y_off);
        // let np2_in_bounds = garden.points_in_subgarden(&coords_after_np2, x_off, y_off);
        // let np3_in_bounds = garden.points_in_subgarden(&coords_after_np3, x_off, y_off);
        // let np4_in_bounds = garden.points_in_subgarden(&coords_after_np4, x_off, y_off);
        // let np5_in_bounds = garden.points_in_subgarden(&coords_after_np5, x_off, y_off);

        // println!(
        //     "Checking {name}. Lens: {} {} {} {}",
        //     n_in_bounds.len(),
        //     np2_in_bounds.len(),
        //     np2_in_bounds.len(),
        //     np3_in_bounds.len()
        // );
        // assert_eq!(n_in_bounds, np2_in_bounds);
        // assert_eq!(np1_in_bounds, np3_in_bounds);
    }
}

fn num_reachable_after_steps_bruteforce(input: &str, steps: i64) -> u64 {
    Garden::infinite_from_str(input)
        .reachable_from_start_after_steps(steps)
        .len() as u64
}

fn num_reachable_after_maps_mathy(diamond_size: u64) -> u64 {
    // let diamond_size = 202300u64;
    // let diamond_size = 3u64;
    let inner_diamond_size = diamond_size - 1;
    let num_squares_in_inner_diamond = (inner_diamond_size + 1) * (inner_diamond_size + 1)
        + inner_diamond_size * inner_diamond_size;
    let mut num_even = 1;
    let mut num_odd = 0;
    let mut ring = 1;
    while num_even + num_odd < num_squares_in_inner_diamond {
        if ring % 2 == 0 {
            num_even += ring * 4;
        } else {
            num_odd += ring * 4;
        }
        ring += 1;
    }
    assert!(num_even + num_odd == num_squares_in_inner_diamond);
    // dbg!(num_even);
    // dbg!(num_odd);

    let inner_even_val = 7265;
    let inner_odd_val = 7325;

    let total_inner = num_even * inner_even_val + num_odd * inner_odd_val;

    let outer_corner_val = 14853 * 2 + 14852 * 2;
    let side_val = inner_diamond_size * (14790 + 14795 + 14793 + 14786);
    // println!("part 2: {}", outer_corner_val + side_val + total_inner);
    outer_corner_val + side_val + total_inner
}

fn main() {
    let input = &std::fs::read_to_string("input.txt").expect("input.txt should exist");
    // println!("part 1: {}", part1(input));
    // println!("part 2: {}", part2(input));
    // asdf();
    // play_with(input);

    // println!("part 1 bruteforce: {}", num_reachable_after_steps_bruteforce(input, 64));
    // println!("part 1 mathy: {}", num_reachable_after_maps_mathy(1));

    println!(
        "1x1 bruteforce: {}",
        num_reachable_after_steps_bruteforce(input, 65 + 131 * 1)
    );
    println!("1x1 mathy: {}", num_reachable_after_maps_mathy(1));

    println!(
        "2x2 bruteforce: {}",
        num_reachable_after_steps_bruteforce(input, 65 + 131 * 2)
    );
    println!("2x2 mathy: {}", num_reachable_after_maps_mathy(2));

    println!(
        "3x3 bruteforce: {}",
        num_reachable_after_steps_bruteforce(input, 65 + 131 * 3)
    );
    println!("3x3 mathy: {}", num_reachable_after_maps_mathy(3));
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
