fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn parse(input: &str) -> Vec<(i32, i32)> {
    let parts: Vec<&str> = input.split("mul(").collect();
    let mut pairs = Vec::new();

    for part in &parts[1..] {
        if let Some(end_idx) = part.find(')') {
            let content = &part[..end_idx];
            if let Some((x, y)) = content.split_once(',') {
                if let (Ok(x), Ok(y)) = (x.trim().parse::<i32>(), y.trim().parse::<i32>()) {
                    pairs.push((x, y));
                }
            }
        }
    }
    pairs
}

fn get_products(input: Vec<(i32, i32)>) -> i32 {
    let product_sum: i32 = input.iter().map(|&(x, y)| x * y).sum();
    product_sum
}

fn part1(input: &str) -> i32 {
    let parsed_input = parse(input);
    let product_sum = get_products(parsed_input);
    product_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example.txt");
        let result = part1(input);
        assert_eq!(result, 161 as i32);
    }
}
