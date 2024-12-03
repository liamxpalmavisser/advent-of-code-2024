fn main() {
    let input = include_str!("./input.txt");
    let result = part1(input);
    dbg!(result);
}

fn check_enable_state(expr: &str, current_state: bool) -> bool {
    match expr {
        e if e.contains("do()") => true,
        e if e.contains("don't()") => false,
        _ => current_state,
    }
}

fn parse(input: &str) -> Vec<(i32, i32)> {
    let parts: Vec<&str> = input.split("mul(").collect();
    let mut pairs = Vec::new();
    let mut enabled = true;

    for part in &parts[1..] {
        if let Some(idx) = part.find(')') {
            let expr = &part[..idx];

            enabled = check_enable_state(expr, enabled);

            if let Some((x, y)) = expr.split_once(',') {
                if let (Ok(x), Ok(y)) = (x.trim().parse::<i32>(), y.trim().parse::<i32>()) {
                    if enabled {
                        pairs.push((x, y));
                    }
                }
            }

            if let Some(rest) = part.get(idx..) {
                enabled = check_enable_state(rest, enabled);
            }
        }
    }
    pairs
}

fn get_products(pairs: Vec<(i32, i32)>) -> i32 {
    pairs.iter().map(|&(x, y)| x * y).sum()
}

fn part1(input: &str) -> i32 {
    get_products(parse(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example.txt");
        let result = part1(input);
        assert_eq!(result, 48);
    }
}
