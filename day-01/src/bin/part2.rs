use std::collections::{hash_map, HashMap};

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn create_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_columns(grid: Vec<Vec<char>>) -> (Vec<u32>, Vec<u32>) {
    let second_column: Vec<u32> = grid
        .iter()
        .filter_map(|row| row.get(8..13))
        .map(|slice| slice.iter().collect::<String>().parse::<u32>().unwrap())
        .collect();
    let first_column: Vec<u32> = grid
        .iter()
        .filter_map(|row| row.get(0..5))
        .map(|slice| slice.iter().collect::<String>().parse::<u32>().unwrap())
        .collect();

    (first_column, second_column)
}

fn get_columns_example(grid: Vec<Vec<char>>) -> (Vec<u32>, Vec<u32>) {
    let first_column: Vec<u32> = grid
        .iter()
        .filter_map(|row| row.get(0))
        .map(|ch| ch.to_digit(10).expect("No digit"))
        .collect();
    let second_column: Vec<u32> = grid
        .iter()
        .filter_map(|row| row.get(4))
        .map(|ch| ch.to_digit(10).expect("No digit"))
        .collect();

    (first_column, second_column)
}

fn count_to_hash(column: Vec<u32>) -> HashMap<u32, u32> {
    let mut hash_counter: HashMap<u32, u32> = HashMap::new();

    for n in column.iter() {
        match hash_counter.get(n) {
            Some(count) => hash_counter.insert(*n, count + 1),
            None => hash_counter.insert(*n, 1),
        };
    }
    hash_counter
}

fn multiply_cols(col: Vec<u32>, counter: HashMap<u32, u32>) -> u32 {
    let mut sum_col: Vec<u32> = Vec::new();
    for n in col.iter() {
        match counter.get(n) {
            Some(count) => {
                sum_col.push(*n * *count);
            }
            None => (),
        }
    }

    if sum_col.is_empty() {
        panic!("Joe");
    } else {
        sum_col.into_iter().sum()
    }
}

fn part2(_input: &str) -> u32 {
    let grid = create_grid(_input);
    let (first_column, second_column) = get_columns(grid);

    let hash_counter = count_to_hash(second_column);

    multiply_cols(first_column, hash_counter)
}

fn part2_example(_input: &str) -> u32 {
    let grid = create_grid(_input);
    let (first_column, second_column) = get_columns_example(grid);

    let hash_counter = count_to_hash(second_column);

    multiply_cols(first_column, hash_counter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        let result = part2_example(input);
        assert_eq!(result, 31 as u32);
    }
}
