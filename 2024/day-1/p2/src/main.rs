use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("../input").unwrap();
    let mut separated_input = split_input(input);
    // println!("{:?}", separated_input);

    // create hashmap of recurrence


    // println!("{}", output);

    create_recurrence_hashmap(separated_input);
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

fn create_recurrence_hashmap(mut input: (Vec<u32>, Vec<u32>)) {
    let hashmap: HashMap<u32, u32> = HashMap::new();
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();


    // frequency map of values right
    let frequency_right: HashMap<u32,u32> = input.1
        .iter()
        .copied()
        .fold(HashMap::new(), |mut map, val|{
            map.entry(val)
                .and_modify(|frq|*frq+=1)
                .or_insert(1);
            map
        });
    // println!("{:?}", frequency_right);

    // multiply left values with right value linked to key. if not exist multi by 0.
    println!("{:?}", input.0);

    for value in input.0.iter_mut() {
        let factor = frequency_right.get(value).unwrap_or(&0);
        *value *= factor;
    }
    input.0.retain(|&i| i != 0); // filter out values that became 0.


    println!("{:?}", input.0);

    // product of vector

    let res: u32 = input.0.iter().copied().reduce(|a, b| a + b).unwrap();

    println!("{res}")
}