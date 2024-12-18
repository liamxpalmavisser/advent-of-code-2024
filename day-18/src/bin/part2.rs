use std::collections::{HashSet, VecDeque};

use utils::*;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input, 1024, 70);
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

fn bfs(grid: &mut Grid<u8>, start: Point, end: Point) -> Option<i32> {
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

fn find_obstacle(grid: &mut Grid<u8>, obstacle: Point, start: Point, end: Point) -> bool {
    grid[obstacle] = b'#';

    if bfs(grid, start, end).is_none() {
        return true;
    }

    return false;
}

fn part2(input: &str, first_n: usize, size: i32) -> (i32, i32) {
    let (grid, start, end) = parse(input, first_n, size);
    let mut grid = grid.clone();

    let obstacle_points: Vec<Point> = input
        .lines()
        .skip(first_n)
        .filter_map(|line| {
            let mut parts = line.split(',').map(|p| p.trim().parse::<i32>());
            if let (Some(Ok(x)), Some(Ok(y))) = (parts.next(), parts.next()) {
                Some(Point::new(x, y))
            } else {
                None
            }
        })
        .collect();

    for point in obstacle_points.into_iter() {
        if find_obstacle(&mut grid, point, start, end) {
            return (point.x, point.y);
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example.txt");
        let result = part2(input, 12, 6);
        assert_eq!(result, (6, 1) as (i32, i32));
    }
}
