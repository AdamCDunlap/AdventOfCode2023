use std::sync::atomic::ATOMIC_USIZE_INIT;

fn parse_numbers(s: &str) -> Vec<u64> {
    s.split(' ').filter_map(|n| n.parse().ok()).collect()
}

struct Race {
    time: u64,
    distance: u64,
}

fn parse_races(s: &str) -> Vec<Race> {
    let mut lines = s.lines();
    let times = parse_numbers(lines.next().unwrap());
    let distances = parse_numbers(lines.next().unwrap());
    times
        .iter()
        .zip(distances.iter())
        .map(|(&time, &distance)| Race { time, distance })
        .collect()
}

fn distance_travelled(race_time: u64, charge_time: u64) -> u64 {
    charge_time * (race_time - charge_time)
}
#[test]
fn test_distance_travelled() {
    assert_eq!(distance_travelled(7, 0), 0);
    assert_eq!(distance_travelled(7, 1), 6);
    assert_eq!(distance_travelled(7, 2), 10);
    assert_eq!(distance_travelled(7, 3), 12);
    assert_eq!(distance_travelled(7, 4), 12);
    assert_eq!(distance_travelled(7, 5), 10);
    assert_eq!(distance_travelled(7, 6), 6);
    assert_eq!(distance_travelled(7, 7), 0);

    assert!(distance_travelled(30, 10) <= 200);
    assert!(distance_travelled(30, 11) > 200);
    assert!(distance_travelled(30, 19) > 200);
    assert!(distance_travelled(30, 20) <= 200);
}

fn minmax_charge_to_win(race: &Race) -> (u64, u64) {
    // Travelled = charge_time * (race_time - charge_time)
    // Want to beat distance, so solve
    // distance < charge_time * (race_time - charge_time)
    // distance < charge_time * race_time - charge_time^2
    // -charge_time^2 + race_time * charge_time - distance > 0
    // roots of charge_time = (race_time +- sqrt(race_time^2 - 4*distance))/2

    let sqrt_discriminant = ((race.time * race.time - 4 * race.distance) as f64).sqrt();
    let min = (race.time as f64 - sqrt_discriminant) / 2.0;
    let max = (race.time as f64 + sqrt_discriminant) / 2.0;
    let mut int_min = min.ceil();
    if int_min == min {
        int_min += 1.0;
    }
    let mut int_max = max.floor();
    if int_max == max {
        int_max -= 1.0;
    }
    (int_min as u64, int_max as u64)
}

#[test]
fn test_minmax_charge_to_win() {
    assert_eq!(
        minmax_charge_to_win(&Race {
            distance: 9,
            time: 7
        }),
        (2, 5)
    );
    assert_eq!(
        minmax_charge_to_win(&Race {
            distance: 40,
            time: 15
        }),
        (4, 11)
    );
    assert_eq!(
        minmax_charge_to_win(&Race {
            distance: 200,
            time: 30
        }),
        (11, 19)
    );
}

fn ways_to_win_race(race: &Race) -> u64 {
    let (min, max) = minmax_charge_to_win(race);
    max - min + 1
}

fn part1(input: &str) -> u64 {
    use std::ops::Mul;
    let races = parse_races(input);
    races.iter().map(ways_to_win_race).fold(1, u64::mul)
}

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT1), 288);
}

#[test]
fn test_part2() {
    assert_eq!(part1(TEST_INPUT2), 71503);
}

fn main() {
    println!("part 1: {}", part1(REAL_INPUT1));
    println!("part 2: {}", part1(REAL_INPUT2));
}

const TEST_INPUT1: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

const REAL_INPUT1: &str = r#"Time:        55     82     64     90
Distance:   246   1441   1012   1111"#;

const TEST_INPUT2: &str = r#"Time:      71530
Distance:  940200"#;

const REAL_INPUT2: &str = r#"Time:        55826490
Distance:   246144110121111"#;
