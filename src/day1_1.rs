
pub fn run(inp: String) -> String {
    let mut numbers = Vec::new();
    numbers.push(Vec::new());
    for ele in inp.chars() {
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