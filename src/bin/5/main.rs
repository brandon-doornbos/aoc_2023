#[derive(Debug)]
struct Translation {
    source: i64,
    destination: i64,
    length: i64,
}

fn main() {
    let input = include_str!("input.txt").trim().lines();
    let (seeds, translations) = preprocess(&input);

    println!("Part 1: {}", part_1(&seeds, &translations));
    println!("Part 2: {}", part_2(&seeds, &translations));
}

fn preprocess(input: &std::str::Lines) -> (Vec<i64>, Vec<Vec<Translation>>) {
    let mut iter = input.clone();
    let seeds: Vec<i64> = iter
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut translations = vec![vec![]];

    for line in iter {
        if line == "" {
            continue;
        } else if line.starts_with(|c: char| c.is_ascii_alphabetic()) {
            translations.push(vec![]);
        } else {
            let translation: Vec<i64> = line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();

            let translation_index = translations.len() - 1;
            translations[translation_index].push(Translation {
                destination: translation[0],
                source: translation[1],
                length: translation[2],
            })
        }
    }

    (seeds, translations)
}

fn process_translations(numbers: &mut Vec<i64>, translations: &Vec<Translation>) {
    for number in numbers {
        for translation in translations {
            if *number >= translation.source && *number < translation.source + translation.length {
                *number += translation.destination - translation.source;
                break;
            }
        }
    }
}

fn part_1(seeds: &Vec<i64>, translations: &Vec<Vec<Translation>>) -> i64 {
    let mut results = seeds.clone();
    for layer in translations {
        process_translations(&mut results, layer);
    }

    *results.iter().min().unwrap()
}

fn part_2(seed_ranges: &Vec<i64>, translations: &Vec<Vec<Translation>>) -> i64 {
    let mut seeds = vec![];
    for i in (0..seed_ranges.len()).step_by(2) {
        for j in seed_ranges[i]..(seed_ranges[i] + seed_ranges[i + 1]) {
            seeds.push(j);
        }
    }

    println!("{:?}", seeds.len());

    for layer in translations {
        process_translations(&mut seeds, layer);
    }

    *seeds.iter().min().unwrap()
}
