fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn create_grid(lines: &str) -> Vec<Vec<char>> {
    lines.lines().map(|line| line.chars().collect()).collect()
}

fn get_columns(grid: Vec<Vec<char>>) -> (Vec<u32>, Vec<u32>) {
    let second_column: Vec<u32> = grid
        .iter()
        .filter_map(|row| row.get(8..13))
        .map(|slice| {slice.iter().collect::<String>().parse::<u32>().unwrap()})
        .collect();
    let first_column: Vec<u32> = grid
        .iter()
        .filter_map(|row| row.get(0..5))
        .map(|slice| {slice.iter().collect::<String>().parse::<u32>().unwrap()})
        .collect();

    (first_column, second_column)
}

fn sort_elements(mut column: Vec<u32>) -> Vec<u32> {
    column.sort_unstable_by(|a, b| b.cmp(a));
    column
}

fn get_differences(column_1: Vec<u32>, column_2: Vec<u32>) -> u32 {
    let mut distance_vec: Vec<u32> = Vec::new();
    for (a, b) in column_1.iter().zip(column_2.iter()) {
        distance_vec.push(a.abs_diff(*b))
    }
    let total_distance: u32 = distance_vec.iter().sum();
    total_distance
}

fn part1(_input: &str) -> u32 {
    let grid = create_grid(_input);
    let (first_column, second_column) = get_columns(grid);
    let first_sorted_column = sort_elements(first_column);
    let second_sorted_column = sort_elements(second_column);

    let total_differences = get_differences(first_sorted_column, second_sorted_column);
    total_differences
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example2.txt");
        let result = part1(input);
        assert_eq!(result, 11 as u32);
    }
}
