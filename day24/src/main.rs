use std::str::FromStr;

#[derive(Debug)]
struct Line {
    px: i64,
    py: i64,
    // pz: i64,
    vx: i64,
    vy: i64,
    // vz: i64,
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<i64> = s
            .split(|ch| ch == ',' || ch == '@')
            .map(|n| n.trim().parse().unwrap())
            .collect();
        assert!(numbers.len() == 6);
        Ok(Line {
            px: numbers[0],
            py: numbers[1],
            // pz: numbers[2],
            vx: numbers[3],
            vy: numbers[4],
            // vz: numbers[5],
        })
    }
}

impl Line {
    fn xy_intersection(&self, other: &Line) -> Option<(f64, f64)> {
        // println!("Check if {self:?} intersects {other:?}");
        // First, find equations for each line.
        // A1*px1 + B1 = py1
        // A1*(px1 + vx1) + B1 = py1 + vy1
        // Subtract them:
        // A1 * vx1 = vy1
        // A1 = vy1 / vx1
        // B1 = py1 - px1 * A1
        // A2 = vy2 / vx2
        // B2 = py2 - px2 * A2
        //
        //
        // Let X,Y be the intersection point.
        // A1*X + B1 = Y
        // A2*X + B2 = Y
        // Subtract them:
        // A1*X + B1 - A2*X - B2 = 0
        // X * (A1 - A2) + B1 - B2 = 0
        // X = (B2 - B1) / (A1 - A2)
        // Y = A1 * X + B1

        let vx1 = self.vx as f64;
        let vy1 = self.vy as f64;
        let px1 = self.px as f64;
        let py1 = self.py as f64;

        let vx2 = other.vx as f64;
        let vy2 = other.vy as f64;
        let px2 = other.px as f64;
        let py2 = other.py as f64;

        // if vx1 == 0.0 || vx2 == 0 {
        //     return None;
        // }

        let a1 = vy1 / vx1;
        let b1 = py1 - px1 * a1;
        let a2 = vy2 / vx2;
        let b2 = py2 - px2 * a2;

        if a1 == a2 {
            None
        } else {
            let x = (b2 - b1) / (a1 - a2);
            let y = a1 * x + b1;

            if (x - px1).signum() != vx1.signum() {
                print!("In past for 1. ");
                None
            } else if (x - px2).signum() != vx2.signum() {
                print!("In past for 2. ");

                None
            } else {
                Some((x, y))
            }
        }
    }
}

fn count_xy_intersections_in_test_zone(input: &str, min_xy: f64, max_xy: f64) -> usize {
    let lines: Vec<Line> = input.trim().lines().map(|l| l.parse().unwrap()).collect();

    (0..lines.len())
        .map(|l1_idx| {
            let lines = &lines;

            (l1_idx + 1..lines.len())
                .filter(move |l2_idx| {
                    let l1 = &lines[l1_idx];
                    let l2 = &lines[*l2_idx];
                    print!("Check if {l1:?} intersects {l2:?}: ");

                    let Some(intersection) = lines[l1_idx].xy_intersection(&lines[*l2_idx]) else {
                        println!("Do not intersect");
                        return false;
                    };
                    println!("Intersect at ({},{})", intersection.0, intersection.1);

                    intersection.0 >= min_xy
                        && intersection.0 <= max_xy
                        && intersection.1 >= min_xy
                        && intersection.1 <= max_xy
                })
                .count()
        })
        .sum()
}

#[test]
fn test_count_xy_intersections_in_test_zone() {
    assert_eq!(
        count_xy_intersections_in_test_zone(TEST_INPUT, 7.0, 27.0),
        2
    );
}

fn part1(input: &str) -> usize {
    count_xy_intersections_in_test_zone(input, 200000000000000.0, 400000000000000.0)
}

fn main() {
    let input = &std::fs::read_to_string("input.txt").expect("input.txt should exist");
    println!("Part 1: {}", part1(input));
}

const TEST_INPUT: &str = r"
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
";
