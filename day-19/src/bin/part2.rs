fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> i64 {
    let (available_towels, designs) = parse(input);

    let mut possible_count = 0;

    for design in designs {
        let result = dp(design, &available_towels);

        possible_count += result;
    }

    return possible_count;
}

fn dp(design: &str, available_towels: &Vec<&str>) -> i64 {
    let mut n_options = vec![0; design.len() + 1];
    n_options[0] = 1;

    for i in 0..design.len() {
        if n_options[i] > 0 {
            for towel in available_towels {
                if design[i..].starts_with(towel) {
                    n_options[i + towel.len()] += n_options[i];
                }
            }
        }
    }

    return n_options[design.len()];
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
