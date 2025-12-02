use std::ops::{RangeInclusive};

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", solve_input(input));
}

fn solve_input(input: &str) -> &str {
    dbg!(input_to_ranges(input));
    return "a"
}

fn input_to_ranges(input: &str) -> Vec<RangeInclusive<i64>> {
    let mut ranges: Vec<RangeInclusive<i64>> = vec![];
    let blocks: Vec<&str> = input.split(',').collect();
    blocks.iter().for_each(|block| {
        let split = block.split('-').collect::<Vec<&str>>();
        ranges.push(RangeInclusive::new(split[0].parse::<i64>().unwrap(), split[1].parse::<i64>().unwrap()));
    });
    return ranges
}