use std::collections::{HashMap, HashSet, VecDeque};

use utils::*;

fn main() {
    let input = include_str!("./input.txt");
    let result = compute_part2(input);
    dbg!(result);
}

fn parse_input(input: &str) -> (Grid<u8>, Point, Point) {
    let grid = Grid::parse(input);
    let start_point = grid.find(b'S').expect("Start point should exist");
    let end_point = grid.find(b'E').expect("End point should exist");
    (grid, start_point, end_point)
}

fn calculate_distances(grid: &Grid<u8>, start: Point) -> HashMap<Point, i64> {
    let mut distances: HashMap<Point, i64> = HashMap::new();
    let mut exploration_queue = VecDeque::new();

    exploration_queue.push_back((start, 0));
    distances.insert(start, 0);

    while let Some((current_point, current_depth)) = exploration_queue.pop_front() {
        for neighbor in current_point.neighbors() {
            if grid.contains(neighbor)
                && grid[neighbor] != b'#'
                && !distances.contains_key(&neighbor)
            {
                exploration_queue.push_back((neighbor, current_depth + 1));
                distances.insert(neighbor, current_depth + 1);
            }
        }
    }

    distances
}

fn find_reachable_cheats(grid: &Grid<u8>, start: Point) -> HashSet<(Point, i64)> {
    let mut visited_points: HashSet<Point> = HashSet::new();
    let mut exploration_queue = VecDeque::new();
    let mut reachable_cheats = HashSet::new();

    visited_points.insert(start);

    for neighbor in start.neighbors() {
        if grid.contains(neighbor) && !visited_points.contains(&neighbor) && grid[neighbor] == b'#'
        {
            exploration_queue.push_back((neighbor, 1));
            visited_points.insert(neighbor);
        }
    }

    while let Some((current_point, current_depth)) = exploration_queue.pop_front() {
        if current_depth <= 20 {
            if grid[current_point] != b'#' {
                reachable_cheats.insert((current_point, current_depth));
            }
            for neighbor in current_point.neighbors() {
                if grid.contains(neighbor) && !visited_points.contains(&neighbor) {
                    exploration_queue.push_back((neighbor, current_depth + 1));
                    visited_points.insert(neighbor);
                }
            }
        }
    }

    reachable_cheats
}

fn identify_cheat_routes(grid: &Grid<u8>) -> HashMap<Point, HashSet<(Point, i64)>> {
    let mut cheat_start_points = grid.find_all(b'.');
    let start_point = grid.find(b'S').unwrap();
    let end_point = grid.find(b'E').unwrap();

    cheat_start_points.push(start_point);
    cheat_start_points.push(end_point);

    let mut cheat_routes = HashMap::new();
    for cheat_start in cheat_start_points {
        let cheat_end_points = find_reachable_cheats(grid, cheat_start);
        cheat_routes.insert(cheat_start, cheat_end_points);
    }

    cheat_routes
}

fn calculate_valid_paths(input: &str, time_savings: i64) -> i64 {
    let (grid, start_point, end_point) = parse_input(input);
    let start_distances = calculate_distances(&grid, start_point);
    let end_distances = calculate_distances(&grid, end_point);
    let total_path_length = *end_distances.get(&start_point).unwrap();

    let cheat_routes = identify_cheat_routes(&grid);
    let mut valid_path_count = 0;

    for (cheat_start, cheat_ends) in cheat_routes.iter() {
        let &distance_to_cheat_start = start_distances.get(cheat_start).unwrap();
        if distance_to_cheat_start <= total_path_length - time_savings {
            for (cheat_end, cheat_depth) in cheat_ends {
                let &distance_to_cheat_end = end_distances.get(cheat_end).unwrap();
                if distance_to_cheat_start + distance_to_cheat_end + cheat_depth
                    <= total_path_length - time_savings
                {
                    valid_path_count += 1;
                }
            }
        }
    }

    valid_path_count
}

fn compute_part2(input: &str) -> i64 {
    calculate_valid_paths(input, 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_case() {
        let input = include_str!("./example.txt");
        let result = compute_part2(input);
        assert_eq!(result, 14);
    }
}
