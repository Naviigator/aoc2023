use std::collections::{HashMap, HashSet};

pub const NORTH_SOUTH: char = '|';
pub const EAST_WEST: char = '-';
pub const NORTH_EAST: char = 'L';
pub const NORTH_WEST: char = 'J';
pub const SOUTH_EAST: char = 'F';
pub const SOUTH_WEST: char = '7';
pub const GROUND: char = '.';
pub const STARTING_POSITION: char = 'S';

pub const NORTH: Direction = Direction{x:0, y:-1};
pub const SOUTH: Direction = Direction{x:0, y:1};
pub const EAST: Direction = Direction{x:1, y:0};
pub const WEST: Direction = Direction{x:-1, y:0};

pub const ALL_DIRECTIONS: [Direction;4] = [EAST, SOUTH, WEST, NORTH];


pub fn run(inp: String) -> String {
    let map = parse(inp);
    let directions_per_pipe_type = get_possible_directions();
    let current_pos = get_starting_position(&map).unwrap();
    let mut path_taken = HashSet::new();
    path_taken.insert(current_pos.clone());
    let mut current_pos = find_attaching_pos(&map, &directions_per_pipe_type, current_pos).unwrap();
    let mut current_pipe_type = map.get(&current_pos).unwrap();

    'traversing: loop {
        path_taken.insert(current_pos.clone());
        for ele in directions_per_pipe_type.get(current_pipe_type).unwrap_or(&vec![]) {
            let new_pos = current_pos.go(ele);
            if path_taken.contains(&new_pos) {
                continue;
            }
            let new_pipe_type = map.get(&new_pos).unwrap();
            if new_pipe_type == &GROUND {
                continue;
            }

            current_pipe_type = new_pipe_type;
            current_pos = new_pos;
            continue 'traversing;
        }
        break;
    }

    return format!("{}", path_taken.len() / 2);
}

pub fn parse(inp: String) -> HashMap<Coordinate, char> {
    let mut result = HashMap::new();
    let mut y = 0;
    let mut x = 0;
    for ele in inp.chars() {
        if ele == '\n' {
            x = 0;
            y += 1;
            continue;
        }
        result.insert(Coordinate{x, y}, ele);
        x += 1;
    }
    result
}

pub fn get_starting_position(map: &HashMap<Coordinate, char>) -> Option<Coordinate> {
    for ele in map {
        if ele.1 == &STARTING_POSITION {
            return Some(ele.0.clone())
        }
    }
    return None
}

pub fn find_attaching_pos(map: &HashMap<Coordinate, char>, directions_per_pipe_type: &HashMap<char, Vec<Direction>>, coord: Coordinate) -> Option<Coordinate> {
    find_all_attaching_pos(map, directions_per_pipe_type, coord).get(0).map(|x| x.clone())
}

pub fn find_all_attaching_pos(map: &HashMap<Coordinate, char>, directions_per_pipe_type: &HashMap<char, Vec<Direction>>, coord: Coordinate) -> Vec<Coordinate> {
    let mut result = Vec::new();
    for dir in &ALL_DIRECTIONS {
        let new_coord = coord.go(dir);
        let current_pipe_type = map.get(&new_coord).unwrap_or(&GROUND);
        for ele in directions_per_pipe_type.get(current_pipe_type).unwrap_or(&vec![]) {
            if coord == new_coord.go(ele) {
                result.push(new_coord.clone());
            }
        }
    }
    result
}

pub fn get_possible_directions() -> HashMap<char, Vec<Direction>> {
    let mut result = HashMap::new();
    result.insert(NORTH_SOUTH, vec![NORTH, SOUTH]);
    result.insert(EAST_WEST, vec![EAST, WEST]);
    result.insert(NORTH_EAST, vec![NORTH, EAST]);
    result.insert(NORTH_WEST, vec![NORTH, WEST]);
    result.insert(SOUTH_EAST, vec![SOUTH, EAST]);
    result.insert(SOUTH_WEST, vec![SOUTH, WEST]);
    result
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Coordinate {
    pub fn go(&self, dir: &Direction) -> Coordinate {
        Coordinate{
            x: (dir.x + self.x as i64) as usize,
            y: (dir.y + self.y as i64) as usize,
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct Direction {
    pub x: i64,
    pub y: i64,
}