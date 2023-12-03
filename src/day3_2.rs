
pub fn run(inp: String) -> String {
    let lines : Vec<&str> = inp.split('\n').collect();

    let mut count = 0;

    for (y, line) in lines.clone().iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '*' {
                let neighbours = find_neighbours(x, y, &lines);
                let ratio = neighbours.get(0).unwrap_or(&0) * neighbours.get(1).unwrap_or(&0);//let's assume there's no parts where 3 neighbours are connected
                count += ratio;
            }
        }
    }

    return format!("{}", count);
}

fn find_neighbours(x_inp: usize, y_inp: usize, lines: &Vec<&str>) -> Vec<usize> {
    let mut result = Vec::with_capacity(2);
    for y in y_inp.checked_sub(1).unwrap_or(0)..y_inp+2 {
        let current_line: Vec<char> = lines.get(y).unwrap_or(&"").trim().chars().collect();
        let mut inside_number = false;
        for x in x_inp.checked_sub(1).unwrap_or(0)..x_inp+2 {
            let is_numeric = current_line.get(x).unwrap_or(&'.').is_numeric();
            if is_numeric && !inside_number {
                result.push(get_number_at(x, &current_line));
                inside_number = true;
            }
            if !is_numeric {
                inside_number = false;
            }
        }
    }

    result
}

fn get_number_at(x_inp: usize, line: &Vec<char>) -> usize {
    let mut x  = x_inp;
    for x_leftbound in (0..x_inp).rev() {
        if line.get(x_leftbound).unwrap_or(&'.').is_numeric() {
            x = x_leftbound;
        } else {
            break;
        }
    }
    let mut result = "".to_string();
    for x in x..line.len() {
        let c = line.get(x).unwrap_or(&'.');
        if !c.is_numeric() {
            break;
        }
        result.push(c.clone());
    }

    return result.parse().unwrap()
}