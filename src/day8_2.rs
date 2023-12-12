use std::collections::HashSet;

use crate::day8_1;

pub fn run(inp: String) -> String {
    let split: Vec<&str> = inp.split('\n').collect();
    let directions = split.get(0).unwrap().to_string();
    let nodes = day8_1::parse_nodes(split);
    
    let mut starting_locations = HashSet::new();
    for ele in &nodes {
        if ele.0.ends_with('A') {
            starting_locations.insert(ele.0.clone());
        }
    }

    let mut greatest_common_divisor = 0;
    for ele in starting_locations {
        let mut current_location = ele;
        let mut steps = 0;
        'walktrough: loop {
            for c in directions.chars() {
                steps += 1;
                let node = nodes.get(&current_location).unwrap();
                let mut new_value = &node.right;
                if c == 'L' {
                    new_value = &node.left;
                }
                if new_value.ends_with('Z') {
                    if greatest_common_divisor == 0 {
                        greatest_common_divisor = steps;
                    } else {
                        greatest_common_divisor = greatest_common_divisor * steps / greatest_possible_divisor(greatest_common_divisor, steps);
                    }
                    break 'walktrough
                }
                current_location = new_value.to_string();
            }
        }
    }

    return format!("{}", greatest_common_divisor);
}

pub fn greatest_possible_divisor(first: usize, second: usize) -> usize {
    if first == 0 {
        return second
    }
    if second > first {
        return greatest_possible_divisor(second % first, first)
    }
    return greatest_possible_divisor(first % second, second)
}