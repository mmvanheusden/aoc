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

    reports.iter().filter(|&x| x.is_valid()).count()
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
    fn valid_step(&self) -> bool {
        if self.0.len() == 1 {
            return self.0[0].abs_diff(self.0[1]) == 1
        }

        // println!("{:?} i:{} d:{}",array, array.iter().is_sorted_by(|x, y| (1..4).contains(&(**y as i8 - **x as i8))),array.iter().is_sorted_by(|x, y| (1..4).contains(&(**x as i8 - **y as i8))));

        println!("{:?}", self.0);
        println!("Diff: {:?}", self.0.iter().zip(self.0.iter().skip(1)).map(|(a, b)| b.abs_diff(*a)).collect::<Vec<_>>());

        self.0 // increasing
            .iter()
            .is_sorted_by(|x, y| (1..4).contains(&(**y as i8 - **x as i8)))
        ||
        self.0 // decreasing
            .iter()
            .is_sorted_by(|x, y| (1..4).contains(&(**x as i8 - **y as i8)))
    }


    fn is_valid(&self) -> bool {
        // firstly the step must be valid.
        let step = self.valid_step();
        if !step {
            for mutation in bruteforce_mutations(&self.0) {
                if mutation.valid_step() {
                    println!("fixed {:?} to {:?}",&self.0, mutation);
                    return true
                }
            }
        };
        step
    }
}



// at this point just bruteforce it :crying:
fn bruteforce_mutations(array: &Vec<u8>) -> Vec<Report> {
    let mut result: Vec<Report> = array.iter().enumerate().map(|(i, _)| {
        let mut new_list = array.clone();
        new_list.remove(i);
        Report(new_list)
    }).collect();

    println!("{:?}", result);
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


        assert_eq!(true, reports[0].is_valid());
        assert_eq!(false, reports[1].is_valid());
        assert_eq!(false, reports[2].is_valid());
        assert_eq!(true, reports[3].is_valid()); // after correction
        assert_eq!(true, reports[4].is_valid()); // after correction
        assert_eq!(true, reports[5].is_valid());

    }
}