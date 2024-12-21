use std::collections::{HashMap, HashSet, VecDeque};

use utils::*;

fn main() {
    let input = include_str!("./input.txt");
    let result = calculate_part1(input);
    dbg!(result);
}

fn parse_input(input: &str) -> (Grid<u8>, Point, Point) {
    let grid = Grid::parse(input);
    let start_point = grid.find(b'S').expect("Start point should exist");
    let end_point = grid.find(b'E').expect("End point should exist");
    (grid, start_point, end_point)
}

fn compute_distances(grid: &Grid<u8>, start: Point) -> HashMap<Point, i32> {
    let mut distance_map: HashMap<Point, i32> = HashMap::new();
    let mut exploration_queue = VecDeque::new();

    exploration_queue.push_back((start, 0));
    distance_map.insert(start, 0);

    while let Some((current_point, current_distance)) = exploration_queue.pop_front() {
        for neighbor in current_point.neighbors() {
            if grid.contains(neighbor)
                && grid[neighbor] != b'#'
                && !distance_map.contains_key(&neighbor)
            {
                exploration_queue.push_back((neighbor, current_distance + 1));
                distance_map.insert(neighbor, current_distance + 1);
            }
        }
    }

    distance_map
}

fn find_cheat_ends(grid: &Grid<u8>, start: Point) -> HashSet<Point> {
    let mut visited_points: HashSet<Point> = HashSet::new();
    let mut exploration_queue = VecDeque::new();
    let mut reachable_points = HashSet::new();

    exploration_queue.push_back((start, 0));
    visited_points.insert(start);

    for neighbor in start.neighbors() {
        if grid.contains(neighbor) && !visited_points.contains(&neighbor) && grid[neighbor] == b'#'
        {
            exploration_queue.push_back((neighbor, 1));
            visited_points.insert(neighbor);
        }
    }

    while let Some((current_point, depth)) = exploration_queue.pop_front() {
        if depth > 2 {
            break;
        }
        if grid[current_point] != b'#' {
            reachable_points.insert(current_point);
        } else {
            for neighbor in current_point.neighbors() {
                if grid.contains(neighbor) && !visited_points.contains(&neighbor) {
                    exploration_queue.push_back((neighbor, depth + 1));
                    visited_points.insert(neighbor);
                }
            }
        }
    }

    reachable_points
}

fn identify_cheat_pairs(grid: &Grid<u8>) -> HashMap<Point, HashSet<Point>> {
    let cheat_start_points = grid.find_all(b'.');
    let mut cheat_pairs = HashMap::new();

    for cheat_start in cheat_start_points {
        let cheat_end_points = find_cheat_ends(grid, cheat_start);
        cheat_pairs.insert(cheat_start, cheat_end_points);
    }

    cheat_pairs
}

fn calculate_part1(input: &str) -> i32 {
    let (grid, start_point, end_point) = parse_input(input);
    let start_distances = compute_distances(&grid, start_point);
    let end_distances = compute_distances(&grid, end_point);
    let total_path_length = *end_distances.get(&start_point).unwrap();

    let cheat_pairs = identify_cheat_pairs(&grid);
    let mut valid_path_count = 0;

    for (cheat_start, cheat_ends) in cheat_pairs.iter() {
        let &start_to_cheat_distance = start_distances.get(cheat_start).unwrap();
        if start_to_cheat_distance <= total_path_length - 100 {
            for cheat_end in cheat_ends {
                let &cheat_to_end_distance = end_distances.get(cheat_end).unwrap();
                if start_to_cheat_distance + cheat_to_end_distance + 2 <= total_path_length - 100 {
                    valid_path_count += 1;
                }
            }
        }
    }

    valid_path_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_case() {
        let input = include_str!("./example.txt");
        let result = calculate_part1(input);
        assert_eq!(result, 14);
    }
}
