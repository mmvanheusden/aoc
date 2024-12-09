use std::fmt::Debug;
use std::ops::Sub;
use std::time::SystemTime;

fn main() {
    let start = SystemTime::now();
    let input = include_str!("../../input.txt");
    let duration = SystemTime::now().duration_since(start).unwrap();
    println!("{}", solve_input(input));
    println!("Took: {:?}", duration);
}

fn solve_input(input: &str) -> usize {
    let reports: Vec<Report> = input.lines().map(Report::from).collect();

    reports.iter().filter(|&x| x.valid()).count()
}

#[derive(Debug, Eq, PartialEq)]
struct Report(
    Vec<u8>
);

impl From<&str> for Report {
    fn from(aoc_input_line: &str) -> Report {
        Report(aoc_input_line.split_whitespace().map(|x| x.parse::<u8>().unwrap()).collect())
    }
}

impl Report {
    fn valid(&self) -> bool {
        fn valid_step_and_order(set: &Vec<u8>) -> bool {
            if set.len() == 1 {
                return set[0].abs_diff(set[1]) == 1
            }

            set // increasing
                .iter()
                .is_sorted_by(|x, y| (1..4).contains(&(**y as i8 - **x as i8)))
            ||
            set // decreasing
                .iter()
                .is_sorted_by(|x, y| (1..4).contains(&(**x as i8 - **y as i8)))
        }


        // firstly the step and order must be valid.
        let valid = valid_step_and_order(&self.0);

        if !valid { // invalid in the first place. maybe we can mutate the report to make it a valid report.
            let valid_mutations: Vec<bool> = bruteforce_mutations(&self.0).iter().map(|x|valid_step_and_order(&x.0)).filter(|x| *x).collect(); // map each mutation to true/false, filter out false.
            return valid_mutations.len() > 0 // when there is at least 1 valid mutation, it means that the report can be fixed with an modification. So we return true.
        };
        valid
    }
}

// at this point just bruteforce it :crying:
fn bruteforce_mutations(array: &Vec<u8>) -> Vec<Report> {
    let result: Vec<Report> = array.iter().enumerate().map(|(i, _)| {
        let mut new_list = array.clone();
        new_list.remove(i);
        Report(new_list)
    }).collect();

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let input =
            /*y             n           n       y c           y c         y*/
            "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";
        let reports: Vec<Report> = input.lines().map(Report::from).collect();


        assert_eq!(true, reports[0].valid());
        assert_eq!(false, reports[1].valid());
        assert_eq!(false, reports[2].valid());
        assert_eq!(true, reports[3].valid()); // after correction
        assert_eq!(true, reports[4].valid()); // after correction
        assert_eq!(true, reports[5].valid());
    }

    #[test]
    fn test_input() {
        let input = include_str!("../../input.txt");

        assert_eq!(665, solve_input(input));
    }
}