use std::collections::HashMap;

fn main() {
    let input = include_str!("../../../input.txt");
    println!("{}", solve_input(input));
}

fn solve_input(input: &str) -> usize {
    // put each character of each line into vector
    let list = vectorize_input(input);
    // println!("{:?}", list);

    let mapped_values = map_safety(list);
    println!("{:?}", mapped_values);

    // count results

    let mut amount = 0;

    amount = mapped_values.iter().filter(|(k,v)| **v == true).count();

    amount
}

#[derive(Debug)]
enum ArrayOrder {
    Increasing,
    Decreasing,
}

fn vectorize_input(input: &str) -> Vec<Vec<u8>> {
    let mut result = Vec::new();
    for line in input.lines() {
        // println!("{}", line);
        result.push(line.split_whitespace().map(|x| x.parse::<u8>().unwrap()).collect())
    }

    result
}

fn map_safety(array: Vec<Vec<u8>>) -> HashMap<Vec<u8>, bool> {
    let mut hashmap = HashMap::new();
    for item in array {
        let mut safety = true;
        if check_order(item.clone()).is_none() {
            safety = false;
        }

        if !valid_steps(item.clone()) {
            safety = false;
        }

        hashmap.insert(
            item.clone(),
            safety,
        );
    }

    hashmap
}

fn check_order(array: Vec<u8>) -> Option<ArrayOrder> {
    let mut increasing = true;
    let mut decreasing = true;
    if array.len() <= 1 {
        return None;
    }

    for i in 1..array.len() {
        if array[i] < array[i - 1] {
            increasing = false;
        }

        if array[i] > array[i - 1] {
            decreasing = false;
        }
    }

    if increasing {
        Some(ArrayOrder::Increasing)
    } else if decreasing {
        Some(ArrayOrder::Decreasing)
    } else {
        None
    }
}

fn valid_steps(array: Vec<u8>) -> bool {
    if array.len() == 1 {
        return (usize::abs_diff(array[0] as usize, array[1] as usize) == 1)
    }

    for i in 1..array.len() {
        let eval = (1..4).contains(&usize::abs_diff(array[i] as usize, array[i - 1] as usize));
        if !eval {
            return false;
        }
    }

    // println!("{:?} has right step", array);

    true
}
