use std::collections::HashMap;

pub fn run(inp: String) -> String {
    let split_inputs = inp.split("\n\n");
    let mut seeds: Vec<usize> = Vec::new();
    let mut mappings: HashMap<String, Mapping> = HashMap::new();

    for input in split_inputs {
        if seeds.is_empty() {
            seeds = input.split(' ').filter_map(|s| s.parse().ok()).collect();
            continue;
        }
        let mapping = parse(input);
        mappings.insert(mapping.from.clone(), mapping);
    }

    let mut lowest = usize::MAX;
    for ele in seeds {
        let mut from = "seed".to_string();
        let mut number = ele;
        while mappings.contains_key(&from) {
            let mapping = mappings.get(&from).unwrap();
            from = mapping.to.clone();
            for ele in &mapping.nbr_to_nbr {
                if ele.src <= number && ele.src + ele.cnt > number {
                    number = ele.dst + (number - ele.src);
                    break;
                }
            }
        }
        lowest = lowest.min(number);
    }

    return format!("{}", lowest);
}

pub fn parse(inp: &str) -> Mapping {
    let lines: Vec<&str> = inp.split('\n').map(|s| s.trim()).collect();
    let from_to_split: Vec<&str> = lines.get(0).unwrap().split(" ").next().unwrap().split("-").collect();
    let mut nbr_to_nbr = Vec::new();
    for ele in lines.into_iter().skip(1) {
        let parts: Vec<usize> = ele.split(' ').map(|s| s.parse().unwrap()).collect();
        let dst = parts.get(0).unwrap().clone();
        let src = parts.get(1).unwrap().clone();
        let cnt = parts.get(2).unwrap().clone();
        nbr_to_nbr.push(NumberToNumber { src, dst, cnt });
    }
    
    return Mapping {
        from: from_to_split.get(0).unwrap().to_string(),
        to: from_to_split.get(2).unwrap().to_string(),
        nbr_to_nbr: nbr_to_nbr,
    }
}

pub struct Mapping {
    pub from: String,
    pub to: String,
    pub nbr_to_nbr: Vec<NumberToNumber>,
}

pub struct NumberToNumber {
    pub src: usize,
    pub dst: usize,
    pub cnt: usize,
}