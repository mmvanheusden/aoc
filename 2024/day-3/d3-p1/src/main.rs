use regex;
use regex::Regex;

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve_input(input));
}

fn solve_input(input: &str) -> u128 {
    // extract all instructions with my awesome expression
    let ex = Regex::new(r"mul\(\d*,\d*\)").unwrap();
    let matches: Vec<&str> = ex.find_iter(input).map(|m| m.as_str()).collect(); // walk through input and convert matches to string and push to array.
    let instructions: Vec<Multiplication> = matches.into_iter().map(Multiplication::from_instruction).collect();

    let sum: u128 = instructions.iter().map(|m| m.a * m.b).sum();

    sum
}

#[derive(Debug)]
struct Multiplication {
    a: u128,
    b: u128
}

impl Multiplication {
    fn from_instruction(instruction: &str) -> Multiplication {
        let (l,r) = instruction.split_once(",").unwrap(); // unpack l and r into tuple.

        Multiplication {
            a: l[4..].parse::<u128>().unwrap(),
            b: r[..r.len()-1].parse::<u128>().unwrap(),
        }
    }
}