use std::str::Split;

fn main() {
    let input_raw = include_str!("input.txt");
    let input = input_raw.trim().split('\n');

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &Split<char>) -> u32 {
    input
        .clone()
        .map(|str| line_to_value_1(str))
        .collect::<Vec<u32>>()
        .into_iter()
        .sum()
}

fn line_to_value_1(str: &str) -> u32 {
    let matches: Vec<&str> = str.matches(|c: char| c.is_digit(10)).collect();
    (matches[0].to_owned() + matches[matches.len() - 1])
        .parse()
        .unwrap()
}

fn part_2(input: &Split<char>) -> u32 {
    input
        .clone()
        .map(|str| line_to_value_2(str))
        .collect::<Vec<u32>>()
        .into_iter()
        .sum()
}

fn line_to_value_2(str: &str) -> u32 {
    let mut num = 0;
    let mut new_str = str;

    loop {
        if let Some(n) = starts_with_digit(new_str) {
            num += 10 * n;
            break;
        }
        new_str = new_str.split_at(1).1;
    }

    new_str = str;
    loop {
        if let Some(n) = ends_with_digit(new_str) {
            num += n;
            break;
        }
        new_str = new_str.split_at(new_str.len() - 1).0;
    }

    num
}

fn starts_with_digit(str: &str) -> Option<u32> {
    match str {
        s if s.starts_with("1") | s.starts_with("one") => Some(1),
        s if s.starts_with("2") | s.starts_with("two") => Some(2),
        s if s.starts_with("3") | s.starts_with("three") => Some(3),
        s if s.starts_with("4") | s.starts_with("four") => Some(4),
        s if s.starts_with("5") | s.starts_with("five") => Some(5),
        s if s.starts_with("6") | s.starts_with("six") => Some(6),
        s if s.starts_with("7") | s.starts_with("seven") => Some(7),
        s if s.starts_with("8") | s.starts_with("eight") => Some(8),
        s if s.starts_with("9") | s.starts_with("nine") => Some(9),
        _ => None,
    }
}

fn ends_with_digit(str: &str) -> Option<u32> {
    match str {
        s if s.ends_with("1") | s.ends_with("one") => Some(1),
        s if s.ends_with("2") | s.ends_with("two") => Some(2),
        s if s.ends_with("3") | s.ends_with("three") => Some(3),
        s if s.ends_with("4") | s.ends_with("four") => Some(4),
        s if s.ends_with("5") | s.ends_with("five") => Some(5),
        s if s.ends_with("6") | s.ends_with("six") => Some(6),
        s if s.ends_with("7") | s.ends_with("seven") => Some(7),
        s if s.ends_with("8") | s.ends_with("eight") => Some(8),
        s if s.ends_with("9") | s.ends_with("nine") => Some(9),
        _ => None,
    }
}
