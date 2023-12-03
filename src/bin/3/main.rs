use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt")
        .trim()
        .lines()
        .map(|str| str.chars().collect())
        .collect();

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn is_symbol(c: char) -> bool {
    !(c == '.' || c.is_ascii_digit())
}

fn part_1(input: &Vec<Vec<char>>) -> u32 {
    let y = input.len() - 1;
    let x = input[0].len() - 1;

    let mut total = 0;

    for i in 0..=y {
        let mut part_num = 0;
        let mut valid = false;

        for j in 0..=x {
            if input[i][j].is_ascii_digit() {
                part_num *= 10;
                part_num += input[i][j].to_digit(10).unwrap_or(0);

                valid = valid
                    || (i > 0 && is_symbol(input[i - 1][j]))
                    || (i < y && is_symbol(input[i + 1][j]))
                    || (j > 0 && is_symbol(input[i][j - 1]))
                    || (j < x && is_symbol(input[i][j + 1]))
                    || (i > 0 && j > 0 && is_symbol(input[i - 1][j - 1]))
                    || (i > 0 && j < x && is_symbol(input[i - 1][j + 1]))
                    || (i < y && j > 0 && is_symbol(input[i + 1][j - 1]))
                    || (i < y && j < x && is_symbol(input[i + 1][j + 1]));

                if j != x {
                    continue;
                }
            }

            if valid {
                total += part_num;
            }
            part_num = 0;
            valid = false;
        }
    }

    total
}

fn part_2(input: &Vec<Vec<char>>) -> u32 {
    let y = input.len() - 1;
    let x = input[0].len() - 1;

    let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    for i in 0..=y {
        let mut part_num = 0;
        let mut valid_gears = vec![];

        for j in 0..=x {
            if input[i][j].is_ascii_digit() {
                part_num *= 10;
                part_num += input[i][j].to_digit(10).unwrap_or(0);

                if i > 0 && input[i - 1][j] == '*' {
                    valid_gears.push((i - 1, j));
                }
                if i < y && input[i + 1][j] == '*' {
                    valid_gears.push((i + 1, j));
                }
                if j > 0 && input[i][j - 1] == '*' {
                    valid_gears.push((i, j - 1));
                }
                if j < x && input[i][j + 1] == '*' {
                    valid_gears.push((i, j + 1));
                }
                if i > 0 && j > 0 && input[i - 1][j - 1] == '*' {
                    valid_gears.push((i - 1, j - 1));
                }
                if i > 0 && j < x && input[i - 1][j + 1] == '*' {
                    valid_gears.push((i - 1, j + 1));
                }
                if i < y && j > 0 && input[i + 1][j - 1] == '*' {
                    valid_gears.push((i + 1, j - 1));
                }
                if i < y && j < x && input[i + 1][j + 1] == '*' {
                    valid_gears.push((i + 1, j + 1));
                }

                if j != x {
                    continue;
                }
            }

            for gear in &valid_gears {
                if gears.get(gear).is_none() {
                    gears.insert(*gear, vec![]);
                }
                if !gears.get(gear).unwrap().contains(&part_num) {
                    gears.get_mut(gear).unwrap().push(part_num);
                }
            }
            part_num = 0;
            valid_gears.clear();
        }
    }

    let mut total = 0;

    for gear in gears {
        if gear.1.len() == 2 {
            total += gear.1[0] * gear.1[1];
        }
    }

    total
}
