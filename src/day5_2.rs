use crate::day5_1;

use std::collections::HashMap;

pub fn run(inp: String) -> String {
    let split_inputs = inp.split("\n\n");
    let mut ranges: Vec<Range> = Vec::new();
    let mut mappings: HashMap<String, day5_1::Mapping> = HashMap::new();

    for input in split_inputs {
        if ranges.is_empty() {
            let tmp: Vec<usize> = input.split(' ').filter_map(|s| s.parse().ok()).collect();
            for i in (0..tmp.len()).step_by(2) {
                let start = tmp.get(i).unwrap().clone();
                let cnt = tmp.get(i+1).unwrap().clone();
                ranges.push(Range { start, end: start + cnt })
            }
            continue;
        }
        let mapping = day5_1::parse(input);
        mappings.insert(mapping.from.clone(), mapping);
    }

    let mut from = "seed".to_string();
    while mappings.contains_key(&from) {
        let mut new_ranges = Vec::new();
        let mapping = mappings.get(&from).unwrap();
        from = mapping.to.clone();
        for nbr_to_nbr in &mapping.nbr_to_nbr {
            let mut unused_ranges = Vec::new();
            for source_ele in ranges {
                let range_before = Range{
                    start: source_ele.start,
                    end: source_ele.end.min(nbr_to_nbr.src).max(source_ele.start),
                };
                let range_after = Range{
                    start: range_before.end.max(nbr_to_nbr.src + nbr_to_nbr.cnt).min(source_ele.end),
                    end: source_ele.end,
                };
                let usable_range = Range{
                    start: range_before.end,
                    end: range_after.start,
                };
                if range_before.end > range_before.start {
                    unused_ranges.push(range_before);
                }
                if range_after.end > range_after.start {
                    unused_ranges.push(range_after);
                }
                if usable_range.end > usable_range.start {
                    let new_range = Range { start: usable_range.start - nbr_to_nbr.src + nbr_to_nbr.dst, end: usable_range.end - nbr_to_nbr.src + nbr_to_nbr.dst };
                    new_ranges.push(new_range)
                }
            }
            ranges = unused_ranges;
        }
        ranges.append(&mut new_ranges);
    }
    let mut lowest = usize::MAX;
    for ele in ranges {
        lowest = lowest.min(ele.start);
    }

    return format!("{}", lowest);
}

pub struct Range {
    pub start: usize,
    pub end: usize,
}