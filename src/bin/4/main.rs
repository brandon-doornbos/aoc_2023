struct Card {
    id: usize,
    wins: usize,
}

fn main() {
    let input = include_str!("input.txt")
        .trim()
        .lines()
        .map(parse_line)
        .collect();

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn parse_line(line: &str) -> Card {
    let parts: Vec<&str> = line.split(':').collect();
    let id: usize = parts[0].split_whitespace().collect::<Vec<&str>>()[1]
        .parse()
        .unwrap();

    let numbers: Vec<Vec<i32>> = parts[1]
        .split('|')
        .map(|str| {
            str.split_whitespace()
                .map(|str| str.parse().unwrap_or(0))
                .collect()
        })
        .collect();

    let wins = numbers[1]
        .iter()
        .fold(0, |acc, n| acc + numbers[0].contains(n) as usize);

    Card { id, wins }
}

fn part_1(input: &Vec<Card>) -> i32 {
    let mut total = 0;

    for card in input {
        total += if card.wins > 0 {
            2_i32.pow((card.wins - 1).try_into().unwrap())
        } else {
            0
        };
    }

    total
}

fn part_2(input: &Vec<Card>) -> i32 {
    let mut card_cache = vec![1; input.len()];

    for card in input {
        for i in 0..card.wins {
            card_cache[card.id + i] += card_cache[card.id - 1];
        }
    }

    card_cache.iter().sum()
}
