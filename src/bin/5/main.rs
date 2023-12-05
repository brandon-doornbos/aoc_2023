struct Translation {
    source: i64,
    destination: i64,
    length: i64,
}

struct Range {
    start: i64,
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

    iter.next();
    iter.next();

    let mut translations: Vec<Vec<Translation>> = vec![vec![]];

    for line in iter {
        if line.starts_with(|c: char| c.is_ascii_digit()) {
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
        } else if line == "" {
            translations.push(vec![]);
        }
    }

    (seeds, translations)
}

fn process_translations_1(numbers: &mut Vec<i64>, translations: &Vec<Translation>) {
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
        process_translations_1(&mut results, layer);
    }

    *results.iter().min().unwrap()
}

fn process_translations_2(ranges: &mut Vec<Range>, translations: &Vec<Translation>) {
    let mut result = vec![];

    while !ranges.is_empty() {
        for i in (0..ranges.len()).rev() {
            let mut not_removed = true;

            for translation in translations {
                let translation_end = translation.source + translation.length;

                if ranges[i].start >= translation.source && ranges[i].start < translation_end {
                    let range_end = ranges[i].start + ranges[i].length;

                    if range_end > translation_end {
                        ranges.push(Range {
                            start: translation_end,
                            length: range_end - translation_end,
                        });
                        ranges[i].length = translation_end - ranges[i].start;
                    }

                    ranges[i].start += translation.destination - translation.source;
                    result.push(ranges.swap_remove(i));
                    not_removed = false;
                    break;
                }
            }
            if not_removed {
                result.push(ranges.swap_remove(i));
            }
        }
    }

    ranges.append(&mut result);
}

fn part_2(seeds: &Vec<i64>, translations: &Vec<Vec<Translation>>) -> i64 {
    let mut seed_ranges = vec![];
    for i in (0..seeds.len()).step_by(2) {
        seed_ranges.push(Range {
            start: seeds[i],
            length: seeds[i + 1],
        });
    }

    for layer in translations {
        process_translations_2(&mut seed_ranges, layer);
    }

    seed_ranges.sort_unstable_by(|a, b| a.start.cmp(&b.start));
    seed_ranges[0].start
}
