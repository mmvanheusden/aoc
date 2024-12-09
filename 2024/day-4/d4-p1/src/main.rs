fn main() {
    let input = include_str!("../../input");
    println!("{}", solve_input(input));
}

fn solve_input(input: &str) -> u16 {
    // https://www.youtube.com/watch?v=tiLwW8StyBc
    let zoek = Woordzoeker::from(input);

    zoek.find_str("XMAS");
    1
}

#[derive(Debug)]
struct Woordzoeker(Vec<Vec<char>>);

impl Woordzoeker {
    fn print(&self) {
        self.0.iter().for_each(|row| {
            println!("{}", row.iter().collect::<String>());
        });
        println!();
    }

    fn rotate(&self) -> Self {
        let columns = self.0.len(); // (m) length
        let rows = self.0[0].len(); // (n) width

        let mut rotated: Vec<Vec<char>> = vec![];
        for i in 0..rows {
            let mut row = vec![];
            for j in 0..columns {
                row.push(self.0[j][i]);
            }
            rotated.push(row);
        }

        Woordzoeker(rotated)
    }

    fn find_str(&self, target: &str) {
    }
}

impl From<&str> for Woordzoeker {
    fn from(aoc_input: &str) -> Self {
        Woordzoeker(
            aoc_input
                .lines()
                .map(|line| line.chars().collect())
                .collect()
        )
    }
}