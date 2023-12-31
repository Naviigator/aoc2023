pub mod day1_1;
pub mod day1_2;
pub mod day2_1;
pub mod day2_2;
pub mod day3_1;
pub mod day3_2;
pub mod day4_1;
pub mod day4_2;
pub mod day5_1;
pub mod day5_2;
pub mod day6_1;
pub mod day6_2;
pub mod day7_1;
pub mod day7_2;
pub mod day8_1;
pub mod day8_2;
pub mod day9_1;
pub mod day9_2;
pub mod day10_1;
pub mod day10_2;
pub mod day11_1;
pub mod day11_2;

use std::collections::HashMap;

fn main() {
    let mut day_fns: HashMap<String, fn(String) -> String> = HashMap::new();
    day_fns.insert("1.1".to_string(), day1_1::run);
    day_fns.insert("1.2".to_string(), day1_2::run);
    day_fns.insert("2.1".to_string(), day2_1::run);
    day_fns.insert("2.2".to_string(), day2_2::run);
    day_fns.insert("3.1".to_string(), day3_1::run);
    day_fns.insert("3.2".to_string(), day3_2::run);
    day_fns.insert("4.1".to_string(), day4_1::run);
    day_fns.insert("4.2".to_string(), day4_2::run);
    day_fns.insert("5.1".to_string(), day5_1::run);
    day_fns.insert("5.2".to_string(), day5_2::run);
    day_fns.insert("6.1".to_string(), day6_1::run);
    day_fns.insert("6.2".to_string(), day6_2::run);
    day_fns.insert("7.1".to_string(), day7_1::run);
    day_fns.insert("7.2".to_string(), day7_2::run);
    day_fns.insert("8.1".to_string(), day8_1::run);
    day_fns.insert("8.2".to_string(), day8_2::run);
    day_fns.insert("9.1".to_string(), day9_1::run);
    day_fns.insert("9.2".to_string(), day9_2::run);
    day_fns.insert("10.1".to_string(), day10_1::run);
    day_fns.insert("10.2".to_string(), day10_2::run);
    day_fns.insert("11.1".to_string(), day11_1::run);
    day_fns.insert("11.2".to_string(), day11_2::run);

    println!("What day to solve? EG: 1.1, 1.2,...");
    let mut day_string: String = String::new();
    _ = std::io::stdin().read_line(&mut day_string).unwrap();
    day_string = day_string.trim().to_string();
    let func = day_fns.get(&day_string).unwrap();
    
    println!("No crash? I'm impressed, you can enter a proper input. Enter your challenge. When done - please add 2 empty newlines. Let's hope no challenges use 2 empty newlines because then I will need to think...");
    let mut challenge: String = String::new();
    let mut input: String = String::new();
    let mut last_empty = false;
    loop {
        input.clear();
        _ = std::io::stdin().read_line(&mut input).unwrap();
        challenge.push_str(input.replace("\r", "").as_str());
        if input.trim().is_empty() {
            if last_empty {
                break;
            }
            last_empty = true;
            continue;
        }
        last_empty = false;
    }
    challenge = challenge.trim_end_matches("\n").to_string();
    println!("{}", func(challenge));

    println!("Press any key to close the application.");
    _ = std::io::stdin().read_line(&mut day_string);
}