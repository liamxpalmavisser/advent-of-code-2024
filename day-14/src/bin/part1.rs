use utils::*;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn parse_robots(input: &str) -> Vec<i32> {
    let mut result = Vec::new();
    let mut current_number = String::new();
    let mut negative = false;

    for c in input.chars() {
        if c.is_ascii_digit() {
            current_number.push(c);
        } else if c == '-' && current_number.is_empty() {
            negative = true;
        } else if !current_number.is_empty() {
            let number: i32 = current_number.parse().unwrap();
            result.push(if negative { -number } else { number });
            current_number.clear();
            negative = false;
        }
    }

    if !current_number.is_empty() {
        let number: i32 = current_number.parse().unwrap();
        result.push(if negative { -number } else { number });
    }

    result
}

type Robot = [i32; 4];

fn parse(input: &str) -> Vec<Robot> {
    parse_robots(input)
        .chunks(4)
        .map(|chunk| [chunk[0], chunk[1], chunk[2], chunk[3]])
        .collect()
}

fn get_quadrant(width: i32, height: i32, (x, y): (i32, i32)) -> Option<u8> {
    let half_width = width / 2;
    let half_height = height / 2;

    match (x >= 0, x < width, y >= 0, y < height) {
        (true, true, true, true) => match (x, y) {
            (x, _) if x == half_width => None,
            (_, y) if y == half_height => None,
            (x, y) if x < half_width && y < half_height => Some(1),
            (x, y) if x > half_width && y < half_height => Some(2),
            (x, y) if x < half_width && y > half_height => Some(3),
            (x, y) if x > half_width && y > half_height => Some(4),
            _ => None,
        },
        _ => None,
    }
}

fn compute(input: &[Robot], width: i32, height: i32) -> i32 {
    let mut n_in_quadrant = [0; 4];

    for [x, y, dx, dy] in input {
        let new_x = (x + 100 * dx).rem_euclid(width);
        let new_y = (y + 100 * dy).rem_euclid(height);

        if let Some(quadrant) = get_quadrant(width, height, (new_x, new_y)) {
            n_in_quadrant[(quadrant - 1) as usize] += 1;
        }
    }

    n_in_quadrant.iter().product()
}

pub fn part2(robots: &[Robot]) -> usize {
    let mut xs = vec![vec![0; robots.len()]; 101];
    let mut ys = vec![vec![0; robots.len()]; 103];
    let mut grid = Grid {
        width: 101,
        height: 103,
        bytes: vec![0; (101 * 103) as usize],
    };

    for (time, row) in xs.iter_mut().enumerate() {
        for (i, [x, _, dx, _]) in robots.iter().enumerate() {
            row[i] = (x + dx * time as i32).rem_euclid(101);
        }
    }

    for (time, row) in ys.iter_mut().enumerate() {
        for (i, [_, y, _, dy]) in robots.iter().enumerate() {
            row[i] = (y + dy * time as i32).rem_euclid(103);
        }
    }

    'outer: for time in 1..10403 {
        for (&x, &y) in xs[time % 101].iter().zip(ys[time % 103].iter()) {
            let point = Point::new(x, y);
            if grid[point] == time {
                continue 'outer;
            }

            grid[point] = time;
        }

        return time;
    }

    unreachable!()
}

fn part1(input: &str) -> i32 {
    let parsed_input = parse(input);
    compute(&parsed_input, 101, 103)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example.txt");
        let result = part1(input);
        assert_eq!(result, 12 as i32);
    }
}
