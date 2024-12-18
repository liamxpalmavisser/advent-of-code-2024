use std::{cmp::Reverse, collections::BinaryHeap};
mod computer;
use computer::Computer;

fn main() {
    let output = part2();
    dbg!(output);
}

fn part2() -> u64 {
    let program: Vec<u64> = vec![2, 4, 1, 1, 7, 5, 1, 5, 4, 0, 0, 3, 5, 5, 3, 0];
    backtrack(program)
}

fn part2_example() -> u64 {
    let program: Vec<u64> = vec![0, 3, 5, 4, 3, 0];
    backtrack(program)
}

fn backtrack(program: Vec<u64>) -> u64 {
    let mut heap = BinaryHeap::new();
    for i in 1..9 {
        heap.push(Reverse((i as u64, program.len())));
    }

    while let Some(Reverse((num, depth))) = heap.pop() {
        let mut computer = Computer::new(&program, Some(num), None, None);

        if computer.run_once() == program[depth - 1] {
            if depth == 1 {
                return num;
            }
            for a in get_previous_a(num) {
                heap.push(Reverse((a, depth - 1)));
            }
        }
    }
    unreachable!()
}

fn get_previous_a(a: u64) -> Vec<u64> {
    (8 * a..8 * (a + 1)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2_example();
        assert_eq!(result, 117440 as u64);
    }
}
