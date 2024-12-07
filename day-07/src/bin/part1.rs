fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn parse_input(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(":");
            let expected = parts.next().expect("Line missing ':' separator");
            let values = parts.next().expect("Line missing ':' separator");

            let expected_number = expected.parse::<i64>().expect("This should be a number");
            let input_values = values
                .split_whitespace()
                .map(|number| number.parse::<i64>().expect("This should be a number"))
                .collect::<Vec<i64>>();

            (expected_number, input_values)
        })
        .collect()
}

fn backtracking(first: i64, second: &[i64], expected: i64) -> bool {
    if second.is_empty() {
        return first == expected;
    }

    let rest = &second[1..];

    let options = [first + second[0], first * second[0]];

    options
        .iter()
        .filter(|&&new_first| new_first <= expected)
        .any(|&new_first| backtracking(new_first, rest, expected))
}

fn part1(input: &str) -> i64 {
    let mut correct_equation_count = 0;

    let parsed_input = parse_input(input);

    for (expected_value, input_values) in parsed_input.into_iter() {
        let first_number = input_values[0];
        let second = &input_values[1..];

        if backtracking(first_number, second, expected_value) {
            correct_equation_count += expected_value;
        }
    }

    correct_equation_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example.txt");
        let result = part1(input);
        assert_eq!(result, 3749 as i64);
    }
}
