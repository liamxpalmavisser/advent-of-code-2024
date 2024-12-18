use std::collections::{HashSet, VecDeque};

use utils::*;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input, 1024, 70);
    dbg!(output);
}

fn parse(input: &str, n: usize, size: i32) -> (Grid<u8>, Point, Point) {
    let mut grid = Grid::new(size + 1, size + 1, b'.');

    let special_points: Vec<Point> = input
        .lines()
        .take(n)
        .filter_map(|line| {
            let mut parts = line.split(',').map(|p| p.trim().parse::<i32>());
            if let (Some(Ok(x)), Some(Ok(y))) = (parts.next(), parts.next()) {
                Some(Point::new(x, y))
            } else {
                None
            }
        })
        .collect();

    for &point in &special_points {
        grid[point] = b'#';
    }

    (grid, Point::new(0, 0), Point::new(size, size))
}

fn bfs(grid: Grid<u8>, start: Point, end: Point) -> Option<i32> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    visited.insert(start);

    queue.push_back((start, 0));

    while let Some((current, depth)) = queue.pop_front() {
        if current == end {
            return Some(depth);
        }

        for neighbor in current.neighbors() {
            if grid.contains(neighbor) && grid[neighbor] != b'#' && !visited.contains(&neighbor) {
                queue.push_back((neighbor, depth + 1));
                visited.insert(neighbor);
            }
        }
    }
    None
}

fn part1(input: &str, first_n: usize, size: i32) -> i32 {
    let (grid, start, end) = parse(input, first_n, size);
    let depth = bfs(grid, start, end).expect("Should be solution");
    depth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example.txt");
        let result = part1(input, 12, 6);
        assert_eq!(result, 22 as i32);
    }
}
