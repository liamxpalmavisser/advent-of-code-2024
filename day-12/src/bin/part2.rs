use std::collections::{HashMap, HashSet, VecDeque};
use utils::*;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn flood_fill(grid: &Grid<u8>, start_point: Point, visited: &mut HashSet<Point>) -> u32 {
    let target_value = grid[start_point];
    let mut todo = VecDeque::new();

    let mut corner_points = std::collections::HashMap::new();
    let mut adjacent_points = std::collections::HashMap::new();

    let mut region_size = 0;

    todo.push_back(start_point);
    visited.insert(start_point);

    while let Some(point) = todo.pop_front() {
        region_size += 1;

        let x = 2 * point.x;
        let y = 2 * point.y;

        add_to_points(&mut corner_points, x, y);
        add_to_points(&mut corner_points, x + 2, y);
        add_to_points(&mut corner_points, x, y + 2);
        add_to_points(&mut corner_points, x + 2, y + 2);

        add_to_points(&mut adjacent_points, x + 1, y);
        add_to_points(&mut adjacent_points, x, y + 1);
        add_to_points(&mut adjacent_points, x + 2, y + 1);
        add_to_points(&mut adjacent_points, x + 1, y + 2);

        for next in point.neighbors() {
            if grid.contains(next) && grid[next] == target_value && !visited.contains(&next) {
                visited.insert(next);
                todo.push_back(next);
            }
        }
    }

    corner_points.retain(|_, v| *v < 4);
    adjacent_points.retain(|_, v| *v < 2);

    let mut side_count = 0;

    for &point in corner_points.keys() {
        let mut adjacent_sides = 0;
        let directions = vec![
            Point::new(0, -1),
            Point::new(1, 0),
            Point::new(0, 1),
            Point::new(-1, 0),
        ];

        for i in 0..4 {
            let first_adj = Point::new(point.x + directions[i].x, point.y + directions[i].y);
            let second_adj = Point::new(
                point.x + directions[(i + 1) % 4].x,
                point.y + directions[(i + 1) % 4].y,
            );

            if adjacent_points.contains_key(&first_adj) && adjacent_points.contains_key(&second_adj)
            {
                adjacent_sides += 1;
            }
        }

        if adjacent_sides == 1 {
            side_count += 1;
        } else if adjacent_sides == 4 {
            side_count += 2;
        }
    }

    region_size * side_count
}

fn add_to_points(points_map: &mut HashMap<Point, u32>, x: i32, y: i32) {
    *points_map.entry(Point::new(x, y)).or_insert(0) += 1;
}

fn calculate_cost(grid: &Grid<u8>) -> u32 {
    let mut visited = HashSet::new();
    let mut total_cost = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            let point = Point::new(x, y);
            if !visited.contains(&point) {
                let cost = flood_fill(grid, point, &mut visited);
                total_cost += cost;
            }
        }
    }

    total_cost
}

fn part2(input: &str) -> u32 {
    let grid = Grid::parse(input);
    calculate_cost(&grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example.txt");
        let result = part2(input);
        assert_eq!(result, 1206);
    }
}
