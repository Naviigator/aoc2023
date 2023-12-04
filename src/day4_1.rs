use std::collections::HashSet;

pub fn run(inp: String) -> String {
    let mut score = 0;
    let card_strs = inp.split('\n');

    for card_str in card_strs {
        let mut points = 0;
        let card = parse(card_str);
        for ele in card.your_numbers {
            if card.winning_numbers.contains(&ele) {
                points = if points == 0 {
                    1
                } else {
                    points * 2
                }
            }
        }
        score += points;
    }

    return format!("{}", score);
}

pub fn parse(inp: &str) -> Card {
    let mut inp_one_space = inp.replace("  ", " ");
    while inp_one_space.contains("  ") {
        inp_one_space = inp_one_space.replace("  ", " ");
    }
    let split: Vec<&str> = inp_one_space.split(':').collect();
    let first_part: Vec<&str> = split.get(0).unwrap().split(" ").collect();
    let id = first_part.get(1).unwrap();
    let all_numbers: Vec<&str> = split.get(1).unwrap().split("|").collect();
    
    return Card {
        id: id.parse().unwrap(),
        winning_numbers: parse_numbers(all_numbers.get(0).unwrap()),
        your_numbers: parse_numbers(all_numbers.get(1).unwrap()),
    }
}

pub fn parse_numbers(inp: &str) -> HashSet<usize> {
    return inp.trim().split(" ").map(|i| i.parse().unwrap()).collect();
}

pub struct Card {
    pub id: usize,
    pub winning_numbers: HashSet<usize>,
    pub your_numbers: HashSet<usize>,
}