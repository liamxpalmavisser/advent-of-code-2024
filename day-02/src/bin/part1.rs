fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

pub struct Solution<'a> {
    input: &'a str,
    rows: Vec<Vec<i32>>,
}

impl<'a> Solution<'a> {
    pub fn new(input: &'a str) -> Self {
        let rows = Self::parse_input_to_rows(input);
        Self { input, rows }
    }

    fn parse_input_to_rows(input: &str) -> Vec<Vec<i32>> {
        let rows: Vec<Vec<i32>> = input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|num| num.parse::<i32>().expect("This should be a number"))
                    .collect()
            })
            .collect();
        rows
    }

    pub fn sort_row(&mut self) -> i32 {
        let mut n_safe: i32 = 0;

        for row in self.rows.iter() {
            let initial_diff = row[1] - row[0];
            let initial_sign = initial_diff.signum();
            let mut completed_without_break = true;

            for window in row.windows(2) {
                if (window[1] - window[0]).signum() != initial_sign
                    || (window[1] - window[0]).abs() > 3
                    || (window[1] - window[0]) == 0
                {
                    completed_without_break = false;
                    break;
                }
            }
            if completed_without_break {
                println!("{:?}", row);
                n_safe += 1;
            }
        }
        n_safe
    }
}

fn part1(_input: &str) -> i32 {
    let mut solution = Solution::new(_input);
    solution.sort_row()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example.txt");
        let result = part1(input);
        assert_eq!(result, 2 as i32);
    }
}
