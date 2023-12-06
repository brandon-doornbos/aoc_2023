fn main() {
    let input = include_str!("input.txt").trim().lines();

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &std::str::Lines) -> i32 {
    let mut parsed: Vec<Vec<i32>> = input
        .clone()
        .map(|str| {
            str.split_at(9)
                .1
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    let distance = parsed.swap_remove(1);
    let time = parsed.swap_remove(0);
    let mut values = vec![];

    for i in 0..distance.len() {
        for t in 0..time[i] {
            if t * (time[i] - t) > distance[i] {
                values.push(time[i] - (2 * (t - 1)) - 1);
                break;
            }
        }
    }

    values.iter().product()
}

fn part_2(input: &std::str::Lines) -> i64 {
    let mut parsed: Vec<i64> = input
        .clone()
        .map(|str| {
            str.split_at(9)
                .1
                .chars()
                .filter(char::is_ascii_digit)
                .collect::<String>()
                .parse()
                .unwrap()
        })
        .collect();

    let distance = parsed.swap_remove(1);
    let time = parsed.swap_remove(0);

    let mut t = time / 2;
    let mut iter = 4;

    loop {
        if t * (time - t) > distance && (t - 1) * (time - (t - 1)) < distance {
            return time - (2 * (t - 1)) - 1;
        } else if t * (time - t) > distance {
            t -= time / iter;
        } else {
            t += time / iter;
        }
        iter *= 2;
    }
}
