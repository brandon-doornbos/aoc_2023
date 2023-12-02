use std::collections::HashMap;
use std::str::Lines;

fn main() {
    let input_raw = include_str!("input.txt");
    let input = input_raw.trim().lines();

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &Lines) -> i32 {
    let lut: HashMap<&str, i32> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    input
        .clone()
        .filter_map(|str| parse_line_1(str, &lut))
        .sum::<i32>()
}

fn parse_line_1(str: &str, lut: &HashMap<&str, i32>) -> Option<i32> {
    let split: Vec<&str> = str.splitn(3, [' ', ':']).collect();

    for set in split[2].split(';') {
        let cubes = set
            .trim_start()
            .split([' ', ','])
            .filter(|str| !str.is_empty())
            .collect::<Vec<&str>>();

        for i in (0..cubes.len()).step_by(2) {
            if cubes[i].parse::<i32>().unwrap() > lut[cubes[i + 1]] {
                return None;
            }
        }
    }

    Some(split[1].parse().unwrap())
}

fn part_2(input: &Lines) -> i32 {
    input.clone().map(|str| parse_line_2(str)).sum::<i32>()
}

fn parse_line_2(str: &str) -> i32 {
    let split: Vec<&str> = str.splitn(3, [' ', ':']).collect();

    let mut map: HashMap<&str, i32> = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

    for set in split[2].split(';') {
        let cubes = set
            .trim_start()
            .split([' ', ','])
            .filter(|str| !str.is_empty())
            .collect::<Vec<&str>>();

        for i in (0..cubes.len()).step_by(2) {
            let n = cubes[i].parse::<i32>().unwrap();
            if &n > map.get(cubes[i + 1]).unwrap() {
                let x = map.get_mut(cubes[i + 1]).unwrap();
                *x = n;
            }
        }
    }

    map.get("red").unwrap() * map.get("green").unwrap() * map.get("blue").unwrap()
}
