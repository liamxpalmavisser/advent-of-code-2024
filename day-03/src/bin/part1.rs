fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(_input: &str) {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example.txt");
        let result = part1(input);
        assert_eq!(result, _ as i32);
    }
}
