fn main() {
    let input = include_str!("input.txt")
        .trim()
        .lines()
        .map(|s| s.split_whitespace().map(|s| s.parse().unwrap()).collect());

    println!("Part 1: {}", input.clone().map(next).sum::<i32>());
    println!("Part 2: {}", input.map(previous).sum::<i32>());
}

fn find_diff(seq: &Vec<i32>, last: &mut Vec<i32>, vec_op: fn(&Vec<i32>) -> i32) -> Vec<i32> {
    if seq.iter().all(|x| *x == 0) {
        return vec![];
    }
    let mut new = vec![0; seq.len() - 1];
    for i in 0..(seq.len() - 1) {
        new[i] = seq[i + 1] - seq[i];
    }
    last.push(vec_op(&new));
    find_diff(&new, last, vec_op)
}

fn next(seq: Vec<i32>) -> i32 {
    let mut last = vec![*seq.last().unwrap()];
    find_diff(&seq, &mut last, |vec| *vec.last().unwrap());
    last.iter().sum()
}

fn previous(seq: Vec<i32>) -> i32 {
    let mut first = vec![seq[0]];
    find_diff(&seq, &mut first, |vec| vec[0]);
    first.iter().rev().fold(0, |acc, x| x - acc)
}
