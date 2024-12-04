fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn get_coord_value(grid: &Vec<Vec<char>>, coord: (i32, i32)) -> Option<char> {
    let (y, x) = coord;
    if let Some(column) = grid.get(y as usize) {
        if let Some(value) = column.get(x as usize) {
            return Some(*value);
        }
    }
    None
}

fn is_xmas(grid: &Vec<Vec<char>>, current: (i32, i32), dir: (i32, i32), depth: i32) -> bool {
    let next = (current.0 + dir.0, current.1 + dir.1);
    match (get_coord_value(grid, next), depth) {
        (Some('M'), 0) => is_xmas(grid, next, dir, depth + 1),
        (Some('A'), 1) => is_xmas(grid, next, dir, depth + 1),
        (Some('S'), 2) => true,
        _ => false,
    }
}

fn get_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part1(input: &str) -> u32 {
    let grid = get_grid(input);
    let directions = [
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
    ];
    let mut count = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            if ch == 'X' {
                let coord = (y as i32, x as i32);

                for &dir in &directions {
                    if is_xmas(&grid, coord, dir, 0) {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example.txt");
        let result = part1(input);
        assert_eq!(result, 18 as u32);
    }
}
