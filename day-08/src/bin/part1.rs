use std::{
    char,
    collections::{HashMap, HashSet},
};

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn parse_map(input: &str) -> (HashMap<char, Vec<(i32, i32)>>, (i32, i32)) {
    let mut antennas = HashMap::new();

    let mut height = 0;
    let mut width = 0;

    for (row_idx, row) in input.lines().enumerate() {
        height += 1;
        width = row.len() as i32;
        for (col_idx, ch) in row.chars().enumerate() {
            let coord = (row_idx as i32, col_idx as i32);
            if ch != '.' {
                antennas.entry(ch).or_insert_with(Vec::new).push(coord);
            }
        }
    }

    (antennas, (height - 1, width - 1))
}

fn in_bounds(bounds: &(i32, i32), coord: &(i32, i32)) -> bool {
    coord.0 >= 0 && coord.0 <= bounds.0 && coord.1 >= 0 && coord.1 <= bounds.1
}

fn find_antinodes(
    coord_list: Vec<(i32, i32)>,
    mut covered_antinodes: HashSet<(i32, i32)>,
    bounds: &(i32, i32),
) -> HashSet<(i32, i32)> {
    for i in 0..coord_list.len() {
        for j in 0..coord_list.len() {
            if i != j {
                let potential_antinode = (
                    2 * coord_list[j].0 - coord_list[i].0,
                    2 * coord_list[j].1 - coord_list[i].1,
                );
                if in_bounds(bounds, &potential_antinode) {
                    covered_antinodes.insert(potential_antinode);
                }
            }
        }
    }

    covered_antinodes
}

fn part1(input: &str) -> i32 {
    let (antennas, bounds) = parse_map(input);
    let mut covered_antinodes = HashSet::new();

    for antenna in antennas.into_values() {
        covered_antinodes = find_antinodes(antenna, covered_antinodes, &bounds)
    }

    covered_antinodes.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example.txt");
        let result = part1(input);
        assert_eq!(result, 14 as i32);
    }
}
