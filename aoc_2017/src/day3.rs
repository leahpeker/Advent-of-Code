use round_to::*;
use std::collections::HashMap;

pub fn day3_part1(number: i32) -> i32 {
    let n = (((number as f64).sqrt().ceil() + 1.0) / 2.0).ceil_to_i32();
    let grid_size = 2 * n - 1;
    let min = (grid_size - 2).pow(2) + 1;

    let mut steps = n - 1;

    let corners = [
        min + grid_size - 2,
        min + grid_size * 2 - 3,
        min + grid_size * 3 - 4,
        grid_size.pow(2),
    ];

    for i in 0..4 {
        let prev_corner;
        if i == 0 {
            prev_corner = min;
        } else {
            prev_corner = corners[i - 1];
        }

        let current_corner = corners[i];

        if number == current_corner {
            steps += grid_size / 2;
            break;
        } else if number < current_corner {
            let mid_point = (current_corner - prev_corner) / 2 + prev_corner;
            steps += (number - mid_point).abs();
            break;
        }
    }

    steps
}

pub fn day3_part2(number: i32) -> i32 {
    let mut spiral: HashMap<(i32, i32), i32> = HashMap::new();

    spiral.insert((0, 0), 1);

    let mut n = 1;
    let mut prev_node = (0, 0);

    loop {
        let mut curr_node: (i32, i32) = (prev_node.0 + 1, prev_node.1);
        let node_value = get_node_value(curr_node, &spiral);
        if node_value > number {
            return node_value;
        }
        spiral.insert(curr_node.clone(), node_value);

        n += 1;
        let grid_size: i32 = 2 * n - 1;

        let start_y = curr_node.1 + 1;
        for y in start_y..(start_y + grid_size - 2) {
            // go up
            curr_node = (curr_node.0, y);

            let node_value = get_node_value(curr_node, &spiral);
            if node_value > number {
                return node_value;
            }

            spiral.insert(curr_node.clone(), node_value);
        }

        let start_x = curr_node.0;
        let left_x = start_x - (grid_size - 1);
        for x in (left_x..start_x).rev() {
            // go left
            curr_node = (x, curr_node.1);

            let node_value = get_node_value(curr_node, &spiral);
            if node_value > number {
                return node_value;
            }

            spiral.insert(curr_node.clone(), node_value);
        }

        let start_y = curr_node.1;
        let bottom_y = start_y - (grid_size - 1);
        for y in (bottom_y..start_y).rev() {
            // go down
            curr_node = (curr_node.0, y);

            let node_value = get_node_value(curr_node, &spiral);
            if node_value > number {
                return node_value;
            }

            spiral.insert(curr_node.clone(), node_value);
        }

        let start_x = curr_node.0 + 1;
        for x in start_x..(start_x + (grid_size - 1)) {
            // go right
            curr_node = (x, curr_node.1);

            let node_value = get_node_value(curr_node, &spiral);
            if node_value > number {
                return node_value;
            }

            spiral.insert(curr_node.clone(), node_value);
        }

        prev_node = curr_node.clone();
    }
}

fn get_node_value(curr_node: (i32, i32), spiral: &HashMap<(i32, i32), i32>) -> i32 {
    let mut node_value: i32 = 0;

    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let node = (&curr_node.0 + dx, &curr_node.1 + dy);
            node_value += spiral.get(&node).unwrap_or(&0);
        }
    }

    node_value
}
