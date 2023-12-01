use std::collections::HashMap;

pub fn run(inp: String) -> String {
    let mut replace = HashMap::new();
    replace.insert("one".to_string(), "1");
    replace.insert("two".to_string(), "2");
    replace.insert("three".to_string(), "3");
    replace.insert("four".to_string(), "4");
    replace.insert("five".to_string(), "5");
    replace.insert("six".to_string(), "6");
    replace.insert("seven".to_string(), "7");
    replace.insert("eight".to_string(), "8");
    replace.insert("nine".to_string(), "9");
    let mut inp_replaced = inp.clone();
    for ele in replace {
        let mut actual_replace = ele.0.clone();
        actual_replace.push_str(ele.1);
        actual_replace.push_str(ele.0.as_str());
        inp_replaced = inp_replaced.replace(&ele.0, actual_replace.as_str());
    }
    let mut numbers = Vec::new();
    numbers.push(Vec::new());
    for ele in inp_replaced.chars() {
        let nbr = ele.to_digit(10);
        if nbr.is_some() {
            numbers.last_mut().unwrap().push(nbr.unwrap());
        }
        if ele == '\n' {
            numbers.push(Vec::new());
        }
    }
    let mut count = 0;
    for line_numbers in numbers {
        count += line_numbers.first().unwrap() * 10;
        count += line_numbers.last().unwrap();
    }

    return format!("{}", count);
}