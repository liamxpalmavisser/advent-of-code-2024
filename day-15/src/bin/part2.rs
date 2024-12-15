use std::{collections::VecDeque, mem::swap};

use utils::*;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn parse(input: &str) -> (Grid<u8>, &str) {
    let (griddy, second_part) = input
        .split_once("\n\n")
        .expect("This should parse correctly");
    let grid = Grid::parse(griddy);

    (grid, second_part)
}

#[derive(Debug)]
struct DoublePoint {
    points: (Point, Point),
}

fn move_vert(grid: &mut Grid<u8>, position: &mut Point, dir: Point) {
    let mut queue: VecDeque<DoublePoint> = VecDeque::new();
    let mut visited: VecDeque<DoublePoint> = VecDeque::new();
    let new_position = *position + dir;

    match grid[new_position] {
        b'#' => return,
        b'[' => {
            queue.push_front(DoublePoint {
                points: (*position + dir, *position + dir + RIGHT),
            });
            visited.push_front(DoublePoint {
                points: (*position + dir, *position + dir + RIGHT),
            })
        }
        b']' => {
            queue.push_front(DoublePoint {
                points: (*position + dir + LEFT, *position + dir),
            });
            visited.push_front(DoublePoint {
                points: (*position + dir + LEFT, *position + dir),
            })
        }
        _ => (),
    }

    while let Some(new_position) = queue.pop_back() {
        let (position_1, position_2) = new_position.points;
        match (grid[position_1 + dir], grid[position_2 + dir]) {
            (b'#', _) => return,
            (_, b'#') => return,
            (b'[', b']') => {
                queue.push_front(DoublePoint {
                    points: (position_1 + dir, position_2 + dir),
                });
                visited.push_front(DoublePoint {
                    points: (position_1 + dir, position_2 + dir),
                });
            }
            (b']', b'.') => {
                queue.push_front(DoublePoint {
                    points: (position_1 + dir + LEFT, position_1 + dir),
                });
                visited.push_front(DoublePoint {
                    points: (position_1 + dir + LEFT, position_1 + dir),
                });
            }
            (b'.', b'[') => {
                queue.push_front(DoublePoint {
                    points: (position_2 + dir, position_2 + dir + RIGHT),
                });
                visited.push_front(DoublePoint {
                    points: (position_2 + dir, position_2 + dir + RIGHT),
                });
            }
            (b']', b'[') => {
                queue.push_front(DoublePoint {
                    points: (position_1 + dir + LEFT, position_1 + dir),
                });
                visited.push_front(DoublePoint {
                    points: (position_1 + dir + LEFT, position_1 + dir),
                });
                queue.push_front(DoublePoint {
                    points: (position_2 + dir, position_2 + dir + RIGHT),
                });
                visited.push_front(DoublePoint {
                    points: (position_2 + dir, position_2 + dir + RIGHT),
                });
            }
            (_, _) => continue,
        }
    }

    while let Some(positions) = visited.pop_front() {
        let (position1, position2) = positions.points;
        let pos1_dir = position1 + dir;
        let pos2_dir = position2 + dir;

        let temp1 = grid[pos1_dir];
        grid[pos1_dir] = grid[position1];
        grid[position1] = temp1;

        let temp2 = grid[pos2_dir];
        grid[pos2_dir] = grid[position2];
        grid[position2] = temp2;
    }

    swap(&mut b'.', &mut grid[*position]);

    *position += dir;
}

fn move_one(grid: &mut Grid<u8>, position: &mut Point, dir: Point) {
    let mut new_position = *position + dir;
    let mut size = 2;

    loop {
        match grid[new_position] {
            b'#' => return,
            b'.' => break,
            _ => {
                new_position += dir;
                size += 1;
            }
        }
    }

    let mut previous = b'.';
    let mut new_position = *position;

    for _ in 0..size {
        swap(&mut previous, &mut grid[new_position]);
        new_position += dir;
    }

    *position += dir;
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
        match dir {
            LEFT | RIGHT => move_one(grid, &mut start, dir),
            UP | DOWN => move_vert(grid, &mut start, dir),
            _ => continue,
        }
    }
}

fn part2(input: &str) -> i32 {
    let (grid, moves) = parse(input);
    let mut grid = stretch(&grid);

    submarine_go(&mut grid, moves);

    let zero_points = &grid.find_all(b'[');
    let mut result = 0;
    for zero in zero_points.into_iter() {
        let score = 100 * zero.y + zero.x;
        result += score;
    }
    result
}

fn stretch(grid: &Grid<u8>) -> Grid<u8> {
    let mut next = Grid::new(grid.width * 2, grid.height, b'.');

    for y in 0..grid.height {
        for x in 0..grid.width {
            let (left, right) = match grid[Point::new(x, y)] {
                b'#' => (b'#', b'#'),
                b'O' => (b'[', b']'),
                b'@' => (b'@', b'.'),
                _ => continue,
            };

            next[Point::new(2 * x, y)] = left;
            next[Point::new(2 * x + 1, y)] = right;
        }
    }

    next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example.txt");
        let result = part2(input);
        assert_eq!(result, 9021 as i32);
    }
}
