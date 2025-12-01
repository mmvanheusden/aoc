fn main() {
    let input = include_str!("../input.txt");
    println!("{}", solve_input(input));
}

fn solve_input(input: &str) -> &str {
    for line in input.lines() {
        // if line == "\n" { continue }
        let rotation: Rotation = match line.chars().next().unwrap() {
            'R' => {
                Rotation::Left(line[1..line.len()].parse::<u32>().unwrap())
            },
            'L' => {
                Rotation::Right(line[1..line.len()].parse::<u32>().unwrap())
            },
            _ => panic!()
        };
        dbg!(rotation);
    }
    return "a"
}

#[derive(Debug)]
enum Rotation {
    Left(u32),
    Right(u32)
}
