fn linenumber(s: &str) -> Option<u32> {
    let spelled_nums = [
        ("0", 0),
        ("1", 1),
        ("one", 1),
        ("2", 2),
        ("two", 2),
        ("3", 3),
        ("three", 3),
        ("4", 4),
        ("four", 4),
        ("5", 5),
        ("five", 5),
        ("6", 6),
        ("six", 6),
        ("7", 7),
        ("seven", 7),
        ("8", 8),
        ("eight", 8),
        ("9", 9),
        ("nine", 9),
    ];

    let firstdigit = spelled_nums
        .iter()
        .filter_map(|(search, val)| Some((s.find(search)?, val)))
        .min_by_key(|(pos, _)| *pos)?
        .1;

    let lastdigit = spelled_nums
        .iter()
        .filter_map(|(search, val)| Some((s.rfind(search)?, val)))
        .max_by_key(|(pos, _)| *pos)?
        .1;
    // dbg!(s);
    // dbg!(firstdigit);
    // dbg!(lastdigit);

    // let firstdigitchar = s.chars().find(|x| char::is_digit(*x, 10))?;
    // let lastdigitchar = s.chars().rev().find(|x| char::is_digit(*x, 10))?;

    // let firstdigit = firstdigitchar.to_digit(10).unwrap();
    // let lastdigit = lastdigitchar.to_digit(10).unwrap();

    Some(firstdigit * 10 + lastdigit)
}

fn main() -> Result<(), ()> {
    let input = &std::fs::read_to_string("input.txt").expect("input.txt should exist");

    //     let str = r#"
    //     two1nine
    // eightwothree
    // abcone2threexyz
    // xtwone3four
    // 4nineeightseven2
    // zoneight234
    // 7pqrstsixteen"#;

    let sum: u32 = input
        .lines()
        .map(linenumber)
        .fold(Some(0), |a, b| Some(a.unwrap_or(0) + b.unwrap_or(0)))
        .unwrap();
    println!("result: {}", sum);
    Ok(())
}
