use crate::day11_1;

pub fn run(inp: String) -> String {
    let coords = day11_1::parse(inp, 1000000 - 1);

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