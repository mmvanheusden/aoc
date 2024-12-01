use std::fs;

fn main() {
    let input = fs::read_to_string("../input").unwrap();
    let mut separated_input = split_input(input);
    // println!("{:?}", separated_input);

    // sort left
    separated_input.0.sort();
    // sort right
    separated_input.1.sort();

    // println!("{:?}", separated_input);

    let difference = take_difference(separated_input);
    // println!("{:?}", difference);

    let mut output = 0;
    for val in difference {
        output += val;
    }

    println!("{}", output);
}


// split input into tuple with 2 vectors.
fn split_input(input: String) -> (Vec<u32>, Vec<u32>) {
    let mut separated_values: (Vec<u32>, Vec<u32>) = (Vec::new(), Vec::new());
    for line in input.lines() {
        separated_values.0.push(line[0..5].parse::<u32>().unwrap());
        separated_values.1.push(line[8..13].parse::<u32>().unwrap());
    }

    separated_values
}

fn take_difference(input: (Vec<u32>, Vec<u32>)) -> Vec<usize> {
    let mut output: Vec<usize> = Vec::new();
    for (left, right) in input.0.iter().zip(input.1.iter()) {
        output.push((*left as usize).abs_diff(*right as usize))
    }

    output
}