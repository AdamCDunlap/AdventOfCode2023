use std::str::FromStr;

#[derive(Debug, Clone)]
struct Brick {
    name: String,
    north: i64,
    south: i64,
    east: i64,
    west: i64,
    top: i64,
    bottom: i64,
}

impl FromStr for Brick {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<Vec<i64>> = s
            .split('~')
            .map(|p| p.split(',').map(|v| v.parse().unwrap()).collect())
            .collect();
        Ok(Brick {
            east: std::cmp::min(parts[0][0], parts[1][0]),
            west: std::cmp::max(parts[0][0], parts[1][0]),
            north: std::cmp::min(parts[0][1], parts[1][1]),
            south: std::cmp::max(parts[0][1], parts[1][1]),
            bottom: std::cmp::min(parts[0][2], parts[1][2]),
            top: std::cmp::max(parts[0][2], parts[1][2]),
            name: "?".to_string(),
        })
    }
}

impl Brick {
    fn overlaps_xy(&self, other: &Brick) -> bool {
        self.east <= other.west
            && self.west >= other.east
            && self.north <= other.south
            && self.south >= other.north
    }

    fn supports(&self, other: &Brick) -> bool {
        (other.bottom == (self.top + 1)) && self.overlaps_xy(other)
    }

    fn num_under(&self, others: &[Brick]) -> usize {
        others.iter().filter(|o| o.supports(self)).count()
    }

    fn is_safe_to_disintegrate(&self, others: &[Brick]) -> bool {
        others
            .iter()
            .find(|b| {
                if !self.supports(b) {
                    return false;
                }
                let num_under = b.num_under(others);
                // println!(
                //     "{:?} under {:?} which is on top of {num_under}",
                //     self, b
                // );
                b.num_under(others) == 1
            })
            .is_none()
    }
}

#[test]
fn test_overlaps_xy() {
    let bricks: Vec<Brick> = TEST_INPUT.lines().map(|l| l.parse().unwrap()).collect();

    let should_overlap = |a: usize, b: usize| {
        assert!(bricks[a].overlaps_xy(&bricks[b]));
        assert!(bricks[b].overlaps_xy(&bricks[a]));
    };

    should_overlap(0, 1);
    should_overlap(0, 2);
    should_overlap(2, 3);
    should_overlap(2, 4);
    should_overlap(3, 5);
    should_overlap(4, 5);
    should_overlap(5, 6);
}

fn parse_bricks(input: &str) -> Vec<Brick> {
    input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            let mut b: Brick = l.parse().unwrap();
            b.name = String::from_utf8(vec![b'A' + (i % 26) as u8]).unwrap();
            b
        })
        .collect()
}

#[test]
fn test_supports() {
    let mut bricks = parse_bricks(TEST_INPUT);
    settle_bricks(&mut bricks);
    bricks.sort_unstable_by_key(|b| b.name.clone());

    let check_supporters = |test: usize, supporters: &[usize]| {
        for i in 0..bricks.len() {
            if supporters.iter().find(|x| **x == i).is_some() {
                assert!(
                    bricks[test].supports(&bricks[i]),
                    "{test} ({:?}) should support {i} ({:?})",
                    bricks[test],
                    bricks[i]
                );
            } else {
                assert!(
                    !bricks[test].supports(&bricks[i]),
                    "{test} ({:?}) should NOT support {i} ({:?})",
                    bricks[test],
                    bricks[i]
                );
            }
        }
    };

    check_supporters(0, &[1, 2]);
    check_supporters(2, &[3, 4]);
    check_supporters(3, &[5]);
    check_supporters(4, &[5]);
    check_supporters(5, &[6]);
    check_supporters(6, &[]);
}

fn settle_bricks(bricks: &mut Vec<Brick>) {
    bricks.sort_unstable_by_key(|b| b.bottom);

    for falling_idx in 0..bricks.len() {
        let highest_below = (0..falling_idx)
            .rev()
            .filter(|below_idx| bricks[falling_idx].overlaps_xy(&bricks[*below_idx]))
            .map(|below_idx| bricks[below_idx].top)
            .max()
            .unwrap_or(0);
        let amount_to_fall = bricks[falling_idx].bottom - highest_below - 1;
        assert!(amount_to_fall >= 0);
        bricks[falling_idx].bottom -= amount_to_fall;
        bricks[falling_idx].top -= amount_to_fall;
        assert!(bricks[falling_idx].bottom > 0);
    }
}

fn count_bricks_disintegrated_chain(bricks: &[Brick], to_delete: usize) -> usize {
    let mut bricks: Vec<Option<Brick>> = bricks[..to_delete]
        .iter()
        .chain(bricks[to_delete + 1..].iter())
        .map(|b| Some(b.clone()))
        .collect();
    bricks.sort_unstable_by_key(|b| b.as_ref().unwrap().bottom);

    for test_idx in 0..bricks.len() {
        if bricks[test_idx].as_ref().unwrap().bottom == 1 {
            continue;
        }
        if (0..test_idx)
            .find(|below_idx| {
                let below = &bricks[*below_idx];
                let Some(ref below) = below else {
                    // println!("Index {below_idx} is None. array: {bricks:?}");
                    return false;
                };
                below.supports(bricks[test_idx].as_ref().unwrap())
            })
            .is_none()
        {
            // println!("Setting index {test_idx}, value {:?} to None", bricks[test_idx]);
            bricks[test_idx] = None;
        }
    }

    bricks.iter().filter(|b| b.is_none()).count()
}

#[test]
fn test_count_bricks_disintegrated_chain() {
    let mut bricks = parse_bricks(TEST_INPUT);
    settle_bricks(&mut bricks);

    assert_eq!(count_bricks_disintegrated_chain(&bricks, 0), 6);
    assert_eq!(count_bricks_disintegrated_chain(&bricks, 1), 0);
    assert_eq!(count_bricks_disintegrated_chain(&bricks, 2), 0);
    assert_eq!(count_bricks_disintegrated_chain(&bricks, 3), 0);
    assert_eq!(count_bricks_disintegrated_chain(&bricks, 4), 0);
    assert_eq!(count_bricks_disintegrated_chain(&bricks, 5), 1);
    assert_eq!(count_bricks_disintegrated_chain(&bricks, 6), 0);
}

fn part1(input: &str) -> usize {
    let mut bricks = parse_bricks(input);
    settle_bricks(&mut bricks);

    bricks
        .iter()
        .filter(|b| b.is_safe_to_disintegrate(&bricks))
        .count()
}

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 5);
}

fn part2(input: &str) -> usize {
    let mut bricks = parse_bricks(input);
    settle_bricks(&mut bricks);
    (0..bricks.len())
        .map(|to_delete| count_bricks_disintegrated_chain(&bricks, to_delete))
        .sum()
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 7);
}

fn main() {
    let input = &std::fs::read_to_string("input.txt").expect("input.txt should exist");
    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

const TEST_INPUT: &str = r"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
