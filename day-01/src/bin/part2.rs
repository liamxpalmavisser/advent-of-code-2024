use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

pub struct Solution<'a> {
    input: &'a str,
    col1: Vec<i32>,
    counter: HashMap<i32, i32>,
}

impl<'a> Solution<'a> {
    pub fn new(input: &'a str) -> Self {
        let (col1, col2) = Self::parse_input_to_columns(input);
        let counter = Self::get_counter(col2);

        Self {
            input,
            col1,
            counter,
        }
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

    fn get_counter(col: Vec<i32>) -> HashMap<i32, i32> {
        let mut counter = HashMap::new();
        for n in col.into_iter() {
            match counter.get(&n) {
                Some(count) => counter.insert(n, count + 1),
                None => counter.insert(n, 1),
            };
        }
        counter
    }

    fn get_product(&self) -> i32 {
        self.col1
            .clone()
            .into_iter()
            .map(|n| match self.counter.get(&n) {
                Some(count) => count * n,
                None => 0,
            })
            .sum()
    }
}

fn part2(input: &str) -> i32 {
    let solution = Solution::new(input);
    solution.get_product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example.txt");
        let result = part2(input);
        assert_eq!(result, 31 as i32);
    }
}
