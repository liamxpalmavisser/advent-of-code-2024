use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    fn turn_right(&self) -> Dir {
        match self {
            Dir::N => Dir::E,
            Dir::E => Dir::S,
            Dir::S => Dir::W,
            Dir::W => Dir::N,
        }
    }

    fn next_coord(&self, coord: (i32, i32)) -> (i32, i32) {
        match self {
            Dir::N => (coord.0 - 1, coord.1),
            Dir::E => (coord.0, coord.1 + 1),
            Dir::S => (coord.0 + 1, coord.1),
            Dir::W => (coord.0, coord.1 - 1),
        }
    }
}

fn parse_map(input: &str) -> ((i32, i32), Dir, HashSet<(i32, i32)>, (i32, i32)) {
    let mut start_pos = (0, 0);
    let mut start_dir = Dir::N;
    let mut obstacles = HashSet::new();

    let mut height = 0;
    let mut width = 0;

    for (row_idx, row) in input.lines().enumerate() {
        height += 1;
        width = row.len() as i32;
        for (col_idx, ch) in row.chars().enumerate() {
            let coord = (row_idx as i32, col_idx as i32);
            match ch {
                '^' => {
                    start_pos = coord;
                    start_dir = Dir::N;
                }
                '>' => {
                    start_pos = coord;
                    start_dir = Dir::E;
                }
                'v' => {
                    start_pos = coord;
                    start_dir = Dir::S;
                }
                '<' => {
                    start_pos = coord;
                    start_dir = Dir::W;
                }
                '#' => {
                    obstacles.insert(coord);
                }
                _ => {}
            }
        }
    }

    (start_pos, start_dir, obstacles, (height, width))
}

fn simulate_guard(
    start_pos: (i32, i32),
    start_dir: Dir,
    obstacles: HashSet<(i32, i32)>,
    bounds: (i32, i32),
) -> HashSet<(i32, i32)> {
    let mut visited = HashSet::new();
    let mut current_pos = start_pos;
    let mut current_dir = start_dir;

    visited.insert(current_pos);

    loop {
        let next_pos = current_dir.next_coord(current_pos);

        if next_pos.0 < 0 || next_pos.0 >= bounds.0 || next_pos.1 < 0 || next_pos.1 >= bounds.1 {
            break;
        }

        if obstacles.contains(&next_pos) {
            current_dir = current_dir.turn_right();
        } else {
            current_pos = next_pos;
            visited.insert(current_pos);
        }
    }

    visited
}

fn part1(input: &str) -> i32 {
    let input = input.trim();
    let (start_pos, start_dir, obstacles, bounds) = parse_map(input);
    let visited_positions = simulate_guard(start_pos, start_dir, obstacles, bounds);
    visited_positions.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example.txt");
        let result = part1(input);
        assert_eq!(result, 41 as i32);
    }
}
