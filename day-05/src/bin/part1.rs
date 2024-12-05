use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn rule_check(first: i32, second: i32, hashmap: HashMap<i32, i32>) -> bool {
    if let (Some(&first_value), Some(&second_value)) = (hashmap.get(&first), hashmap.get(&second)) {
        if first_value < second_value {
            return true;
        } else {
            return false;
        }
    }
    return true;
}

fn parse_rules(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line
                .split('|')
                .map(|num| num.parse::<i32>().expect("This should be a number"));
            let first = parts.next().expect("Missing first number");
            let second = parts.next().expect("Missing second number");
            (first, second)
        })
        .collect()
}

fn turn_vec_into_hash(line: &Vec<i32>) -> HashMap<i32, i32> {
    let hashmap: HashMap<i32, i32> = line
        .iter()
        .enumerate()
        .map(|(index, &value)| (value, index as i32))
        .collect();
    hashmap
}

fn parse_order(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|num| num.parse::<i32>().expect("this should be a number"))
                .collect()
        })
        .collect()
}

fn split_input(input: &str) -> (&str, &str) {
    input
        .split_once("\n\n")
        .expect("Input does not contain two parts separate")
}

fn part1(input: &str) -> i32 {
    let (first_input, second_input) = split_input(input);
    let rules = parse_rules(first_input);
    let order = parse_order(second_input);
    let mut middle_values: Vec<i32> = Vec::new();

    for (idx, line) in order.iter().enumerate() {
        let current_hash = turn_vec_into_hash(line);
        let mut rule_broken = false;

        for rule in rules.clone().into_iter() {
            let (first, second) = rule;
            if !rule_check(first, second, current_hash.clone()) {
                rule_broken = true;
                break;
            }
        }

        if !rule_broken {
            if let Some(middle) = line.get(line.len() / 2) {
                middle_values.push(*middle);
            } else {
                println!("Line {idx} is empty, no middle value.");
            }
        }
    }
    middle_values.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example.txt");
        let result = part1(input);
        assert_eq!(result, 143 as i32);
    }
}
