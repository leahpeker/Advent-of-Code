use std::fs::File;
use std::io::{self, BufRead};
use std::vec;

pub fn day1_part1(){
    let file = File::open("/Users/leahpeker/workspace/advent_of_code/aoc_2023/src/day1.txt").expect("Failed to open file");

    let reader = io::BufReader::new(file);

    let mut sum = 0;

    for line in reader.lines(){
        let mut first_digit: i32 = -1;
        let mut second_digit: i32 = -1;
        let result = line.unwrap();
        let reversed_result: Vec<_> = result.chars().rev().collect();
        for (i, c) in result.chars().enumerate() {
            if second_digit == -1 {
                let char = reversed_result.iter().nth(i).unwrap();
                if char.is_digit(10){
                    second_digit = char.to_digit(10).unwrap() as i32;
                }
            }
            if first_digit == -1 {
                if c.is_digit(10){
                    first_digit = c.to_digit(10).unwrap() as i32;
                }
            }
        }
        sum += first_digit * 10 + second_digit;
        }

    println!("Sum: {}", sum);
}

pub fn day1_part2(){
    let file = File::open("/Users/leahpeker/workspace/advent_of_code/aoc_2023/src/day1.txt").expect("Failed to open file");

    let reader = io::BufReader::new(file);

    let mut sum = 0;

    for line in reader.lines(){
        let mut first_digit: i32 = -1;
        let mut second_digit: i32 = -1;
        let result = line.unwrap();
        let reversed_result: Vec<_> = result.chars().rev().collect();

        let list_of_nums = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
        let list_of_rev_nums = vec!["eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin"];

        let mut current_forward_string = String::new();
        let mut current_reverse_string = String::new();

        for (i, c) in result.chars().enumerate() {
            if second_digit == -1 {
                let char = reversed_result.iter().nth(i).unwrap();
                if char.is_digit(10){
                    second_digit = char.to_digit(10).unwrap() as i32;
                } else {
                    current_reverse_string.push(*char);
                    if list_of_rev_nums.contains(&current_reverse_string.as_str()){
                        second_digit = match_word_to_num(current_reverse_string.chars().rev().collect::<String>().as_str()); 
                    } else if list_of_rev_nums.iter().all(|&s| !s.contains(&current_reverse_string)){
                        while !list_of_rev_nums.iter().any(|&s| s.starts_with(current_reverse_string.as_str())){
                            if current_reverse_string.is_empty(){
                                current_reverse_string = c.to_string();
                                break;
                            }
                            current_reverse_string.remove(0);
                        }                    
                    }
                }
            }
            if first_digit == -1 {
                if c.is_digit(10){
                    first_digit = c.to_digit(10).unwrap() as i32;
                } else {
                    current_forward_string.push(c);
                    if list_of_nums.contains(&current_forward_string.as_str()){
                        first_digit = match_word_to_num(current_forward_string.as_str()); 
                    } else if list_of_nums.iter().all(|&s| !s.contains(&current_forward_string)){
                        while !list_of_nums.iter().any(|&s| s.starts_with(current_forward_string.as_str())){
                            if current_forward_string.is_empty(){
                                current_forward_string = c.to_string();
                                break;
                            }
                            current_forward_string.remove(0);
                        }
                    }
                }
            }
        }
        sum += first_digit * 10 + second_digit;
    }
    println!("Sum: {}", sum);
}

fn match_word_to_num(word: &str) -> i32{
    match word {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        &_ => -1,
    }
}