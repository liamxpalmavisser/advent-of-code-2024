use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> i64 {
    let (available_towels, designs) = parse(input);

    let mut possible_count = 0;
    let mut cache = HashMap::new();

    for design in designs {
        let result = dfs(design, &available_towels, &mut cache);

        possible_count += result;
    }

    return possible_count as i64;
}

fn dfs(design: &str, available_towels: &HashSet<&str>, cache: &mut HashMap<String, i64>) -> i64 {
    if let Some(&result) = cache.get(design) {
        return result;
    }
    let mut count = 0;

    for &towel in available_towels {
        if towel == design {
            count += 1;
        }

        if let Some(remaining_design) = design.strip_prefix(towel) {
            count += dfs(remaining_design, available_towels, cache)
        }
    }

    cache.insert(design.to_string(), count);
    return count;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example.txt");
        let result = part2(input);
        assert_eq!(result, 16 as i64);
    }
}
