use std::collections::HashMap;

use aoc_2023::lcm;

fn main() {
    let mut input = include_str!("input.txt").trim().lines();
    let directions: Vec<usize> = input
        .next()
        .unwrap()
        .chars()
        .map(|c| if c == 'L' { 0 } else { 1 })
        .collect();
    input.next();
    let map: HashMap<&str, [&str; 2]> = input.map(str_to_element).collect();

    println!("Part 1: {}", part_1(&directions, &map));
    println!("Part 2: {}", part_2(&directions, &map));
}

fn str_to_element(str: &str) -> (&str, [&str; 2]) {
    let parsed = str.split_at(3);
    let element: Vec<&str> = parsed
        .1
        .split(['=', ' ', '(', ',', ')'])
        .filter(|str| !str.is_empty())
        .collect();
    (parsed.0, [element[0], element[1]])
}

fn count_steps(
    directions: &[usize],
    map: &HashMap<&str, [&str; 2]>,
    start_element: &str,
    end_condition: fn(&str) -> bool,
) -> u64 {
    let mut steps = 0;
    let mut element = start_element;
    for direction in directions.iter().cycle() {
        if end_condition(element) {
            break;
        }
        element = map.get(&element).unwrap()[*direction];
        steps += 1;
    }
    steps
}

fn part_1(directions: &[usize], map: &HashMap<&str, [&str; 2]>) -> u64 {
    count_steps(directions, map, "AAA", |element| element == "ZZZ")
}

fn part_2(directions: &[usize], map: &HashMap<&str, [&str; 2]>) -> u64 {
    map.keys()
        .filter(|&element| element.ends_with('A'))
        .map(|element| count_steps(directions, map, element, |element| element.ends_with('Z')))
        .reduce(lcm)
        .unwrap()
}
