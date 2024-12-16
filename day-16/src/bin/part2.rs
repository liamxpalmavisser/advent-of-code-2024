use priority_queue::PriorityQueue;
use std::collections::{HashMap, HashSet, VecDeque};

use utils::*;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

fn dijkstra(grid: Grid<u8>) -> (HashMap<(Point, Point), i32>, Point, Point, i32) {
    let mut heap = PriorityQueue::new();
    let start: Point = grid.find(b'S').expect("Start should be there");
    let dir = RIGHT;

    let end: Point = grid.find(b'E').expect("End should be there");
    let mut lowest = i32::MAX;

    heap.push((start, dir), 0);

    let mut cost_so_far = HashMap::new();

    cost_so_far.insert((start, RIGHT), 0);

    while let Some(((current, current_dir), current_priority)) = heap.pop() {
        let current_cost = -current_priority;

        if current == end {
            lowest = lowest.min(current_cost);
            continue;
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
                if !cost_so_far.contains_key(&(next, *new_dir))
                    || if let Some(cost) = cost_so_far.get(&(next, *new_dir)).cloned() {
                        new_cost < cost
                    } else {
                        false
                    }
                {
                    cost_so_far.insert((next, *new_dir), new_cost);
                    heap.push((next, *new_dir), -new_cost);
                }
            }
        }
    }

    (cost_so_far, end, start, lowest)
}

fn backwards_bfs(
    cost_dict: HashMap<(Point, Point), i32>,
    end: Point,
    start: Point,
    lowest: i32,
) -> i32 {
    let mut queue = VecDeque::new();
    let mut path = HashSet::new();

    for dir in &[UP, RIGHT, DOWN, LEFT] {
        if let Some(&cost) = cost_dict.get(&(end, *dir)) {
            if cost == lowest {
                queue.push_back((end, *dir, lowest));
            }
        }
    }

    while let Some((current, dir, cost)) = queue.pop_front() {
        path.insert(current);

        if current == start {
            continue;
        }

        for new_dir in &[UP, RIGHT, DOWN, LEFT] {
            let next = current - dir;

            let new_cost = {
                if dir == *new_dir {
                    cost - 1
                } else {
                    cost - 1001
                }
            };

            if let Some(&cost_vis) = cost_dict.get(&(next, *new_dir)) {
                if cost_vis == new_cost {
                    queue.push_back((next, *new_dir, cost_vis));
                }
            }
        }
    }

    path.len() as i32
}

fn part2(input: &str) -> i32 {
    let grid = parse(input);

    let (cost_dict, end, start, lowest) = dijkstra(grid);

    let n_steps = backwards_bfs(cost_dict, end, start, lowest);

    n_steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example.txt");
        let result = part2(input);
        assert_eq!(result, 45 as i32);
    }
}
