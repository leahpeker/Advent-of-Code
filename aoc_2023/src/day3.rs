use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;


pub fn day3_part1(){
    let file = File::open("/Users/leahpeker/workspace/advent_of_code/aoc_2023/src/day3.txt").expect("Failed to open file");

    let reader = io::BufReader::new(file);
    

    let digit_per_pattern = Regex::new(r"[0-9.]").unwrap();


    let mut symbol_bit_matrix: Vec<Vec<u64>> = Vec::new();
    let mut symbol_bit_matrix_left: Vec<Vec<u64>> = Vec::new();
    let mut symbol_bit_matrix_right: Vec<Vec<u64>> = Vec::new();

    let mut num_bit_matrix: Vec<Vec<u64>> = Vec::new();

    let mut num_matrix: Vec<Vec<u64>> = Vec::new();

    for prelim_line in reader.lines(){
        let line = prelim_line.unwrap();

        // first make a binary matrix
        let bit_nums: Vec<u64> = line.chars().map(|c| if c.is_digit(10) {1} else {0}).collect();
        let bit_symbols_0 = digit_per_pattern.replace_all(&line, "0").to_string();
        let bit_symbols: Vec<u64> = bit_symbols_0.chars().map(|c| if c == '0' {0} else {1}).collect();

        let mut bit_symbols_left = bit_symbols.clone();
        bit_symbols_left.insert(0, 0);
        bit_symbols_left.pop();

        let mut bit_symbols_right = bit_symbols.clone();
        bit_symbols_right.push(0);
        bit_symbols_right.remove(0);

        symbol_bit_matrix.push(bit_symbols);
        symbol_bit_matrix_left.push(bit_symbols_left);
        symbol_bit_matrix_right.push(bit_symbols_right);

        num_bit_matrix.push(bit_nums);

        


        // then make a number matrix with the full numbers, not just digits
        let mut num_vec: Vec<u64> = Vec::new();
        let mut current_num = String::new();

        for c in line.chars() {
            if c.is_numeric(){
                current_num.push(c);
            } else {
                if !current_num.is_empty(){
                    for _ in 0..current_num.len(){
                        num_vec.push(current_num.parse::<u64>().unwrap());
                    }
                    current_num.clear();
                }
                // this value doesn't actually matter because it should never be found
                num_vec.push(0);
            }
        }
        if !current_num.is_empty(){
            for _ in 0..current_num.len(){
                num_vec.push(current_num.parse::<u64>().unwrap());
            }
        }
        num_matrix.push(num_vec);
    }

    
    // make the other bit matrices we need
    let check_prev_row_matrix = make_prev_matrix(&symbol_bit_matrix);
    let check_prev_row_matrix_left = make_prev_matrix(&symbol_bit_matrix_left);
    let check_prev_row_matrix_right = make_prev_matrix(&symbol_bit_matrix_right);
    
    
    let check_next_row_matrix = make_next_matrix(&symbol_bit_matrix);
    let check_next_row_matrix_left = make_next_matrix(&symbol_bit_matrix_left);
    let check_next_row_matrix_right = make_next_matrix(&symbol_bit_matrix_right);
    
    // get the unions and intersections we need
    
    let prev_row_and_num_intersection = bitwise_and(&check_prev_row_matrix, &num_bit_matrix); 
    let prev_row_left_int = bitwise_and(&check_prev_row_matrix_left, &num_bit_matrix);
    let prev_row_right_int = bitwise_and(&check_prev_row_matrix_right, &num_bit_matrix);

    // println!(" pre left {:?}", check_prev_row_matrix_right[6]);
    // println!(" num {:?}", num_matrix[6]);
    // println!(" check {:?}", prev_row_right_int[6]);


    let current_row_and_num_intersection = bitwise_and(&symbol_bit_matrix, &num_bit_matrix);
    let current_row_left_int = bitwise_and(&symbol_bit_matrix_left, &num_bit_matrix);
    let current_row_right_int = bitwise_and(&symbol_bit_matrix_right, &num_bit_matrix);

    let next_row_and_num_intersection = bitwise_and(&check_next_row_matrix, &num_bit_matrix);
    let next_row_left_int = bitwise_and(&check_next_row_matrix_left, &num_bit_matrix);
    let next_row_right_int = bitwise_and(&check_next_row_matrix_right, &num_bit_matrix);

    let union1  = bitwise_or(&prev_row_and_num_intersection, &current_row_and_num_intersection);
    let union2 = bitwise_or(&union1, &prev_row_left_int);
    let union3 = bitwise_or(&union2, &prev_row_right_int);
    let union4 = bitwise_or(&union3, &current_row_and_num_intersection);
    let union5 = bitwise_or(&union4, &current_row_left_int);
    let union6 = bitwise_or(&union5, &current_row_right_int);
    let union7 = bitwise_or(&union6, &next_row_and_num_intersection);
    let union8 = bitwise_or(&union7, &next_row_left_int);
    let union9 = bitwise_or(&union8, &next_row_right_int);

    // println!("num union9: {:?}", union9);
    // println!("num num_matrix: {:?}", num_matrix);
    let sum = get_values(&union9, &num_matrix);
    println!("sum: {}", sum)



}

