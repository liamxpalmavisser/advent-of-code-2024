fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn parse_input(input: &str) -> Vec<Data> {
    let mut result = Vec::new();
    let mut current_buttons = Vec::new();
    let mut current_prize = (0, 0);

    for line in input.lines() {
        let line = line.trim();

        if line.starts_with("Button") {
            if let Some((x_str, y_str)) = line.split_once(": ") {
                if let Some((x_part, y_part)) = y_str.split_once(", ") {
                    let x = x_part
                        .strip_prefix("X+")
                        .unwrap_or("0")
                        .parse::<i64>()
                        .unwrap();
                    let y = y_part
                        .strip_prefix("Y+")
                        .unwrap_or("0")
                        .parse::<i64>()
                        .unwrap();

                    let cost = if current_buttons.is_empty() { 3 } else { 1 };

                    current_buttons.push(Button { x, y, cost });
                }
            }
        } else if line.starts_with("Prize") {
            if let Some((_, parts)) = line.split_once(": ") {
                if let Some((x_part, y_part)) = parts.split_once(", ") {
                    let x = x_part
                        .split("=")
                        .nth(1)
                        .unwrap_or("0")
                        .parse::<i64>()
                        .unwrap();

                    let y = y_part
                        .split("=")
                        .nth(1)
                        .unwrap_or("0")
                        .parse::<i64>()
                        .unwrap();
                    current_prize = (x, y);
                }
            }
        } else if line.is_empty() {
            if current_buttons.len() == 2 {
                result.push(Data {
                    buttons: (current_buttons[0].clone(), current_buttons[1].clone()),
                    end_point: current_prize,
                });
            }
            current_buttons.clear();
        }
    }

    if current_buttons.len() == 2 {
        result.push(Data {
            buttons: (current_buttons[0].clone(), current_buttons[1].clone()),
            end_point: current_prize,
        });
    }

    result
}

#[derive(Debug)]
struct Data {
    buttons: (Button, Button),
    end_point: (i64, i64),
}

#[derive(Clone, Debug)]
struct Button {
    x: i64,
    y: i64,
    cost: i64,
}

fn solve_linear_combination(a: Button, b: Button, end_point: (i64, i64)) -> Option<(i64, i64)> {
    let (x1, y1) = (a.x, a.y);
    let (x2, y2) = (b.x, b.y);
    let (final_x, final_y) = end_point;

    let determinant = x1 * y2 - x2 * y1;

    if determinant == 0 {
        return None;
    }

    let a_numerator = final_x * y2 - final_y * x2;
    let b_numerator = x1 * final_y - y1 * final_x;

    if a_numerator % determinant != 0 || b_numerator % determinant != 0 {
        return None;
    }

    let a = a_numerator / determinant;
    let b = b_numerator / determinant;

    if a < 0 || b < 0 {
        return None;
    }

    Some((a, b))
}

fn part2(input: &str) -> i64 {
    let data = parse_input(input);
    let mut total_tokens = 0;

    for game in data.into_iter() {
        let (a, b) = game.buttons;
        let end_point = (
            game.end_point.0 + 10000000000000,
            game.end_point.1 + 10000000000000,
        );

        if let Some((x, y)) = solve_linear_combination(a, b, end_point) {
            total_tokens += 3 * x + y;
        }
    }
    total_tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example.txt");
        let result = part2(input);
        assert_eq!(result, 480 as i64);
    }
}
