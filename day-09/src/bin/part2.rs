use std::collections::{BTreeSet, HashMap};

fn main() {
    let input = include_str!("./input.txt");
    let result = process_input(input);
    dbg!(result);
}

fn extract_data(input: &str) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    let mut current_block_size = 0;
    let mut chars = input.chars().enumerate();

    while let Some((index, character)) = chars.next() {
        let value = character.to_digit(10).unwrap() as usize;

        if index % 2 == 0 {
            current_block_size = value;
        } else {
            result.push((current_block_size, value));
        }
    }

    result.push((current_block_size, 0));
    result
}

fn compact_files(blocks: &[(usize, usize)]) -> usize {
    let mut positions: HashMap<usize, (usize, usize)> = HashMap::new();
    let mut free_spaces: BTreeSet<(usize, usize)> = BTreeSet::new();
    let mut cursor_position = 0;

    for (index, (block_size, free_space)) in blocks.iter().enumerate() {
        positions.insert(index, (cursor_position, cursor_position + block_size));
        cursor_position += block_size;

        if *free_space > 0 {
            free_spaces.insert((cursor_position, cursor_position + free_space));
            cursor_position += free_space;
        }
    }

    let mut file_index = blocks.len() - 1;
    while file_index > 0 {
        let (start, end) = positions.get(&file_index).unwrap();
        let block_length = end - start;
        let mut potential_move = None;

        for (free_start, free_end) in &free_spaces {
            if free_start > start {
                break;
            }

            if free_end - free_start >= block_length {
                potential_move = Some((*free_start, *free_end));
                break;
            }
        }

        if let Some((free_start, free_end)) = potential_move {
            let new_end = free_start + block_length;
            positions.insert(file_index, (free_start, new_end));
            free_spaces.remove(&(free_start, free_end));

            if new_end < free_end {
                free_spaces.insert((new_end, free_end));
            }
        }

        file_index -= 1;
    }

    positions
        .into_iter()
        .map(|(index, (start, end))| {
            let sum = (start..end).map(|position| index * position).sum::<usize>();
            sum
        })
        .sum()
}

fn process_input(input: &str) -> usize {
    let blocks = extract_data(input);
    let result = compact_files(&blocks);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compaction() {
        let example_input = include_str!("./example.txt").trim();
        let expected_result = 2858;

        let computed_result = process_input(example_input);
        assert_eq!(computed_result, expected_result);
    }
}
