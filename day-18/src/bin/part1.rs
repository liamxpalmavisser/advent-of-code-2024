use std::collections::{HashSet, VecDeque};

use utils::*;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn parse(input: &str, n: usize) -> Grid<u8> {
    let mut grid = Grid::new(71, 71, b'.');

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

    grid
}

fn bfs(grid: Grid<u8>) -> i32 {
    let mut queue = VecDeque::new();
    let start = Point::new(0, 0);
    let mut visited = HashSet::new();

    queue.push_back((start, 0));

    while let Some((current, depth)) = queue.pop_front() {
        if current == Point::new(70, 70) {
            return depth;
        }

        for neighbor in current.neighbors() {
            if grid.contains(neighbor) && grid[neighbor] != b'#' && !visited.contains(&neighbor) {
                queue.push_back((neighbor, depth + 1));
                visited.insert(neighbor);
            }
        }
    }
    unreachable!()
}

fn part1(input: &str) -> i32 {
    let grid = parse(input, 1024);
    let depth = bfs(grid);
    depth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example.txt");
        let result = part1(input);
        assert_eq!(result, 22 as i32);
    }
}
