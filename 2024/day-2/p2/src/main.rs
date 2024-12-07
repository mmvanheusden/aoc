fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve_input(input));
}

fn solve_input(input: &str) -> usize {
    let mut reports: Vec<Report> = Vec::new();
    for line in input.lines() {
        Report::from_input(line).is_valid();
        reports.push(Report::from_input(line))
    }

    reports.retain(|r| r.is_valid());
    reports.iter().count()
}

#[derive(Debug, Eq, PartialEq)]
struct Report(
    Vec<u8>
);

impl Report {
    fn from_input(line: &str) -> Report {
        Report(line.split_whitespace().map(|x| x.parse::<u8>().unwrap()).collect())
    }

    fn is_valid(&self) -> bool {
         match (check_order(&self.0), valid_steps(&self.0)) {
            (true, true) => true,
             (true, false) => {
                 for attempt in bruteforce_mutations(&self.0) {
                     if valid_steps(&attempt) {
                         // we got a match!
                         return true
                     }
                 }
                 false
             }
            _ => false
        }
    }
}


fn convert_input(input: &str) -> Vec<Vec<u8>> {
    let mut result = Vec::new();
    for line in input.lines() {
        result.push(line.split_whitespace().map(|x| x.parse::<u8>().unwrap()).collect())
    }

    result
}

fn check_order(array: &Vec<u8>) -> bool {
    let mut increasing = true;
    let mut decreasing = true;
    if array.len() <= 1 {
        return true;
    }

    for i in 1..array.len() {
        if array[i] < array[i - 1] {
            increasing = false;
        }

        if array[i] > array[i - 1] {
            decreasing = false;
        }
    }

    increasing || decreasing
}

fn valid_steps(array: &Vec<u8>) -> bool {
    if array.len() == 1 {
        return array[0].abs_diff(array[1]) == 1
    }

    for i in 1..array.len() {
        let eval = (1..4).contains(&array[i].abs_diff(array[i - 1]));
        if !eval {
            return false;
        }
    }

    true
}

// at this point just bruteforce it :crying:
fn bruteforce_mutations(array: &Vec<u8>) -> Vec<Vec<u8>> {
    let mut big_list: Vec<Vec<u8>> = Vec::from(Vec::new());
    for (i,e) in array.iter().enumerate() {
        let mut new_list = array.clone();
        new_list.remove(i);
        big_list.push(new_list);
    }
    big_list
}