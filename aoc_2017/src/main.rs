use std::fs::File;
use std::io::{self, BufRead, BufReader};
pub mod day1;
pub mod day2;

// use crate::day2::day2_part1;
// use crate::day2::day2_part1_rewrite;
use crate::day2::day2_part2;
// use crate::day2::day2_part2_rewrite;

fn main() {
    // day1_part2();
    // day1_part2_rewrite();

    // day1_part1();
    // day1_part1_rewrite();

    let reader_lines = open_file();
    day2_part2(reader_lines);
}

fn open_file() -> std::io::Lines<BufReader<File>> {
    let file =
        File::open("/Users/leahpeker/workspace/advent_of_code/aoc_2017/src/inputs/day2_input.txt")
            .expect("Failed to open file");
    let reader = io::BufReader::new(file);
    reader.lines()
}
