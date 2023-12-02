use std::collections::HashMap;

use crate::day2_1;


pub fn run(inp: String) -> String {
    let mut result = 0;
    let games = inp.split('\n');

    for game in games {
        let parsed = day2_1::parse(game);
        let mut max = HashMap::new();
        for ele in parsed.draws {
            for single_ele in ele {
                let current = max.get(&single_ele.0).unwrap_or(&0);
                if current < &single_ele.1 {
                    max.insert(single_ele.0, single_ele.1);
                }
            }
        }
        let mut game_result = 1;
        for ele in max {
            game_result *= ele.1;
        }
        result += game_result;
    }

    return format!("{}", result);
}
