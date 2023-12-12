pub fn run(inp: String) -> String {
    let replaced = inp.replace(' ', "");
    let split: Vec<&str> = replaced.split('\n').collect();

    let times = parse_numbers(split.get(0).unwrap());
    let distances = parse_numbers(split.get(1).unwrap());

    let time = times.get(0).unwrap().clone();
    let distance_to_beat = distances.get(0).unwrap().clone();
    let starting_point = time / 2;
    let best_distance = starting_point * (time-starting_point);
    let mut distance_sub = 0;
    let mut add_to_sub = 1;
    let mut score = 0;
    for _ in starting_point + 1..time {
        distance_sub += add_to_sub;
        add_to_sub += 2;
        if best_distance - distance_sub > distance_to_beat {
            score += 1;
        } else {
            break;
        }
    }
    score *= 2;//skipped half in our calculation, a*b = b*a
    score += 1;//skipped our initial hit

    return format!("{}", score);
}

pub fn parse_numbers(inp: &str) -> Vec<usize> {
    return inp.trim().split(":").filter_map(|i| i.parse().ok()).collect();
}
