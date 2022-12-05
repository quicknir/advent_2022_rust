use advent_2022::{InputIterator, read_aoc_lines};
use anyhow::Result;

fn part1<I: InputIterator>(_input: I) -> Result<i64> {
    Ok(0)
}
fn part2<I: InputIterator>(_input: I) -> Result<i64> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;
    #[test]
    fn dayn_test() {
        let input = vec![""];
        assert_eq!(part1(input.iter()).unwrap(), 0);
        assert_eq!(part2(input.iter()).unwrap(), 0);
    }
}

fn main() {
    print!("{:?}\n", part1(read_aoc_lines!()));
    print!("{:?}\n", part2(read_aoc_lines!()));
}
