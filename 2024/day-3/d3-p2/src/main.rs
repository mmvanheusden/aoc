use regex;
use regex::Regex;

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve_input(input));
}

fn solve_input(input: &str) -> &str {
    // extract all instructions with my awesome expression
    let ex = Regex::new(r"do\(\)|mul\(\d*,\d*\)|don't\(\)").unwrap();
    let matches: Vec<&str> = ex.find_iter(input).map(|m| m.as_str()).collect();
    let mut instructions: Vec<Multiplication> = vec![];


    // println!("{:?}", matches);

    // let mut stack: VecDeque<bool> = vec![true];
    let mut enabled = true;
    for (i,matsj) in matches.iter().enumerate() {
        println!("{}", i);
        // We start enabled.
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
                println!("weird!")
            }
        }
    }

    // println!("{:?}", instructions);


    let sum: u128 = instructions.iter().map(|m| m.a * m.b).sum();

    println!("{}", sum);
    "a"
}

#[derive(Debug)]
struct Multiplication {
    a: u128,
    b: u128
}

impl Multiplication {
    fn from_instruction(instruction: &str) -> Multiplication {
        let mut iter = instruction.splitn(2, ',');
        let l = iter.next().unwrap();
        let r = iter.next().unwrap();

        // println!("{}", l);
        // println!("{}", r);

        println!("{:?}", Multiplication {
            a: l[4..].parse::<u128>().unwrap(),
            b: r[..r.len() - 1].parse::<u128>().unwrap(),
        });

        Multiplication {
            a: l[4..].parse::<u128>().unwrap(),
            b: r[..r.len()-1].parse::<u128>().unwrap(),
        }
    }
}