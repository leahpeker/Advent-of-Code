use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufReader, Lines},
};

pub fn day2_part1(lines: Lines<BufReader<File>>) {
    let mut int_hashmap: HashMap<usize, i32> = HashMap::new();

    for (i, line) in lines.enumerate() {
        let mut ls_hm: HashMap<i32, i32> = HashMap::new();
        ls_hm.insert(0, 0); // 0 represents smallest
        ls_hm.insert(1, 0); // 1 represents largest

        let numbers: Vec<i32> = line
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        for n in numbers {
            let curr_small = ls_hm.get_mut(&0).unwrap();
            if n < *curr_small || *curr_small == 0 {
                *curr_small = n;
            }

            let curr_larg = ls_hm.get_mut(&1).unwrap();
            if n > *curr_larg {
                *curr_larg = n;
            }
        }

        let smallest = ls_hm.get(&0).unwrap();
        let largest = ls_hm.get(&1).unwrap();

        int_hashmap.insert(i, *largest - *smallest);
    }

    let sum: i32 = int_hashmap.values().sum();
    println!("{}", sum)
}

pub fn day2_part2(lines: Lines<BufReader<File>>) {
    let mut int_hashmap: HashMap<usize, i32> = HashMap::new();

    let mut factors_sets: HashMap<i32, HashSet<i32>> = HashMap::new();

    for (i, line) in lines.enumerate() {
        let numbers: Vec<i32> = line
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let numbers_set: HashSet<i32> = HashSet::from_iter(numbers.clone());

        for n in numbers {
            let factors_set = get_factors(n, &mut factors_sets);
            let overlap_set: Vec<i32> = numbers_set.intersection(&factors_set).copied().collect();

            if overlap_set.len() == 1 {
                int_hashmap.insert(i, (n / overlap_set[0]) as i32);
                break;
            }
        }
    }

    let sum: i32 = int_hashmap.values().sum();
    println!("{}", sum)
}

fn get_factors(n: i32, factors_sets: &mut HashMap<i32, HashSet<i32>>) -> HashSet<i32> {
    if factors_sets.contains_key(&n) {
        factors_sets.get(&n).unwrap().clone()
    } else {
        let mut set = HashSet::new();

        for i in (2..=(n / 2)).rev() {
            if !set.contains(&i) && n % i == 0 {
                set.insert(i);
                set.insert(n / i as i32);

                if factors_sets.contains_key(&i) {
                    set.extend(factors_sets.get(&i).unwrap());
                }
            }
        }

        factors_sets.insert(n, set.clone());
        set
    }
}
