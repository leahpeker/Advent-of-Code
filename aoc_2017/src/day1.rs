use std::fs::File;
use std::io::{self, BufRead};

pub fn day1_part1(){
    let file = File::open("/Users/leahpeker/workspace/advent_of_code/aoc_2017/src/day1_input.txt").expect("Failed to open file");
    let reader = io::BufReader::new(file);

    for line in reader.lines(){
        let og_line_text = line.unwrap();
        let first_digit = og_line_text.chars().nth(0).unwrap();
        let mut appended_line_text = og_line_text.clone();
        appended_line_text.push(first_digit);

        let line_digits = og_line_text.chars();

        let mut sum = 0;

        for (i, digit) in line_digits.clone().enumerate(){
            if digit == appended_line_text.chars().nth(i+1).unwrap(){
                sum += digit.to_digit(10).unwrap();
            }
        }
        println!("{}", sum)

    }
}



pub fn day1_part1_rewrite(){
    let file = File::open("/Users/leahpeker/workspace/advent_of_code/aoc_2017/src/day1_input.txt").expect("Failed to open file");
    let reader = io::BufReader::new(file);

    for line in reader.lines(){
        let line_text  = line.unwrap();
        let chars = line_text.chars();
        let next_chars = line_text.chars().cycle().skip(1);

        let sum: u32 = chars.zip(next_chars).filter(|(a,b) | a == b).map(|(c,_)| c.to_digit(10).unwrap()).sum();
        

        println!("{}", sum)

    }
}

pub fn day1_part2(){
    let file = File::open("/Users/leahpeker/workspace/advent_of_code/aoc_2017/src/day1_input.txt").expect("Failed to open file");
    let reader = io::BufReader::new(file);

    for line in reader.lines(){
        let og_line_text = line.unwrap();
        let mut appended_line_text = og_line_text.clone();
        appended_line_text += &og_line_text;

        let line_length = og_line_text.len();
        let steps_forward = line_length/2;


        let line_digits = og_line_text.chars();

        let mut sum = 0;

        for (i, digit) in line_digits.enumerate(){
            if digit == appended_line_text.chars().nth(i+steps_forward).unwrap(){
                sum += digit.to_digit(10).unwrap();
            }
        }
        println!("{}", sum)
    }
}

pub fn day1_part2_rewrite(){
    let file = File::open("/Users/leahpeker/workspace/advent_of_code/aoc_2017/src/day1_input.txt").expect("Failed to open file");
    let reader = io::BufReader::new(file);

    for line in reader.lines(){
        let line_text  = line.unwrap();
        let len = line_text.len();
        let chars = line_text.chars();
        let next_chars = line_text.chars().cycle().skip(len/2);

        let sum: u32 = chars.zip(next_chars).filter(|(a,b) | a == b).map(|(c,_)| c.to_digit(10).unwrap()).sum();
        

        println!("{}", sum)

    }
}