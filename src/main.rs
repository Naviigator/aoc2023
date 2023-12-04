pub mod day1_1;
pub mod day1_2;
pub mod day2_1;
pub mod day2_2;
pub mod day3_1;
pub mod day3_2;
pub mod day4_1;
pub mod day4_2;

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

    println!("What day to solve? EG: 1.1, 1.2,...");
    let mut day_string: String = String::new();
    _ = std::io::stdin().read_line(&mut day_string).unwrap();
    day_string = day_string.trim().to_string();
    let func = day_fns.get(&day_string).unwrap();
    
    println!("No crash? I'm impressed, you can enter a proper input. Enter your challenge. When done - please add an empty newline. Let's hope no challenges use empty newlines because then I will need to think...");
    let mut challenge: String = String::new();
    let mut input: String = String::new();
    loop {
        input.clear();
        _ = std::io::stdin().read_line(&mut input).unwrap();
        challenge.push_str(input.as_str());
        if input.trim().is_empty() {
            break;
        }
    }
    challenge = challenge.trim_end_matches("\r\n").to_string();
    println!("{}", func(challenge));

    println!("Press any key to close the application.");
    _ = std::io::stdin().read_line(&mut day_string);
}