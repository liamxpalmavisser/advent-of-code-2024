mod computer;
use computer::Computer;

fn main() {
    let output = part1();
    dbg!(output);
}

fn part1() -> Vec<u64> {
    let mut computer = Computer::new(
        &[2, 4, 1, 1, 7, 5, 1, 5, 4, 0, 0, 3, 5, 5, 3, 0],
        Some(64196994),
        None,
        None,
    );
    computer.run();
    computer.outputs
}

fn part1_example() -> Vec<u64> {
    let mut computer = Computer::new(&[0, 1, 5, 4, 3, 0], Some(729), None, None);
    computer.run();
    computer.outputs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let results = part1_example();
        assert_eq!(results, vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
    }
}
