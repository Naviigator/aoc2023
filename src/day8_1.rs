use std::collections::HashMap;

pub fn run(inp: String) -> String {
    let split: Vec<&str> = inp.split('\n').collect();
    let directions = split.get(0).unwrap().to_string();
    let nodes = parse_nodes(split);
    
    let mut steps = 0;
    let mut current = "AAA".to_string();
    'walktrough: loop {
        for ele in directions.chars() {
            let node = nodes.get(&current).unwrap();
            steps += 1;
            if ele == 'L' {
                current = node.left.clone();
            } else {
                current = node.right.clone();
            }
            if current == "ZZZ" {
                break 'walktrough;
            }
        }
    }

    return format!("{}", steps);
}

pub fn parse_nodes(inp: Vec<&str>) -> HashMap<String, LeftAndRight> {
    let mut result = HashMap::new();
    for ele in inp.iter().skip(2) {
        let split: Vec<&str> = ele.split('=').collect();
        let key = split.get(0).unwrap().trim().to_string();
        let left_and_right = parse_left_and_right(split.get(1).unwrap());
        result.insert(key, left_and_right);
    }

    result
}

pub fn parse_left_and_right(inp: &str) -> LeftAndRight {
    let x: &[_] = &['(', ')', ' '];
    let split: Vec<&str> = inp.trim_matches(x).split(",").map(|x| x.trim()).collect();
    return LeftAndRight{
        left: split.get(0).unwrap().to_string(),
        right: split.get(1).unwrap().to_string(),
    }
}

pub struct LeftAndRight {
    pub left: String,
    pub right: String,
}