use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;
pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

// use crate::day2::day2_part1;
// use crate::day2::day2_part1_rewrite;
// use crate::day2::day2_part2;
// use crate::day2::day2_part2_rewrite;
// use crate::day3::day3_part1;
// use crate::day3::day3_part2;
// use crate::day4::day4_part1;
// use crate::day4::day4_part2;
// use crate::day5::day5_part1;
// use crate::day5::day5_part2;
use crate::day6::{day6_part1, day6_part2};

fn main() {
    // day1_part2();
    // day1_part2_rewrite();

    // day1_part1();
    // day1_part1_rewrite();

    // let reader_lines = open_file();
    // day2_part2(reader_lines);

    // let steps = day3_part1(312051);
    // println!("{}", steps);

    // let value = day3_part2(312051);
    // println!("value: {}", value);

    let now = Instant::now();

    let reader_lines = open_file();
    // let value = day4_part2(reader_lines);
    // println!("value: {}", value);

    // let value = day5_part2(reader_lines);
    // let value = day6_part1(reader_lines);
    // println!("value: {}", value);

    let value2 = day6_part2(reader_lines);
    println!("value: {}", value2);

    let elapsed = now.elapsed();
    println!("Elapsed time: {:.2?}", elapsed);
}

fn open_file() -> std::io::Lines<BufReader<File>> {
    let file =
        File::open("/Users/leahpeker/workspace/advent_of_code/aoc_2017/src/inputs/day6_input.txt")
            .expect("Failed to open file");
    let reader = io::BufReader::new(file);
    reader.lines()
}
