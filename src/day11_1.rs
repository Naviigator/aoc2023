use crate::day10_1::Coordinate;

pub fn run(inp: String) -> String {
    let coords = parse(inp, 1);

    let mut result = 0;
    for i in 0..coords.len() {
        for j in i..coords.len() {
            let coord1 = coords.get(i).unwrap();
            let coord2 = coords.get(j).unwrap();
            result += coord1.x.abs_diff(coord2.x) + (coord1.y).abs_diff(coord2.y);
        }
    }

    return format!("{:?}", result);
}

pub fn parse(inp: String, distance: usize) -> Vec<Coordinate> {
    let mut parsed = Vec::new();
    let mut buffer = Vec::new();
    let mut empty_row = Vec::new();
    for ele in inp.chars() {
        if ele == '\n' {
            if !buffer.contains(&'#') {
                empty_row.push(parsed.len());
            }
            parsed.push(buffer);
            buffer = Vec::new();
            continue
        }
        buffer.push(ele);
    }
    if !buffer.contains(&'#') {
        empty_row.push(parsed.len());
    }
    parsed.push(buffer);
    
    let mut empty_col = Vec::new();
    let mut x = 0;
    while x < parsed.get(0).unwrap().len() {
        let mut empty = true;
        for y in 0..parsed.len() {
            if parsed.get(y).unwrap().get(x).unwrap() == &'#' {
                empty = false;
                break;
            }
        }
        if empty {
            empty_col.push(x);
        }
        x+=1;
    }

    let mut y_extra = 0;
    let mut result = Vec::new();
    for y in 0..parsed.len() {
        let mut x_extra = 0;
        if empty_row.contains(&y) {
            y_extra += distance;
            continue;
        }
        let row = parsed.get(y).unwrap();
        for x in 0..row.len() {
            if empty_col.contains(&x) {
                x_extra += distance;
                continue;
            }
            if row.get(x).unwrap() == &'#' {
                result.push(Coordinate{x: x + x_extra, y :y + y_extra});
            }
        }
    }
    result
}