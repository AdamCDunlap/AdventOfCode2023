use std::fmt::{Display, Write};

#[derive(Debug, Eq, PartialEq, Clone)]
enum ReflectionLine {
    Vertical(usize),
    Horizontal(usize),
}

impl ReflectionLine {
    fn score(&self) -> usize {
        match self {
            ReflectionLine::Vertical(num_left) => *num_left,
            ReflectionLine::Horizontal(num_above) => num_above * 100,
        }
    }
}

#[derive(Clone)]
struct Pattern(Vec<String>);

impl Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for l in self.0.iter() {
            f.write_str(l)?;
            f.write_char('\n')?;
        }
        Ok(())
    }
}

fn invert(ch: char) -> char {
    match ch {
        '#' => '.',
        '.' => '#',
        _ => panic!("Unrecognized character {}", ch),
    }
}

impl Pattern {
    fn from_str(str: &str) -> Self {
        Self(str.lines().map(String::from).collect())
    }

    fn height(&self) -> usize {
        self.0.len()
    }
    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn has_vertical_reflection_at(&self, num_left: usize) -> bool {
        let num_to_check = usize::min(num_left, self.width() - num_left);
        for x in 0..num_to_check {
            for y in 0..self.height() {
                if self.0[y].as_bytes()[num_left - x - 1] != self.0[y].as_bytes()[num_left + x] {
                    return false;
                }
            }
        }
        true
    }

    fn has_horizontal_reflection_at(&self, num_above: usize) -> bool {
        let num_to_check = usize::min(num_above, self.height() - num_above);
        for y in 0..num_to_check {
            if self.0[num_above - y - 1] != self.0[num_above + y] {
                return false;
            }
        }
        true
    }

    fn has_reflection_at(&self, line: &ReflectionLine) -> bool {
        match line {
            ReflectionLine::Vertical(n) => self.has_vertical_reflection_at(*n),
            ReflectionLine::Horizontal(n) => self.has_horizontal_reflection_at(*n),
        }
    }

    fn find_reflection_excluding(
        &self,
        exclude: Option<&ReflectionLine>,
    ) -> Result<ReflectionLine, ()> {
        (1..self.height())
            .map(|n| ReflectionLine::Horizontal(n))
            .chain((1..self.width()).map(|n| ReflectionLine::Vertical(n)))
            .filter(|line| self.has_reflection_at(line))
            .filter(|line| exclude != Some(line))
            .next()
            .ok_or(())
    }

    fn find_reflection(&self) -> Result<ReflectionLine, ()> {
        self.find_reflection_excluding(None)
    }

    fn find_smudged_reflection(&self) -> ReflectionLine {
        let orig_reflection = self.find_reflection().unwrap();

        let mut copy = self.clone();

        for lineidx in 0..copy.height() {
            for rowidx in 0..copy.width() {
                let orig_byte = char::from_u32(copy.0[lineidx].as_bytes()[rowidx] as u32).unwrap();
                copy.0[lineidx].replace_range(rowidx..rowidx + 1, &invert(orig_byte).to_string());
                if let Ok(new_line) = copy.find_reflection_excluding(Some(&orig_reflection)) {
                    // println!("line {:?} from:\n{}", new_line, copy);
                    return new_line;
                } else {
                    // println!("no line from:\n{}", copy);
                }
                copy.0[lineidx].replace_range(rowidx..rowidx + 1, &orig_byte.to_string());
            }
        }

        panic!("No change made a different reflection line:\n{}", self);
    }
}

#[test]
fn test_has_horizontal_reflection_at() {
    assert!(Pattern::from_str(
        r"#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
    )
    .has_horizontal_reflection_at(4));

    assert!(Pattern::from_str(
        r"A
B
B
A
C"
    )
    .has_horizontal_reflection_at(2));

    assert!(Pattern::from_str(
        r"A
B
B
A
C"
    )
    .has_horizontal_reflection_at(2));
    assert!(Pattern::from_str(
        r"A
A"
    )
    .has_horizontal_reflection_at(1));
}

#[test]
fn test_has_vertial_reflection_at() {
    assert!(Pattern::from_str(r"AA").has_vertical_reflection_at(1));
    assert!(!Pattern::from_str(r"ABA").has_vertical_reflection_at(1));
    assert!(!Pattern::from_str(r"ABA").has_vertical_reflection_at(2));
    assert!(!Pattern::from_str(r"ABBA").has_vertical_reflection_at(1));
    assert!(Pattern::from_str(r"ABBA").has_vertical_reflection_at(2));
    assert!(!Pattern::from_str(r"ABBA").has_vertical_reflection_at(3));
    assert!(Pattern::from_str(r"AABBA").has_vertical_reflection_at(3));
    assert!(Pattern::from_str(r"AABBA").has_vertical_reflection_at(1));
    assert!(Pattern::from_str(r"XYZAA").has_vertical_reflection_at(4));
}

#[test]
fn test_find_reflections() {
    assert_eq!(
        Pattern::from_str(
            r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."
        )
        .find_reflection(),
        Ok(ReflectionLine::Vertical(5))
    );

    assert_eq!(
        Pattern::from_str(
            r"#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
        )
        .find_reflection(),
        Ok(ReflectionLine::Horizontal(4))
    );

    assert_eq!(
        Pattern::from_str(
            r".#.####.#....
#.#....#.#...
###....##.###
#.##..##.#.##
.#.#..#.#.###
#.######.#...
#.##..##.####"
        )
        .find_reflection(),
        Ok(ReflectionLine::Vertical(12))
    );
}

#[test]
fn test_find_smudged_reflections() {
    assert_eq!(
        Pattern::from_str(
            r".#.####
##..#.#
##..#.#
.#.####
..#..#.
####.#.
#.#.#.#
.#..#.#
##.##..
#.#..#.
#.#...."
        )
        .find_smudged_reflection(),
        ReflectionLine::Horizontal(10)
    );
}

fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .flat_map(|pattern| Pattern::from_str(pattern).find_reflection())
        .map(|l| l.score())
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|pattern| Pattern::from_str(pattern).find_smudged_reflection())
        .map(|l| l.score())
        .sum()
}

#[test]
fn test_part1() {
    assert_eq!(
        part1(
            r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
        ),
        405
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        part2(
            r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
        ),
        400
    );
}

fn main() {
    println!("part 1: {}", part1(include_str!("input.txt")));
    println!("part 2: {}", part2(include_str!("input.txt")));
}
