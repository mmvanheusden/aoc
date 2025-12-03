fn main() {
    let input = include_str!("../example.txt");
    println!("{}", solve_input(input));
}

fn solve_input(input: &str) -> &str {
    let sequences = input_to_sequences(input);
    sequences.iter().for_each(|sequence| {
        dbg!(sequence);
    });
    return "a"
}

fn input_to_sequences(input: &str) -> Vec<&str> {
    return input.lines().collect()
}