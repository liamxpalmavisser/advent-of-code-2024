use priority_queue::PriorityQueue;
use std::collections::HashMap;

use utils::*;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

fn dijkstra(grid: Grid<u8>) -> Option<i32> {
    let mut heap = PriorityQueue::new();
    let start: Point = grid.find(b'S').expect("Start should be there");
    let dir = RIGHT;

    let end: Point = grid.find(b'E').expect("End should be there");

    heap.push((start, dir), 0);

    let mut cost_so_far = HashMap::new();

    cost_so_far.insert(start, 0);

    while let Some(((current, current_dir), current_priority)) = heap.pop() {
        let current_cost = -current_priority;

        if current == end {
            break;
        }

        for new_dir in &[UP, LEFT, DOWN, RIGHT] {
            let next = current + *new_dir;
            if grid[next] != b'#' {
                let new_cost = {
                    if current_dir == *new_dir {
                        current_cost + 1
                    } else {
                        current_cost + 1001
                    }
                };
                cost_so_far.insert(next, new_cost);
                heap.push((next, *new_dir), -new_cost);
            }
        }
    }
    cost_so_far.get(&end).cloned()
}

fn part1(input: &str) -> i32 {
    let grid = parse(input);

    dijkstra(grid).expect("There should be an outcome")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example.txt");
        let result = part1(input);
        assert_eq!(result, 11048 as i32);
    }
}
