use std::collections::{HashSet, VecDeque};
use utils::*;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn get_starts_ends(grid: &Grid<u8>) -> (Vec<Point>, HashSet<Point>) {
    let start_points = grid.find_all(b'0');
    let n_ends = grid.find_all(b'9').into_iter().collect();
    (start_points, n_ends)
}

fn byte_to_i32(byte: u8) -> i32 {
    (byte - b'0') as i32
}

fn bfs(grid: &Grid<u8>, start: Point, ends: &HashSet<Point>) -> i32 {
    let mut queue: VecDeque<Point> = VecDeque::from(vec![start]);
    let mut visited: HashSet<Point> = HashSet::new();
    let mut score = 0;

    visited.insert(start);

    while let Some(current_point) = queue.pop_front() {
        let neighbors = current_point.neighbors();
        let current_value = grid[current_point];

        if ends.contains(&current_point) {
            score += 1;
        }

        for neighbor in neighbors.into_iter() {
            if grid.contains(neighbor)
                && byte_to_i32(grid[neighbor]) == byte_to_i32(current_value) + 1
                && !visited.contains(&neighbor)
            {
                visited.insert(neighbor);
                queue.push_back(neighbor);
            }
        }
    }

    score
}

fn part1(input: &str) -> i32 {
    let grid = Grid::parse(input);
    let (start_points, ends) = get_starts_ends(&grid);

    start_points
        .into_iter()
        .map(|start| {
            let ends_clone = ends.clone();
            bfs(&grid, start, &ends_clone)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example.txt");
        let result = part1(input);
        assert_eq!(result, 36);
    }
}
