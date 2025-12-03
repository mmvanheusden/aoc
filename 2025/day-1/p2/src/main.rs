fn main() {
    let input = include_str!("../input.txt");
    println!("{}", solve_input(input));
}
// 5182 -> te laag
// 7613 -> te hoog
// 6115 -> te laag
// 6389 -> nee
// 6246 -> nee
// 4444 -> nee <- Bleek foutief te zijn, had moeten zijn 5478
// 4896 -> nee <- Bleek foutief te zijn, had moeten zijn 5930

fn solve_input(input: &str) -> String {
    let mut DIAL = 50;
    let mut ends_at_zero = 0;
    let mut hits_zero = 0;
    let rotations = parse_to_rotations(input);
    rotations.iter().for_each(|rotation| {
        println!("\nDIAL before: {DIAL}\tI: {:?}", rotation);
        match rotation {
            Rotation::Left(x) => {
                let clicks = x % 100; // Hiermee zorgen we dat we nooit meer dan 1 rondje draaien.
                if DIAL - x < 0 && DIAL != 0 {
                    let amount = div_up(x, 100 as usize);
                    println!("We gaan {amount}x over 0 heen naar links.");
                    hits_zero += amount;
                }
                if clicks > DIAL {
                    DIAL += 100 - clicks;
                } else {
                    DIAL -= clicks;
                }
            },
            Rotation::Right(x) => {
                let clicks = x % 100;
                let dial_before = DIAL;
                DIAL += clicks;
                if DIAL >= 100 {
                    DIAL -= 100;
                }
                if dial_before + x >= 100 && dial_before != 0 {
                    let amount = div_up(x, 100 as usize);
                    println!("We gaan {amount}x over 0 heen naar rechts.");

                    hits_zero += amount;
                }
            }
        };
        if DIAL == 0 {
            ends_at_zero += 1;
            println!("Eindigd op 0!")
        };
        println!("DIAL after: {DIAL}");
    });
    return format!("EZ: {ends_at_zero}, HZ: {hits_zero}, sum: {}", ends_at_zero+hits_zero)
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

/// Source: https://www.reddit.com/r/rust/comments/bk7v15/my_next_favourite_way_to_divide_integers_rounding/
pub fn div_up(a: &i32, b: usize) -> usize {
    // We *know* that the hint is exact, this is thus precisely the amount of chunks of length `b` each
    (0..*a).step_by(b).size_hint().0
}

#[derive(Debug)]
enum Rotation {
    Left(i32),
    Right(i32)
}
