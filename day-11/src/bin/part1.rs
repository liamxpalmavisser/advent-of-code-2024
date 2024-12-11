use std::collections::HashMap;

use utils::*;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn parse(input: &str) -> Vec<i64> {
    let parsed_input: Vec<i64> = input
        .split_whitespace()
        .map(|value| value.parse::<i64>().expect("a"))
        .collect();
    parsed_input
}

fn apply_rules(number: &i64) -> Vec<i64> {
    match number {
        0 => vec![1],

        _ if number.abs().to_string().len() % 2 == 0 => {
            let num_str = number.abs().to_string();
            let mid = num_str.len() / 2;
            let left = num_str[..mid].parse::<i64>().unwrap_or(0);
            let right = num_str[mid..].parse::<i64>().unwrap_or(0);
            vec![left, right]
        }

        _ => vec![number * 2024],
    }
}

fn dfs(number: i64, cache: &mut HashMap<(i64, usize), usize>, remaining_splits: usize) -> usize {
    if let Some(&cached_result) = cache.get(&(number, remaining_splits)) {
        return cached_result.clone();
    }

    if remaining_splits == 0 {
        return 1;
    }

    let splits = apply_rules(&number);
    let mut total_count = 0;

    for part in splits {
        total_count += dfs(part, cache, remaining_splits - 1);
    }

    cache.insert((number, remaining_splits), total_count);
    total_count
}

fn part1(input: &str) -> usize {
    let parsed_input = parse(input);
    let mut joe = 0;

    for x in parsed_input.into_iter() {
        joe += dfs(x, &mut HashMap::new(), 25);
    }

    joe
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example.txt");
        let result = part1(input);
        assert_eq!(result, 55312 as usize);
    }
}
