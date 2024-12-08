use regex;
use regex::Regex;

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve_input(input));
}

fn solve_input(input: &str) -> &str {
    // extract all instructions with my awesome expression
    let ex = Regex::new(r"mul\(\d*,\d*\)").unwrap();
    let matches: Vec<&str> = ex.find_iter(input).map(|m| m.as_str()).collect();

    // println!("{:?}", matches);


    let mut instructions: Vec<Multiplication> = vec![];
    for matsj in matches {
        let mut iter = matsj.splitn(2, ',');
        let l = iter.next().unwrap();
        let r = iter.next().unwrap();

        // println!("{}", l);
        // println!("{}", r);

        instructions.push(Multiplication {
            a: l[4..].parse::<u128>().unwrap(),
            b: r[..r.len()-1].parse::<u128>().unwrap(),
        })
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