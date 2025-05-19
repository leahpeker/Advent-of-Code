use std::fs::read_to_string;
use std::error;
use regex::Regex;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub fn day5_part1() -> Result<()>{
    let contents = read_to_string("/Users/leahpeker/workspace/advent_of_code/aoc_2023/src/day5.txt").unwrap();
    
    let re = Regex::new(r"seeds:\s*([\d\s]+)|seed-to-soil map:\s*([\d\s]+)|soil-to-fertilizer map:\s*([\d\s]+)|fertilizer-to-water map:\s*([\d\s]+)|water-to-light map:\s*([\d\s]+)|light-to-temperature map:\s*([\d\s]+)|temperature-to-humidity map:\s*([\d\s]+)|humidity-to-location map:\s*([\d\s]+)")?;

    let cap_iter = re.captures_iter(&contents);

    let mut seeds: Vec<i64> = Vec::new();

    for (cap_i, cap) in cap_iter.enumerate() {
        let string = &cap[cap_i+1];

        println!("seeds: {:?}", seeds);
        if cap_i == 0 {
            seeds = string.split_whitespace()
            .flat_map(|num| num.parse::<i64>().ok())
            .collect();
        } else {
            let lines: Vec<Vec<i64>> = string
            .lines()
            .filter(|&line| !line.trim().is_empty())
            .map(|line| {
                line.split_whitespace()
                    .flat_map(|num| num.parse::<i64>().ok())
                    .collect()
            })
            .collect();

            // println!("lines: {:?}", lines);
            for seed in seeds.iter_mut() {

                for line in &lines {
                    let range_start = line[1];
                    let range_end = range_start + line[2];
                    let diff = line[0] - range_start;

                    if (range_start..range_end).contains(&seed) {
                        *seed += diff;
                        break;
                    }
                }
            }
        }


    }
    println!("seeds: {:?}", seeds);
    println!("lowest value: {:?}", seeds.iter().min());

    Ok(())
}


pub fn day5_part2() -> Result<()>{
    let contents = read_to_string("/Users/leahpeker/workspace/advent_of_code/aoc_2023/src/day5.txt").unwrap();
    
    let re = Regex::new(r"seeds:\s*([\d\s]+)|seed-to-soil map:\s*([\d\s]+)|soil-to-fertilizer map:\s*([\d\s]+)|fertilizer-to-water map:\s*([\d\s]+)|water-to-light map:\s*([\d\s]+)|light-to-temperature map:\s*([\d\s]+)|temperature-to-humidity map:\s*([\d\s]+)|humidity-to-location map:\s*([\d\s]+)")?;

    let cap_iter = re.captures_iter(&contents);

    let mut seeds: Vec<i64> = Vec::new();

    for (cap_i, cap) in cap_iter.enumerate() {
        let string = &cap[cap_i+1];

        if cap_i == 0 {
            let initial_seed_list: Vec<i64> = string.split_whitespace()
            .flat_map(|num| num.parse::<i64>().ok())
            .collect();

            for (_index, seed_index) in (0..initial_seed_list.len()).step_by(2).enumerate() {
                let seed_start = initial_seed_list[seed_index];
                let seed_end = initial_seed_list[seed_index + 1] + seed_start;
                
                for new_seed in seed_start..seed_end{
                    seeds.push(new_seed);
                }
                
            }
            // println!("seeds: {:?}", seeds);
        } else {
            let lines: Vec<Vec<i64>> = string
            .lines()
            .filter(|&line| !line.trim().is_empty())
            .map(|line| {
                line.split_whitespace()
                    .flat_map(|num| num.parse::<i64>().ok())
                    .collect()
            })
            .collect();

            // println!("lines: {:?}", lines);
            for seed in seeds.iter_mut() {                

                for line in &lines {
                    let range_start = line[1];
                    let range_end = range_start + line[2];
                    let diff = line[0] - range_start;

                    if (range_start..range_end).contains(&seed) {
                        *seed += diff;
                        break;
                    }
                }
            }
        }


    }
    println!("lowest value: {:?}", seeds.iter().min());

    Ok(())
}

