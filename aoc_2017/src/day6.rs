use std::{
    collections::HashSet,
    fs::File,
    io::{BufReader, Lines},
};

pub fn day6_part1(mut lines: Lines<BufReader<File>>) -> usize {
    let line = lines.next().unwrap().unwrap();
    let banks: Vec<i32> = line
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let (_, count) = search_array(banks);
    count
}

pub fn day6_part2(mut lines: Lines<BufReader<File>>) -> usize {
    let line = lines.next().unwrap().unwrap();
    let banks: Vec<i32> = line
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let (new_banks, _) = search_array(banks);

    let (_, actual_count) = search_array(new_banks);

    actual_count
}

fn search_array(mut banks: Vec<i32>) -> (Vec<i32>, usize) {
    let len = banks.len();
    let mut seen: HashSet<Vec<i32>> = HashSet::new();

    while seen.insert(banks.clone()) {
        let (mut idx, mut blocks) = banks
            .iter()
            .enumerate()
            .rev()
            .max_by_key(|&(i, v)| v)
            .map(|(i, &v)| (i, v))
            .unwrap();

        banks[idx] = 0;
        while blocks > 0 {
            idx = (idx + 1) % len;
            banks[idx] += 1;
            blocks -= 1;
        }
    }

    (banks, seen.len())
}
