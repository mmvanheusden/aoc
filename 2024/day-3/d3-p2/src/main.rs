use regex;
use regex::Regex;

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve_input(input));
}

fn solve_input(input: &str) -> u128 {
    // extract all instructions with my awesome expression
    let ex = Regex::new(r"do\(\)|mul\(\d*,\d*\)|don't\(\)").unwrap(); // match do, don't, sum all at once
    let matches: Vec<&str> = ex.find_iter(input).map(|m| m.as_str()).collect();
    let mut instructions: Vec<Multiplication> = vec![];


    // let mut stack: VecDeque<bool> = vec![true]; // I was first thinking about a stack but then I figured it can be done simpler with just a mutable bool.
    let mut enabled = true; // We start enabled.
    for matsj in matches {

        // match if we are allowed to multiply, and the type of instruction we got.
        match (enabled,matsj.contains("mul("), matsj.contains("do("), matsj.contains("don't(")) {
            (true,true,_,_) => {
                // DO!
                instructions.push(Multiplication::from_instruction(matsj))
            },
            (false,_,true,_) => {
                enabled = true
            }
            (true,_,_,true) => {
                enabled = false
            }
            _ => {
                // an instruction while disabled.
            }
        }
    }


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