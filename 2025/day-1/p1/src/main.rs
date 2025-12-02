fn main() {
    let input = include_str!("../input.txt");
    println!("{}", solve_input(input));
}

fn solve_input(input: &str) -> String {
    let mut DIAL = 50;
    let mut hit_zero = 0;
    let rotations = parse_to_rotations(input);
    rotations.iter().for_each(|rotation| {
        println!("\nDIAL before: {DIAL}\tI: {:?}", rotation);
        match rotation {
            Rotation::Left(x) => {
                let clicks = x % 100; // Hiermee zorgen we dat we nooit meer dan 1 rondje draaien.
                if clicks > DIAL {
                    DIAL += 100 - clicks;
                } else {
                    DIAL -= clicks;
                }
            },
            Rotation::Right(x) => {
                let clicks = x % 100;
                DIAL += clicks;
                if DIAL >= 100 {
                    DIAL -= 100;
                }
            }
        };
        if DIAL == 0 {
            hit_zero += 1;
        };
        println!("DIAL after: {DIAL}");
    });
    return format!("HZ: {hit_zero}")
}

fn parse_to_rotations(input: &str) -> Vec<Rotation> {
    let mut rotations: Vec<Rotation>  = vec![];
    for line in input.lines() {
       rotations.push(match line.chars().next().unwrap() {
            'L' => {
                Rotation::Left(line[1..line.len()].parse::<i32>().unwrap())
            },
            'R' => {
                Rotation::Right(line[1..line.len()].parse::<i32>().unwrap())
            },
            _ => panic!()
        });
    }
    return rotations
}

#[derive(Debug)]
enum Rotation {
    Left(i32),
    Right(i32)
}
