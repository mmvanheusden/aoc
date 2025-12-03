use std::ops::{RangeInclusive};
use itertools::Itertools;

fn main() {
    let input = include_str!("../example.txt");
    println!("{}", solve_input(input));
}

fn solve_input(input: &str) -> String {
    let mut invalid_ids = 0;
    let mut ranges = input_to_ranges(input);
    ranges.iter_mut().for_each(|range| {
        for id in range.into_iter() {
            let converted = id.to_string();
            let split_half = &converted.split_at(&converted.len()/2);
            if split_half.0 == split_half.1 {
                println!("{converted}: {}={}", split_half.0, split_half.1);
                invalid_ids += id;
            }
            
            let iterer = converted.chars().into_iter();
            for (prev, next) in iterer.tuples() {
                if prev == next {
                    println!("{}--{}", prev, next);
                    
                }
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

/// Source: https://gist.github.com/victor-iyi/8a84185c1d52419b0d4915a648d5e3e1
pub fn gcd(mut n: u64, mut m: u64) -> u64 {
  assert!(n != 0 && m != 0);
  while m != 0 {
    if m < n {
      std::mem::swap(&mut m, &mut n);
    }
    m %= n;
  }
  n
}
