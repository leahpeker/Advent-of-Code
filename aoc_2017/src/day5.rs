use std::{
    fs::File,
    io::{BufReader, Lines},
};

pub fn day5_part1(lines: Lines<BufReader<File>>) -> i32 {
    let mut i: usize = 0;
    let mut step_count = 0;

    let mut step_vector: Vec<i32> = lines
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();

    let vec_length = step_vector.len();

    loop {
        step_count += 1;

        let steps = step_vector[i];
        let next_spot = steps + i as i32;

        if next_spot as usize >= vec_length {
            break;
        }

        step_vector[i] += 1;
        i = next_spot as usize;
    }

    step_count
}

pub fn day5_part2(lines: Lines<BufReader<File>>) -> i32 {
    let mut i: usize = 0;
    let mut step_count = 0;

    let mut step_vector: Vec<i32> = lines
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();

    let vec_length = step_vector.len();

    loop {
        step_count += 1;

        let steps = step_vector[i];
        let next_spot = steps + i as i32;

        if next_spot as usize >= vec_length {
            break;
        }

        if steps >= 3 {
            step_vector[i] -= 1;
        } else {
            step_vector[i] += 1;
        }
        i = next_spot as usize;
    }

    step_count
}
