use std::{
    collections::HashSet,
    fs::File,
    io::{BufReader, Lines},
};

pub fn day4_part1(lines: Lines<BufReader<File>>) -> i32 {
    let mut valid_line_count = 0;
    for line in lines {
        let unwrapped_line = line.unwrap();
        let password_list: Vec<_> = unwrapped_line.split_whitespace().collect();
        let password_count: usize = password_list
            .clone()
            .into_iter()
            .collect::<HashSet<_>>()
            .len();
        println!("{}", password_count);
        println!("{}", password_list.len());
        if password_list.len() == password_count {
            valid_line_count += 1;
        }
    }

    valid_line_count
}

pub fn day4_part2(lines: Lines<BufReader<File>>) -> i32 {
    let mut valid_line_count = 0;

    for line in lines {
        let unwrapped_line = line.unwrap();
        let password_list: Vec<_> = unwrapped_line.split_whitespace().collect();

        let password_list_clone = password_list.clone();

        let password_set: HashSet<_> = password_list_clone
            .iter()
            .map(|word| {
                let mut chars: Vec<char> = word.chars().collect();
                chars.sort();
                chars
            })
            .collect();

        if password_list.len() == password_set.len() {
            valid_line_count += 1;
        }
    }

    valid_line_count
}
