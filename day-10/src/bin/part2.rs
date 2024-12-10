use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn get_grid(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect()
}

fn get_coord_value(grid: &Vec<Vec<i32>>, coord: (i32, i32)) -> Option<i32> {
    let (y, x) = coord;
    if let Some(column) = grid.get(y as usize) {
        if let Some(value) = column.get(x as usize) {
            return Some(*value);
        }
    }
    None
}

fn get_start_and_end(grid: &Vec<Vec<i32>>) -> (Vec<(i32, i32)>, HashSet<(i32, i32)>) {
    let mut starts = Vec::new();
    let mut ends = HashSet::new();

    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, value) in row.iter().enumerate() {
            match value {
                0 => starts.push((row_idx as i32, col_idx as i32)),
                9 => {
                    ends.insert((row_idx as i32, col_idx as i32));
                }
                _ => {}
            }
        }
    }

    (starts, ends)
}

fn get_neighbours(grid: &Vec<Vec<i32>>, coord: (i32, i32)) -> Vec<(i32, i32)> {
    if let Some(value) = get_coord_value(grid, coord) {
        let (y, x) = coord;

        let potential_neighbours = vec![(y + 1, x), (y - 1, x), (y, x + 1), (y, x - 1)];

        potential_neighbours
            .into_iter()
            .filter(|&neighbor_coord| {
                if let Some(neighbor_value) = get_coord_value(grid, neighbor_coord) {
                    neighbor_value == value + 1
                } else {
                    false
                }
            })
            .collect()
    } else {
        vec![]
    }
}

fn bfs(grid: &Vec<Vec<i32>>, start_coord: (i32, i32), ends: &mut HashSet<(i32, i32)>) -> i32 {
    let mut queue: VecDeque<(i32, i32)> = VecDeque::from(vec![start_coord]);
    let mut score = 0;

    while !queue.is_empty() && !ends.is_empty() {
        if let Some(current) = queue.pop_front() {
            if ends.contains(&current) {
                score += 1;
            }
            let neighbours = get_neighbours(grid, current);
            for neighbour in neighbours {
                queue.push_back(neighbour);
            }
        }
    }
    score
}

fn part1(input: &str) -> i32 {
    let grid = get_grid(input);
    let (starts, ends) = get_start_and_end(&grid);

    let mut scores = Vec::new();
    for start in starts.into_iter() {
        let mut ends_copy = ends.clone();
        scores.push(bfs(&grid, start, &mut ends_copy));
    }
    scores.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example.txt");
        let result = part1(input);
        assert_eq!(result, 81 as i32);
    }
}
