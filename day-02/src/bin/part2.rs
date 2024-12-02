fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
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

    /// Checks if a row is "safe" under the original rules.
    fn is_safe(row: &[i32]) -> bool {
        if row.len() < 2 {
            return false;
        }
        let initial_diff = row[1] - row[0];
        let initial_sign = initial_diff.signum();

        for window in row.windows(2) {
            let diff = window[1] - window[0];
            if diff.signum() != initial_sign || diff.abs() > 3 || diff == 0 {
                return false;
            }
        }
        true
    }

    /// Checks if a row becomes "safe" by removing one element.
    fn is_safe_with_removal(row: &[i32]) -> bool {
        for i in 0..row.len() {
            // Create a new row with the i-th element removed
            let mut new_row = row.to_vec();
            new_row.remove(i);
            if Self::is_safe(&new_row) {
                return true;
            }
        }
        false
    }

    pub fn sort_row(&mut self) -> i32 {
        let mut n_safe: i32 = 0;

        for row in self.rows.iter() {
            if Self::is_safe(row) || Self::is_safe_with_removal(row) {
                println!("{:?} is safe", row);
                n_safe += 1;
            } else {
                println!("{:?} is unsafe", row);
            }
        }

        n_safe
    }
}


fn part2(_input: &str) -> i32 {
    let mut solution = Solution::new(_input);
    solution.sort_row()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example.txt");
        let result = part2(input);
        assert_eq!(result, 4 as i32);
    }
}
