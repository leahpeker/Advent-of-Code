use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

pub fn day2_part1(){
    let file = File::open("/Users/leahpeker/workspace/advent_of_code/aoc_2023/src/day2.txt").expect("Failed to open file");

    let reader = io::BufReader::new(file);

    let sum = reader.lines().fold(0, |acc: i32, line| {
        let result = line.unwrap();

        let red_indices: Vec<_> = result.match_indices("red").map(|(index, _)| index - 3).collect();
        let green_indices: Vec<_> = result.match_indices("green").map(|(index, _)| index - 3).collect();
        let blue_indices: Vec<_> = result.match_indices("blue").map(|(index, _)| index - 3).collect();

        let red_no = red_indices.iter().any(|index| {
            if result.chars().nth(*index).unwrap().to_digit(10).is_some() {
                result.chars().nth(*index).unwrap().to_digit(10).unwrap() * 10 + result.chars().nth(*index + 1).unwrap().to_digit(10).unwrap() > 12
            } else {
                false
            }
        });
        let green_no = green_indices.iter().any(|index| {
            if result.chars().nth(*index).unwrap().to_digit(10).is_some() {
                result.chars().nth(*index).unwrap().to_digit(10).unwrap() * 10 + result.chars().nth(*index + 1).unwrap().to_digit(10).unwrap() > 13
            } else {
                false
            }
        });
        let blue_no = blue_indices.iter().any(|index| {
            if result.chars().nth(*index).unwrap().to_digit(10).is_some() {
                result.chars().nth(*index).unwrap().to_digit(10).unwrap() * 10 + result.chars().nth(*index + 1).unwrap().to_digit(10).unwrap() > 14
            } else {
                false
            }
        });

        if red_no || green_no || blue_no {
            return acc;
        }
        println!("red_no: {}, green_no: {}, blue_no: {}", red_no, green_no, blue_no);

        let number_pattern = Regex::new(r"\d+").unwrap();
        if let Some(num) = number_pattern.find(&result) {
            acc + num.as_str().parse::<i32>().unwrap()
        } else {
            acc
        }

    });

    println!("Day 2 Part 1: {}", sum)
}

pub fn day2_part2(){
    let file = File::open("/Users/leahpeker/workspace/advent_of_code/aoc_2023/src/day2.txt").expect("Failed to open file");

    let reader = io::BufReader::new(file);

    let sum = reader.lines().fold(0, |acc: i32, line| {
        let result = line.unwrap();

        let mut red_count = 0;
        let mut green_count = 0;
        let mut blue_count = 0;

        let result_list: Vec<String> = result.split_whitespace().map(|s| s.replace(",", "").replace(";", "")).collect();
        result_list.iter().enumerate().for_each(|(index, item)| {
            if item == "red" && result_list[index - 1].parse::<i32>().unwrap() > red_count {
                red_count = result_list[index - 1].parse::<i32>().unwrap();
            } else if item == "green" && result_list[index - 1].parse::<i32>().unwrap() > green_count {
                green_count = result_list[index - 1].parse::<i32>().unwrap();
            } else if item == "blue" && result_list[index - 1].parse::<i32>().unwrap() > blue_count {
                blue_count = result_list[index - 1].parse::<i32>().unwrap();
            } 
        });

        acc + red_count * green_count * blue_count
    });
    println!("Sum: {}", sum)
    }
