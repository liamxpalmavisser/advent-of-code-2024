use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let (available_towels, designs) = parse(input);

    let mut possible_count = 0;
    let mut cache = HashMap::new();

    for design in designs {
        if dfs(design, &available_towels, &mut cache) {
            possible_count += 1;
        }
    }

    return possible_count;
}

fn parse(input: &str) -> (HashSet<&str>, Vec<&str>) {
    let available_towels = input.lines().next().expect("First line should be there");

    let towels: HashSet<&str> = available_towels
        .split(",")
        .map(|towel| towel.trim())
        .collect();

    let designs = input.lines().skip(2).collect();

    return (towels, designs);
}

fn dfs(design: &str, available_towels: &HashSet<&str>, cache: &mut HashMap<String, bool>) -> bool {
    if let Some(&result) = cache.get(design) {
        return result;
    }

    for &towel in available_towels {
        if towel == design {
            cache.insert(design.to_string(), true);
            return true;
        }

        if let Some(remaining_design) = design.strip_prefix(towel) {
            if dfs(remaining_design, available_towels, cache) {
                return true;
            }
        }
    }

    cache.insert(design.to_string(), false);
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example.txt");
        let result = part1(input);
        assert_eq!(result, 6 as i32);
    }
}
