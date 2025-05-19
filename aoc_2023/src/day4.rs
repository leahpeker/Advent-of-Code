use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;


pub fn day4_part1(){
    let file = File::open("/Users/leahpeker/workspace/advent_of_code/aoc_2023/src/day4.txt").expect("Failed to open file");

    let reader = io::BufReader::new(file);

    
    let my_nums_pattern = match Regex::new(r":\s*(\d+\b.*?\|)"){
        Ok(regex) => regex,
        Err(_err) => {
            eprintln!("Error");
            return;
        }
    };


    let winning_nums_pattern = match Regex::new(r"\|\s*(\d+(?:\s+\d+)*)\b"){
        Ok(regex) => regex,
        Err(_err) => {
            eprintln!("Error");
            return;
        }
    };

    let n = reader.lines().fold(0, |acc:i32, line| {
            let result = line.unwrap();

            let my_nums: HashSet<i32> = my_nums_pattern.captures_iter(&result)
            .flat_map(|capture| capture.get(1))
            .flat_map(|mat| {
                mat.as_str().split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
                .collect::<Vec<i32>>()
            })
            .collect();

            let winning_numbers: HashSet<i32> = winning_nums_pattern.captures_iter(&result)
            .flat_map(|captures| captures.get(1))
            .flat_map(|matched| {
                matched.as_str().split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
            })
            .collect();


            let win_count = winning_numbers.intersection(&my_nums).cloned().collect::<HashSet<i32>>().len();

            let card_value = if win_count > 0 {
                2usize.pow((win_count - 1) as u32)
            } else {
                0
            };
            
            acc + card_value as i32
        });

    
    println!("total: {}", n);

}


pub fn day4_part2(){
    let file = File::open("/Users/leahpeker/workspace/advent_of_code/aoc_2023/src/day4.txt").expect("Failed to open file");

    let reader = io::BufReader::new(file);

    let card_num_pattern = Regex::new(r"\b(\d+)\b").unwrap();

    let my_nums_pattern = match Regex::new(r":\s*(\d+\b.*?\|)"){
        Ok(regex) => regex,
        Err(_err) => {
            eprintln!("Error");
            return;
        }
    };


    let winning_nums_pattern = match Regex::new(r"\|\s*(\d+(?:\s+\d+)*)\b"){
        Ok(regex) => regex,
        Err(_err) => {
            eprintln!("Error");
            return;
        }
    };

    let mut copies_dict: HashMap<i32, i32> = HashMap::new();

    let n = reader.lines().fold(0, |acc:i32, line| {
            let result = line.unwrap();

            let card_num: i32 = card_num_pattern
            .captures(&result)
            .and_then(|captures| captures.get(1))
            .map(|number| number.as_str().parse::<i32>().unwrap_or_default())
            .unwrap_or_default();

            let my_nums: HashSet<i32> = my_nums_pattern.captures_iter(&result)
            .flat_map(|capture| capture.get(1))
            .flat_map(|mat| {
                mat.as_str().split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
                .collect::<Vec<i32>>()
            })
            .collect();

            let winning_numbers: HashSet<i32> = winning_nums_pattern.captures_iter(&result)
            .flat_map(|captures| captures.get(1))
            .flat_map(|matched| {
                matched.as_str().split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
            })
            .collect();


            let win_count = winning_numbers.intersection(&my_nums).cloned().collect::<HashSet<i32>>().len();

            let current_card_count = *copies_dict.entry(card_num).or_insert(1);

            for i in 1..win_count + 1 {
                let key = card_num + i as i32;
                let entry = copies_dict.entry(key).or_insert(1);
                *entry += current_card_count;
            }



            println!("current card count: {}", current_card_count);
            
            acc + current_card_count
        });

    
    println!("total: {}", n);

}