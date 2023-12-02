use std::collections::HashMap;


pub fn run(inp: String) -> String {
    let mut valid_game_result = 0;
    let games = inp.split('\n');

    for game in games {
        let parsed = parse(game);
        let mut valid = true;
        for ele in parsed.draws {
            if ele.get(&Color::Red).unwrap_or(&0) > &12 || ele.get(&Color::Green).unwrap_or(&0) > &13 || ele.get(&Color::Blue).unwrap_or(&0) > &14 {
                valid = false;
                break;
            }
        }
        if valid {
            valid_game_result += parsed.id;
        }
    }

    return format!("{}", valid_game_result);
}

pub fn parse(inp: &str) -> Game {
    let split: Vec<&str> = inp.split(':').collect();
    let first_part: Vec<&str> = split.get(0).unwrap().split(" ").collect();
    let id = first_part.get(1).unwrap();
    let all_draws = split.get(1).unwrap();
    
    return Game { id: id.parse().unwrap(), draws: parse_draws(all_draws) }
}

fn parse_draws(inp: &str) -> Vec<HashMap<Color, usize>> {
    let draws: Vec<&str> = inp.split(';').collect();
    let mut result = Vec::new();
    for draw in draws {
        let counts_per_color: Vec<&str> = draw.split(',').collect();
        let mut this_draw = HashMap::with_capacity(counts_per_color.len());
        for ele in counts_per_color {
            let mut split = ele.trim().split(' ');
            let count = split.next().unwrap().parse().unwrap();
            let color_string = split.next().unwrap().trim();
            this_draw.insert(str_to_color(color_string), count);
        }
        result.push(this_draw);
    }

    return result;
}

pub struct Game {
    pub id: usize,
    pub draws: Vec<HashMap<Color, usize>>
}

#[derive(Eq, Hash, PartialEq)]
pub enum Color {
    Blue,
    Red,
    Green,
}

fn str_to_color(inp: &str) -> Color {
    if inp == "blue" {
        return Color::Blue;
    } if inp == "red" {
        return Color::Red
    }if inp == "green" {
        return Color::Green
    }
    panic!("no no no")
}