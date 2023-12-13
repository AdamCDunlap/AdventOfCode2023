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

struct Pattern<'a>(&'a [&'a [u8]]);

impl<'a> Display for Pattern<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for l in self.0.iter() {
            for ch in l.iter() {
                f.write_char(char::from_u32(*ch as u32).unwrap())?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl<'a> Pattern<'a> {
    fn from_bytestr(bytestr: &'static [u8]) -> Self {
        Self(bytestr.split(|c| *c == b'\n').collect::<Vec<_>>().leak())
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
                if self.0[y][num_left - x - 1] != self.0[y][num_left + x] {
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

    fn find_reflection(&self) -> Result<ReflectionLine, ()> {
        let lines: Vec<ReflectionLine> = (1..self.height())
            .filter(|num_above| self.has_horizontal_reflection_at(*num_above))
            .map(|n| ReflectionLine::Horizontal(n))
            .chain(
                (1..self.width())
                    .filter(|num_left| self.has_vertical_reflection_at(*num_left))
                    .map(|n| ReflectionLine::Vertical(n)),
            )
            .collect();

        assert!(lines.len() <= 1);
        lines.first().ok_or(()).cloned()
    }


    fn find_smudged_reflection(&self) -> Result<ReflectionLine, ()> {
        let lines: Vec<ReflectionLine> = (1..self.height())
            .filter(|num_above| self.has_horizontal_reflection_at(*num_above))
            .map(|n| ReflectionLine::Horizontal(n))
            .chain(
                (1..self.width())
                    .filter(|num_left| self.has_vertical_reflection_at(*num_left))
                    .map(|n| ReflectionLine::Vertical(n)),
            )
            .collect();

        assert!(lines.len() <= 1);
        lines.first().ok_or(()).cloned()
    }
}

#[test]
fn test_has_horizontal_reflection_at() {
    assert!(Pattern::from_bytestr(
        br"#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
    )
    .has_horizontal_reflection_at(4));

    assert!(Pattern::from_bytestr(
        br"A
B
B
A
C"
    )
    .has_horizontal_reflection_at(2));

    assert!(Pattern::from_bytestr(
        br"A
B
B
A
C"
    )
    .has_horizontal_reflection_at(2));
    assert!(Pattern::from_bytestr(
        br"A
A"
    )
    .has_horizontal_reflection_at(1));
}

#[test]
fn test_has_vertial_reflection_at() {
    assert!(Pattern::from_bytestr(br"AA").has_vertical_reflection_at(1));
    assert!(!Pattern::from_bytestr(br"ABA").has_vertical_reflection_at(1));
    assert!(!Pattern::from_bytestr(br"ABA").has_vertical_reflection_at(2));
    assert!(!Pattern::from_bytestr(br"ABBA").has_vertical_reflection_at(1));
    assert!(Pattern::from_bytestr(br"ABBA").has_vertical_reflection_at(2));
    assert!(!Pattern::from_bytestr(br"ABBA").has_vertical_reflection_at(3));
    assert!(Pattern::from_bytestr(br"AABBA").has_vertical_reflection_at(3));
    assert!(Pattern::from_bytestr(br"AABBA").has_vertical_reflection_at(1));
    assert!(Pattern::from_bytestr(br"XYZAA").has_vertical_reflection_at(4));
}

#[test]
fn test_find_reflections() {
    assert_eq!(
        Pattern::from_bytestr(
            br"#.##..##.
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
        Pattern::from_bytestr(
            br"#...##..#
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
        Pattern::from_bytestr(
            br".#.####.#....
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

fn part1(input: &[u8]) -> usize {
    let lines: Vec<_> = input.split(|c| *c == b'\n').collect();
    lines
        .split(|line| line.is_empty())
        .flat_map(|pattern| Pattern(pattern).find_reflection())
        .map(|l| l.score())
        .sum()
}


fn part2(input: &[u8]) -> usize {
    let lines: Vec<_> = input.split(|c| *c == b'\n').collect();
    lines
        .split(|line| line.is_empty())
        .map(|pattern| Pattern(pattern).find_smudged_reflection().unwrap())
        .map(|l| l.score())
        .sum()
}


#[test]
fn test_part1() {
    assert_eq!(
        part1(
            br"#.##..##.
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

fn main() {
    println!("part 1: {}", part1(include_bytes!("input.txt")));
    println!("part 2: {}", part2(include_bytes!("input.txt")));
}
