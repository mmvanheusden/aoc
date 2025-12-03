use std::ops::{RangeInclusive};

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", solve_input(input));
}

fn solve_input(input: &str) -> String {
    let mut invalid_ids = 0;
    let mut ranges = input_to_ranges(input);
    ranges.iter_mut().for_each(|range| {
        for id in range.into_iter() {
            let converted = id.to_string();
            let split_id = &converted.split_at(&converted.len()/2);
            if split_id.0 == split_id.1 {
                println!("{converted}: {}={}", split_id.0, split_id.1);
                invalid_ids += id;
            }
        }
    });
    return format!("Invalide ID sum: {invalid_ids}")
}

fn input_to_ranges(input: &str) -> Vec<RangeInclusive<u64>> {
    let mut ranges: Vec<RangeInclusive<u64>> = vec![];
    let blocks: Vec<&str> = input.split(',').collect();
    blocks.iter().for_each(|block| {
        let split = block.split('-').collect::<Vec<&str>>();
        ranges.push(RangeInclusive::new(split[0].parse::<u64>().unwrap(), split[1].parse::<u64>().unwrap()));
    });
    return ranges
}