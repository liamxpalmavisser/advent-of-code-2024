use std::{
    char,
    collections::{BTreeMap, VecDeque},
};

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn parse_disk_map(disk_map_str: &str) -> (BTreeMap<usize, i64>, VecDeque<usize>) {
    let disk_map: Vec<char> = disk_map_str.chars().collect();
    let mut files_position_map = BTreeMap::new();
    let mut empty_spots = VecDeque::new();

    let mut index = 0;
    let mut file_id = 0;
    let mut is_file = true;

    for &ch in disk_map.iter() {
        let count = ch.to_digit(10).unwrap_or(0) as usize;

        if is_file {
            for _ in 0..count {
                files_position_map.insert(index, file_id);
                index += 1;
            }
            if count > 0 {
                file_id += 1;
            }
        } else {
            for _ in 0..count {
                empty_spots.push_back(index);
                index += 1;
            }
        }
        is_file = !is_file;
    }

    (files_position_map, empty_spots)
}
fn move_blocks(mut empty_spots: VecDeque<usize>, mut files_positions: BTreeMap<usize, i64>) -> i64 {
    while !empty_spots.is_empty() {
        if let Some((&position, &placed_file)) = files_positions.iter().next_back() {
            files_positions.remove(&position);

            if let Some(empty_spot) = empty_spots.pop_front() {
                files_positions.insert(empty_spot, placed_file);
            }
        } else {
            break;
        }
    }

    // println!("{:?}", files_positions);

    let moved_blocks: i64 = files_positions
        .values()
        .enumerate()
        .map(|(index, &value)| (index as i64) * value)
        .sum();
    moved_blocks
}

fn part1(input: &str) -> i64 {
    let (files_position_map, empty_spots) = parse_disk_map(input);

    move_blocks(empty_spots, files_position_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example.txt");
        let result = part1(input);
        assert_eq!(result, 1928 as i64);
    }
}
