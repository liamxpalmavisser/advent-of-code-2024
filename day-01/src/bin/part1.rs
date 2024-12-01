fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

pub struct Solution<'a> {
    input: &'a str,
    col1: Vec<i32>,
    col2: Vec<i32>,
}

impl<'a> Solution<'a> {
    pub fn new(input: &'a str) -> Self {
        let (col1, col2) = Self::parse_input_to_columns(input);
        Self { input, col1, col2 }
    }

    fn parse_input_to_columns(input: &str) -> (Vec<i32>, Vec<i32>) {
        let mut col1 = Vec::new();
        let mut col2 = Vec::new();

        let numbers = input.lines().map(|line| line.split_whitespace());

        for mut line in numbers {
            if let (Some(a), Some(b)) = (line.next(), line.next()) {
                col1.push(a.parse::<i32>().expect("This is no number"));
                col2.push(b.parse::<i32>().expect("This is no number"));
            }
        }

        (col1, col2)
    }

    pub fn sort_column(&mut self) {
        self.col1.sort_unstable_by(|a, b| b.cmp(a));
        self.col2.sort_unstable_by(|a, b| b.cmp(a));
    }

    pub fn col_difference(&self) -> i32 {
        self.col1
            .iter()
            .zip(self.col2.iter())
            .map(|(a, b)| (a - b).abs())
            .sum()
    }
}

fn part1(_input: &str) -> i32 {
    let mut solution = Solution::new(_input);
    solution.sort_column();
    solution.col_difference()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        let result = part1(input);
        assert_eq!(result, 11 as i32);
    }
}
