use std::mem::swap;

use utils::*;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn parse(input: &str) -> (Grid<u8>, &str) {
    let (griddy, second_part) = input
        .split_once("\n\n")
        .expect("This should parse correctly");
    let grid = Grid::parse(griddy);

    (grid, second_part)
}

fn move_one(grid: &mut Grid<u8>, start: &mut Point, direction: Point) {
    let mut position = *start + direction;
    let mut size = 2;

    loop {
        match grid[position] {
            b'#' => return,
            b'.' => break,
            _ => {
                position += direction;
                size += 1;
            }
        }
    }

    let mut previous = b'.';
    let mut position = *start;

    for _ in 0..size {
        swap(&mut previous, &mut grid[position]);
        position += direction;
    }

    *start += direction;
}

fn submarine_go(grid: &mut Grid<u8>, moves: &str) {
    let mut start = grid.find(b'@').expect("Should be @");

    for i in moves.bytes() {
        let dir = match i {
            b'<' => LEFT,
            b'>' => RIGHT,
            b'^' => UP,
            b'v' => DOWN,
            _ => continue,
        };

        move_one(grid, &mut start, dir);
    }
}

fn part1(input: &str) -> i32 {
    let (mut grid, moves) = parse(input);

    submarine_go(&mut grid, moves);

    let zero_points = &grid.find_all(b'O');
    let mut result = 0;
    for zero in zero_points.into_iter() {
        let score = 100 * zero.y + zero.x;
        result += score;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example.txt");
        let result = part1(input);
        assert_eq!(result, 10092 as i32);
    }
}
