fn parse(input: &str) -> Vec<i64> {
    input
        .split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect()
}

fn get_differences(nums: &Vec<i64>) -> Vec<i64> {
    nums.windows(2).map(|win| win[1] - win[0]).collect()
}

#[test]
fn test_get_differences() {
    assert_eq!(get_differences(&[-2, 3, 18, 24].into()), [5, 15, 6]);
    assert_eq!(get_differences(&[1].into()), []);
}

fn compute_all_differences(nums: Vec<i64>) -> Vec<Vec<i64>> {
    let mut differences: Vec<Vec<i64>> = vec![nums];
    loop {
        let next_differences = get_differences(&differences.last().unwrap());
        if next_differences.iter().all(|x| *x == 0) {
            break;
        }
        differences.push(next_differences);
    }
    differences
}

#[test]
fn test_compute_all_differences() {
    assert_eq!(
        compute_all_differences(vec![0, 1, 2, 3]),
        [vec![0, 1, 2, 3], vec![1, 1, 1]]
    );
    assert_eq!(
        compute_all_differences(vec![0, 1, 3, 6]),
        [vec![0, 1, 3, 6], vec![1, 2, 3], vec![1, 1]]
    );
}

fn get_next(nums: Vec<i64>) -> i64 {
    let differences = compute_all_differences(nums);
    let mut last_end_diff = 0;
    for diff in differences.iter().rev() {
        last_end_diff += diff.last().unwrap();
    }
    last_end_diff
}

fn get_prev(nums: Vec<i64>) -> i64 {
    let differences = compute_all_differences(nums);
    let mut last_start_diff = 0;
    for diff in differences.iter().rev() {
        last_start_diff = diff.first().unwrap() - last_start_diff;
    }
    last_start_diff
}

fn part1(input: &str) -> i64 {
    input.lines().map(|l| get_next(parse(l))).sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 114);
}

fn part2(input: &str) -> i64 {
    input.lines().map(|l| get_prev(parse(l))).sum()
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 2);
}

fn main() {
    let input = &std::fs::read_to_string("input.txt").expect("input.txt should exist");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

const TEST_INPUT: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;
