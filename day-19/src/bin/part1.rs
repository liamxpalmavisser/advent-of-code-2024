fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let (available_towels, designs) = parse(input);

    let mut possible_count = 0;

    for design in designs {
        dp(design, &available_towels, &mut possible_count);
    }

    return possible_count;
}

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let available_towels = input.lines().next().expect("First line should be there");

    let towels = available_towels
        .split(",")
        .map(|towel| towel.trim())
        .collect();

    let designs = input.lines().skip(2).collect();

    return (towels, designs);
}

fn dp(design: &str, available_towels: &Vec<&str>, possible_count: &mut i32) {
    let mut is_possible = vec![false; design.len() + 1];
    is_possible[0] = true;

    for i in 0..design.len() {
        if is_possible[i] {
            for towel in available_towels {
                if design[i..].starts_with(towel) {
                    is_possible[i + towel.len()] = true;
                }
            }
        }
    }

    if is_possible[design.len()] {
        *possible_count += 1;
    }
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
