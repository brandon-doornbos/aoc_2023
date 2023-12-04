struct Card {
    id: usize,
    have: Vec<i32>,
    winning: Vec<i32>,
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
    let card: usize = parts[0].split_whitespace().collect::<Vec<&str>>()[1]
        .parse::<usize>()
        .unwrap()
        - 1;

    let mut numbers: Vec<Vec<i32>> = parts[1]
        .split('|')
        .map(|str| {
            str.split_whitespace()
                .map(|str| str.parse().unwrap_or(0))
                .collect()
        })
        .collect();

    Card {
        id: card,
        have: numbers.remove(1),
        winning: numbers.remove(0),
    }
}

fn part_1(input: &Vec<Card>) -> i32 {
    let mut total = 0;

    for card in input {
        let mut points = 0;

        for n in &card.have {
            if card.winning.contains(n) {
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }
        }

        total += points;
    }

    total
}

fn part_2(input: &Vec<Card>) -> usize {
    let mut card_cache = vec![1; input.len()];

    for card in input {
        let mut points: usize = 0;

        for n in &card.have {
            if card.winning.contains(n) {
                points += 1;
                card_cache[card.id + points] += card_cache[card.id];
            }
        }
    }

    card_cache.iter().sum()
}
