use std::collections::{HashSet, HashMap};

use crate::{day10_1::{self, Coordinate, Direction, find_all_attaching_pos}};

const UNKNOWN: char = '?';
const FREE: char = 'V';
const PIPE: char = 'P';

pub fn run(inp: String) -> String {
    let directions_per_pipe_type = day10_1::get_possible_directions();
    let map = zoom_in(day10_1::parse(inp), &directions_per_pipe_type);
    let current_pos = day10_1::get_starting_position(&map).unwrap();
    let mut path_taken = HashSet::new();
    path_taken.insert(current_pos.clone());
    let mut current_pos = day10_1::find_attaching_pos(&map, &directions_per_pipe_type, current_pos).unwrap();
    let mut current_pipe_type = map.get(&current_pos).unwrap();

    'traversing: loop {
        path_taken.insert(current_pos.clone());
        for ele in directions_per_pipe_type.get(current_pipe_type).unwrap_or(&vec![]) {
            let new_pos = current_pos.go(ele);
            if path_taken.contains(&new_pos) {
                continue;
            }
            let new_pipe_type = map.get(&new_pos).unwrap();
            if new_pipe_type == &day10_1::GROUND {
                continue;
            }

            current_pipe_type = new_pipe_type;
            current_pos = new_pos;
            continue 'traversing;
        }
        break;
    }

    let mut encapsulation_map = HashMap::with_capacity(map.len());
    for ele in map {
        let to_insert = if path_taken.contains(&ele.0) {
            PIPE
        } else {
            UNKNOWN
        };
        encapsulation_map.insert(ele.0, to_insert);
    }

    propagate_free(&mut encapsulation_map);

    let result = zoom_out(encapsulation_map.clone());
    return format!("{}", result.values().filter(|x| x == &&UNKNOWN).count());
}

fn zoom_in(map: HashMap<Coordinate, char>, directions_per_pipe_type: &HashMap<char, Vec<Direction>>) -> HashMap<Coordinate, char> {
    let mut result = HashMap::with_capacity(map.len() * 9);
    let empty_list = vec![];
    for ele in &map {
        let new_coord_base = Coordinate{x: ele.0.x*3, y: ele.0.y*3};
        let new_coord_ele = Coordinate{x: new_coord_base.x + 1, y: new_coord_base.y + 1};
        let mut directions = directions_per_pipe_type.get(&ele.1).unwrap_or(&empty_list).clone();
        if ele.1 == &day10_1::STARTING_POSITION {
            directions.clear();
            let new_coords = find_all_attaching_pos(&map, directions_per_pipe_type, ele.0.clone());
            for dir in day10_1::ALL_DIRECTIONS {
                if new_coords.contains(&ele.0.go(&dir)) {
                    directions.push(dir);
                }
            }
        }
        for dir in directions {
            if dir == day10_1::NORTH || dir == day10_1::SOUTH {
                result.insert(new_coord_ele.go(&dir), day10_1::NORTH_SOUTH);
            }
            if dir == day10_1::EAST || dir == day10_1::WEST {
                result.insert(new_coord_ele.go(&dir), day10_1::EAST_WEST);
            }
        }
        result.insert(new_coord_ele, ele.1.clone());
        for y in 0..3 {
            for x in 0..3 {
                let new_coord = Coordinate{x: new_coord_base.x + x, y: new_coord_base.y + y};
                if !result.contains_key(&new_coord) {
                    result.insert(new_coord, day10_1::GROUND);
                }
            }
        }
    }
    result
}

fn zoom_out(map: HashMap<Coordinate, char>) -> HashMap<Coordinate, char> {
    let mut result = HashMap::with_capacity(map.len() / 9);
    for ele in map {
        if ele.0.x %3 != 1 || ele.0.y %3 != 1{
            continue;
        }
        result.insert(Coordinate { x: ele.0.x/3, y: ele.0.y/3 }, ele.1);
    }
    result
}

fn propagate_free(encapsulation_map: &mut HashMap<Coordinate, char>) {
    let mut coords = vec![Coordinate{x:0, y:0}];
    while !coords.is_empty() {
        let mut new_coords = Vec::with_capacity(coords.len() * 4);
        for coord in &coords {
            let current = encapsulation_map.get(coord).unwrap();
            if current == &UNKNOWN {
                encapsulation_map.insert(coord.clone(), FREE);
                for direction in &day10_1::ALL_DIRECTIONS {
                    let new_coord = coord.go(direction);
                    if !encapsulation_map.contains_key(&new_coord) {
                        continue;
                    }
                    new_coords.push(new_coord);
                }
            }
        }
        coords = new_coords;
    }

}