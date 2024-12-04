fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
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

fn is_xmas(grid: &Vec<Vec<char>>, current: (i32, i32)) -> bool {
    println!("{:?}", current);
    match (
        get_coord_value(grid, (current.0 - 1, current.1 - 1)),
        get_coord_value(grid, (current.0 + 1, current.1 + 1)),
    ) {
        (Some('M'), Some('S')) | (Some('S'), Some('M')) => {
            match (
                get_coord_value(grid, (current.0 - 1, current.1 + 1)),
                get_coord_value(grid, (current.0 + 1, current.1 - 1)),
            ) {
                (Some('M'), Some('S')) | (Some('S'), Some('M')) => true,
                _ => false,
            }
        }
        _ => false,
    }
}

fn get_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part2(input: &str) -> u32 {
    let grid = get_grid(input);
    let mut count = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            if ch == 'A' {
                let coord = (y as i32, x as i32);

                if is_xmas(&grid, coord) {
                    println!("{:?}", coord);
                    count += 1;
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
        let result = part2(input);
        assert_eq!(result, 18 as u32);
    }
}
