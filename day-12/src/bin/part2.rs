use std::collections::{HashSet, VecDeque};

use utils::*;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn parse(input: &str) -> Grid<u8> {
    let grid = Grid::parse(input);
    grid
}

fn flood_fill(grid: &Grid<u8>, point: Point, visited: &mut HashSet<Point>) -> (i32, i32) {
    let mut queue: VecDeque<Point> = VecDeque::new();
    let mut points_visited = 0;

    let mut perimeter = 0;

    queue.push_front(point);
    visited.insert(point);

    while let Some(current_point) = queue.pop_back() {
        points_visited += 1;

        for neighbor in current_point.neighbors() {
            if grid.contains(neighbor) {
                if grid[current_point] == grid[neighbor] && !visited.contains(&neighbor) {
                    queue.push_front(neighbor);
                    visited.insert(neighbor);
                } else if grid[current_point] != grid[neighbor] {
                    perimeter += 1;
                }
            } else {
                perimeter += 1;
            }
        }
    }

    return (points_visited, perimeter);
}

fn calculate_cost(grid: &Grid<u8>) -> i32 {
    let mut visited = HashSet::new();
    let mut total_cost = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            let point = Point::new(x, y);
            if !visited.contains(&point) {
                let (area, perimeter) = flood_fill(grid, point, &mut visited);
                total_cost += area * perimeter
            }
        }
    }

    total_cost
}

fn part1(input: &str) -> i32 {
    let grid = parse(input);
    calculate_cost(&grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example.txt");
        let result = part1(input);
        assert_eq!(result, 1930 as i32);
    }
}
