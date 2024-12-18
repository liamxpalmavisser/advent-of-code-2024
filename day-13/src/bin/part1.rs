use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use utils::*;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let data = parse_input(input);
    let mut total_tokens = 0;

    for game in data.into_iter() {
        let (a, b) = game.buttons;
        let end_point = game.end_point;

        if let Some(tokens) = dijkstra(end_point, a, b) {
            total_tokens += tokens;
        }
    }

    total_tokens
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
                        .parse::<i32>()
                        .unwrap();
                    let y = y_part
                        .strip_prefix("Y+")
                        .unwrap_or("0")
                        .parse::<i32>()
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
                        .parse::<i32>()
                        .unwrap();

                    let y = y_part
                        .split("=")
                        .nth(1)
                        .unwrap_or("0")
                        .parse::<i32>()
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
    end_point: (i32, i32),
}

#[derive(Clone, Debug)]
struct Button {
    x: i32,
    y: i32,
    cost: i32,
}

fn dijkstra(end_point: (i32, i32), a: Button, b: Button) -> Option<i32> {
    let mut heap = BinaryHeap::new();
    let start = (0, 0);
    let start_depth = 0;

    heap.push(Reverse((0, start, start_depth, 0, 0)));

    let mut visited: HashSet<(i32, i32, i32, i32)> = HashSet::new();
    let mut min_cost: HashMap<((i32, i32), i32, i32), i32> = HashMap::new();
    min_cost.insert((start, 0, 0), 0);

    while let Some(Reverse((current_cost, current_point, current_depth, a_presses, b_presses))) =
        heap.pop()
    {
        if a_presses > 100 || b_presses > 100 {
            continue;
        }

        if current_point == end_point {
            return Some(current_cost);
        }

        if visited.contains(&(current_point.0, current_point.1, a_presses, b_presses)) {
            continue;
        }
        visited.insert((current_point.0, current_point.1, a_presses, b_presses));

        let neighbors = vec![
            (
                (current_point.0 + a.x, current_point.1 + a.y),
                a_presses + 1,
                b_presses,
                a.cost,
            ),
            (
                (current_point.0 + b.x, current_point.1 + b.y),
                a_presses,
                b_presses + 1,
                b.cost,
            ),
        ];

        for (neighbor, new_a_presses, new_b_presses, neighbor_cost) in neighbors {
            let new_cost = current_cost + neighbor_cost;
            let new_depth = current_depth + 1;

            if new_cost
                < *min_cost
                    .get(&(neighbor, new_a_presses, new_b_presses))
                    .unwrap_or(&i32::MAX)
            {
                min_cost.insert((neighbor, new_a_presses, new_b_presses), new_cost);
                heap.push(Reverse((
                    new_cost,
                    neighbor,
                    new_depth,
                    new_a_presses,
                    new_b_presses,
                )));
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example.txt");
        let result = part1(input);
        assert_eq!(result, 480 as i32);
    }
}
