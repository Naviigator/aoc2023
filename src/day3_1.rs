
pub fn run(inp: String) -> String {
    let lines : Vec<&str> = inp.split('\n').collect();

    let mut count = 0;

    let mut current_part_number_str = "".to_string();
    let mut current_part_number_start = None;
    let mut current_part_number_end = None;

    for (y, line) in lines.clone().iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c.is_numeric() {
                current_part_number_str.push(c);
                current_part_number_end = Some(x);
                if current_part_number_start.is_none() {
                    current_part_number_start = Some(x);
                }
            } else {
                count += flush_engine_number(y, &lines, &mut current_part_number_str, &mut current_part_number_start, &mut current_part_number_end);
            }
        }
        count += flush_engine_number(y, &lines, &mut current_part_number_str, &mut current_part_number_start, &mut current_part_number_end);
    }

    return format!("{}", count);
}

fn flush_engine_number(y_inp: usize, lines: &Vec<&str>, current_part_number_str: &mut String, current_part_number_start: &mut Option<usize>, current_part_number_end: &mut Option<usize>) -> usize {
    let nbr: usize = current_part_number_str.parse().unwrap_or(0);

    let mut is_engine_number = false;
    for y in y_inp.checked_sub(1).unwrap_or(0)..y_inp+2 {
        let line_to_check = lines.get(y).unwrap_or(&"");
        let chars_to_check: Vec<char> = line_to_check.trim().chars().collect();
        for x in current_part_number_start.unwrap_or(0).checked_sub(1).unwrap_or(0)..current_part_number_end.unwrap_or(0)+2 {
            if is_symbol(chars_to_check.get(x).unwrap_or(&'.').clone()) {
                is_engine_number = true;
                break;
            }
        }
    }

    current_part_number_str.clear();
    *current_part_number_start = None;
    *current_part_number_end = None;

    if !is_engine_number {
        return 0
    }
    nbr
}

fn is_symbol(c: char) -> bool {
    !c.is_numeric() && c != '.'
}