fn make_prev_matrix(matrix: &Vec<Vec<u64>>) -> Vec<Vec<u64>>{
    let mut prev_row_matrix = matrix.clone();
    let new_row: Vec<u64> = vec![0; prev_row_matrix[0].len().try_into().unwrap()];
    prev_row_matrix.insert(0, new_row);
    prev_row_matrix.pop();
    prev_row_matrix
}

fn make_next_matrix(matrix: &Vec<Vec<u64>>) -> Vec<Vec<u64>>{
    let mut next_row_matrix = matrix.clone();
    let new_row: Vec<u64> = vec![0; next_row_matrix[0].len().try_into().unwrap()];
    next_row_matrix.push(new_row);
    next_row_matrix.remove(0);
    next_row_matrix
}

fn bitwise_and(matrix_1: &Vec<Vec<u64>>, matrix_2: &Vec<Vec<u64>>) -> Vec<Vec<u64>>{

    let mut result = vec![vec![0; matrix_1[0].len()]; matrix_1.len()];
    

    for i in 0..matrix_1.len(){
        for j in 0..matrix_1[0].len(){
            result[i][j] = matrix_1[i][j] & matrix_2[i][j];
        }
    }

    result
}

fn bitwise_or(matrix_1: &Vec<Vec<u64>>, matrix_2: &Vec<Vec<u64>>) -> Vec<Vec<u64>>{

    let mut result = vec![vec![0; matrix_1[0].len()]; matrix_1.len()];

    for i in 0..matrix_1.len(){
        for j in 0..matrix_1[0].len(){
            result[i][j] = matrix_1[i][j] | matrix_2[i][j];
        }
    }

    result
}

fn get_values(bin_matrix: &Vec<Vec<u64>>, num_matrix: &Vec<Vec<u64>>) -> u64{

    let mut sum = 0;
    println!("num matrix len: {}, line len: {}", num_matrix.len(), num_matrix[0].len());
    println!("bin matrix len: {}, line len: {}", bin_matrix.len(), bin_matrix[0].len());

    // println!("num matrix first:{:?}", num_matrix[0]);
    
    for i in 0..bin_matrix.len(){
        let mut skip_j = 0;
        for j in 0..bin_matrix[0].len(){
            if bin_matrix[i][j] == 1 {
                if !(j <= skip_j){
                    sum += num_matrix[i][j];
                    skip_j = j + num_matrix[i][j].to_string().len() - 1;
                    println!("num added: {}", num_matrix[i][j]);
                    println!("j: {}, skip j: {}", j, skip_j);
                }
                // return sum
            }
        }
    }

    sum
}