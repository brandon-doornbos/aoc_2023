use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt").trim().lines();

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Play<'a> {
    hand_type: Hand,
    hand: &'a str,
    bid: usize,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Hand {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Hand {
    fn parse_1(hand: &str) -> Hand {
        let mut labels: HashMap<char, i32> = HashMap::new();

        for c in hand.chars() {
            if labels.get(&c).is_none() {
                labels.insert(c, 0);
            }
            *labels.get_mut(&c).unwrap() += 1;
        }

        match labels.len() {
            1 => Hand::FiveOfAKind,
            2 => {
                let mut values = labels.values();
                if values.next().unwrap() == &4 || values.next().unwrap() == &4 {
                    return Hand::FourOfAKind;
                }
                Hand::FullHouse
            }
            3 => {
                let mut values = labels.values();
                if values.next().unwrap() == &3
                    || values.next().unwrap() == &3
                    || values.next().unwrap() == &3
                {
                    return Hand::ThreeOfAKind;
                }
                Hand::TwoPair
            }
            4 => Hand::OnePair,
            _ => Hand::HighCard,
        }
    }

    fn parse_2(hand: &str) -> Hand {
        let mut labels: HashMap<char, i32> = HashMap::new();
        let mut jokers = 0;

        for c in hand.chars() {
            if c == 'J' {
                jokers += 1;
                continue;
            }
            if labels.get(&c).is_none() {
                labels.insert(c, 0);
            }
            *labels.get_mut(&c).unwrap() += 1;
        }

        match labels.len() {
            0 | 1 => Hand::FiveOfAKind,
            2 => {
                let mut values = labels.values();
                let first = *values.next().unwrap();
                let second = *values.next().unwrap();
                if jokers > 0 {
                    if first == 2 && second == 2 {
                        return Hand::FullHouse;
                    }
                    Hand::FourOfAKind
                } else {
                    if first == 4 || second == 4 {
                        return Hand::FourOfAKind;
                    }
                    Hand::FullHouse
                }
            }
            3 => {
                let mut values = labels.values();
                let first = *values.next().unwrap();
                let second = *values.next().unwrap();
                let third = *values.next().unwrap();
                if jokers > 0 || (first == 3 || second == 3 || third == 3) {
                    return Hand::ThreeOfAKind;
                }
                Hand::TwoPair
            }
            4 => Hand::OnePair,
            _ => Hand::HighCard,
        }
    }
}

fn cmp(first: &str, second: &str, cards: &HashMap<char, i32>) -> Ordering {
    let mut first_chars = first.chars();
    let mut second_chars = second.chars();

    loop {
        let first_char = first_chars.next();
        if first_char.is_none() {
            return Ordering::Equal;
        }

        let ordering = cards
            .get(&first_char.unwrap())
            .cmp(&cards.get(&second_chars.next().unwrap()));

        if ordering != Ordering::Equal {
            return ordering;
        }
    }
}

fn part_1(input: &std::str::Lines) -> usize {
    let mut plays: Vec<Play> = vec![];
    for line in input.clone() {
        let play = line.split_at(6);
        let hand = play.0.trim_end();
        plays.push(Play {
            hand_type: Hand::parse_1(hand),
            hand,
            bid: play.1.parse().unwrap(),
        });
    }

    let cards = HashMap::from([
        ('A', 0),
        ('K', 1),
        ('Q', 2),
        ('J', 3),
        ('T', 4),
        ('9', 5),
        ('8', 6),
        ('7', 7),
        ('6', 8),
        ('5', 9),
        ('4', 10),
        ('3', 11),
        ('2', 12),
    ]);

    // println!("{:#?}", plays);
    plays.sort_unstable_by(|a, b| {
        if std::mem::discriminant(&a.hand_type) == std::mem::discriminant(&b.hand_type) {
            return cmp(a.hand, b.hand, &cards);
        }
        a.hand_type.cmp(&b.hand_type)
    });
    // println!("{:#?}", plays);

    plays
        .iter()
        .enumerate()
        .fold(0, |acc, (i, play)| acc + ((plays.len() - i) * play.bid))
}

fn part_2(input: &std::str::Lines) -> usize {
    let mut plays: Vec<Play> = vec![];
    for line in input.clone() {
        let play = line.split_at(6);
        let hand = play.0.trim_end();
        plays.push(Play {
            hand_type: Hand::parse_2(hand),
            hand,
            bid: play.1.parse().unwrap(),
        });
    }

    let cards = HashMap::from([
        ('A', 0),
        ('K', 1),
        ('Q', 2),
        ('T', 3),
        ('9', 4),
        ('8', 5),
        ('7', 6),
        ('6', 7),
        ('5', 8),
        ('4', 9),
        ('3', 10),
        ('2', 11),
        ('J', 12),
    ]);

    // println!("{:#?}", plays);
    plays.sort_unstable_by(|a, b| {
        if std::mem::discriminant(&a.hand_type) == std::mem::discriminant(&b.hand_type) {
            return cmp(a.hand, b.hand, &cards);
        }
        a.hand_type.cmp(&b.hand_type)
    });
    // println!("{:#?}", plays);

    plays
        .iter()
        .enumerate()
        .fold(0, |acc, (i, play)| acc + ((plays.len() - i) * play.bid))
}
