use std::ops::{RangeInclusive};

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", solve_input(input));
}

fn solve_input(input: &str) -> &str {
    let mut ranges = input_to_ranges(input);
    // dbg!(ranges);
    ranges.iter_mut().for_each(|range| {
        for id in range.into_iter() {
            // dbg!(id);
        }
    });
    return "a"
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