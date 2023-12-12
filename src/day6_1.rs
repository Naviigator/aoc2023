use std::collections::HashSet;

pub fn run(inp: String) -> String {
    let split: Vec<&str> = inp.split('\n').collect();

    let times = parse_numbers(split.get(0).unwrap());
    let distances = parse_numbers(split.get(1).unwrap());
    let mut score = 1;

    for i in 0..times.len() {
        let mut session_score = 0;
        let time = times.get(i).unwrap().clone();
        let distance_to_beat = distances.get(i).unwrap().clone();
        for speed in 1..time {
            let distance = speed * (time - speed);
            if distance > distance_to_beat {
                session_score += 1;
            }
        }
        score *= session_score;
    }

    return format!("{}", score);
}

pub fn parse_numbers(inp: &str) -> Vec<usize> {
    return inp.trim().split(" ").filter_map(|i| i.parse().ok()).collect();
}

pub struct Card {
    pub id: usize,
    pub winning_numbers: HashSet<usize>,
    pub your_numbers: HashSet<usize>,
}