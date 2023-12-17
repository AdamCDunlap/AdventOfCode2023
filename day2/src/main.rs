use std::{cmp, num::ParseIntError};

#[derive(Default, PartialEq, Eq, Debug)]
struct Colors {
    red: u32,
    blue: u32,
    green: u32,
}

impl Colors {
    fn maxes(self, other: &Colors) -> Colors {
        Colors {
            red: cmp::max(self.red, other.red),
            blue: cmp::max(self.blue, other.blue),
            green: cmp::max(self.green, other.green),
        }
    }

    fn can_be_played_by(&self, other: &Colors) -> bool {
        other.red >= self.red && other.blue >= self.blue && other.green >= self.green
    }

    fn power(&self) -> u32 {
        self.red * self.blue * self.green
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Game {
    colors: Colors,
    id: u32,
}

#[derive(Debug, PartialEq, Eq)]
enum AocError<'a> {
    InvalidNumColorFormat,
    DoesntHaveOneColon,
    DoesntStartWithGame,
    UnknownColor(&'a str),
    ParseIntError(ParseIntError),
}

impl From<ParseIntError> for AocError<'_> {
    fn from(value: ParseIntError) -> Self {
        AocError::ParseIntError(value)
    }
}

fn parse_numcol<'a>(c: &mut Colors, numcol: &'a str) -> Result<(), AocError<'a>> {
    let numcol = numcol.trim();

    let [num, col]: [&str; 2] = numcol
        .split(' ')
        .collect::<Vec<_>>()
        .try_into()
        .map_err(|_| AocError::InvalidNumColorFormat)?;

    let num: u32 = num.parse()?;
    match col {
        "red" => c.red += num,
        "blue" => c.blue += num,
        "green" => c.green += num,
        _ => return Err(AocError::UnknownColor(col)),
    }
    Ok(())
}

#[test]
fn test_parse_numcol() {
    let mut colors = Colors::default();
    assert_eq!(parse_numcol(&mut colors, " 1 red  "), Ok(()));
    assert_eq!(colors.red, 1);

    assert_eq!(parse_numcol(&mut colors, " 5 blue  "), Ok(()));
    assert_eq!(colors.blue, 5);
}

fn parse_roll(roll: &str) -> Result<Colors, AocError> {
    let mut colors = Colors::default();
    for roll in roll.split(',') {
        parse_numcol(&mut colors, roll)?;
    }

    Ok(colors)
}

#[test]
fn test_parse_roll() {
    assert!(matches!(
        parse_roll("  3 blue, 4 red  "),
        Ok(Colors {
            red: 4,
            blue: 3,
            green: 0
        })
    ));
}

fn parse_line(line: &str) -> Result<Game, AocError> {
    let [gameinfo, rolls]: [&str; 2] = line
        .trim()
        .split(':')
        .collect::<Vec<_>>()
        .try_into()
        .map_err(|_| AocError::DoesntHaveOneColon)?;
    let gametext = "Game ";
    if !gameinfo.starts_with(gametext) {
        return Err(AocError::DoesntStartWithGame);
    }
    let gamenum = &gameinfo[gametext.len()..];
    let gamenum: u32 = gamenum.parse().map_err(|e| AocError::ParseIntError(e))?;

    let colors = rolls
        .split(';')
        .map(parse_roll)
        .reduce(|mc1, mc2| Ok(Colors::maxes(mc1?, &mc2?)))
        .unwrap_or(Ok(Colors::default()))?;

    Ok(Game {
        colors: colors,
        id: gamenum,
    })
}

#[test]
fn test_parse_line() {
    assert_eq!(
        parse_line("   Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n"),
        Ok(Game {
            colors: Colors {
                red: 1,
                green: 3,
                blue: 4
            },
            id: 2
        })
    )
}

fn aoc_part_1(s: &str, available: Colors) -> Result<u32, AocError> {
    let mut id_sum = 0;
    for line in s.lines() {
        let game = parse_line(line)?;
        if game.colors.can_be_played_by(&available) {
            id_sum += game.id;
        }
    }
    Ok(id_sum)
}

#[test]
fn test_aoc_part_1() {
    assert_eq!(
        aoc_part_1(
            r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green    "#,
            Colors {
                red: 12,
                green: 13,
                blue: 14
            }
        ),
        Ok(8)
    );
}

fn aoc_part_2(s: &str) -> Result<u32, AocError> {
    let mut power_sum = 0;
    for line in s.lines() {
        let game = parse_line(line)?;
        power_sum += game.colors.power();
    }
    Ok(power_sum)
}

#[test]
fn test_aoc_part_2() {
    assert_eq!(
        aoc_part_2(
            r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green    "#
        ),
        Ok(2286)
    );
}

fn main() {
    let input = &std::fs::read_to_string("input.txt").expect("input.txt should exist");
    println!(
        "part 1: {}",
        aoc_part_1(
            input,
            Colors {
                red: 12,
                green: 13,
                blue: 14
            }
        )
        .unwrap()
    );
    println!("part 2: {}", aoc_part_2(input).unwrap());
}
