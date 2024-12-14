use utils::*;

fn main() {
    let input = include_str!("./input.txt");
    dbg!(part2(input));
}

fn parse_robot(input: &str) -> Vec<i32> {
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
    parse_robot(input)
        .chunks(4)
        .map(|chunk| [chunk[0], chunk[1], chunk[2], chunk[3]])
        .collect()
}

fn robot_move(robots: Vec<Robot>) -> Option<usize> {
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

        return Some(time);
    }

    None
}

fn part2(input: &str) -> usize {
    let robots = parse(input);
    if let Some(result) = robot_move(robots) {
        result
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example.txt");
        assert_eq!(part2(input), 12);
    }
}
