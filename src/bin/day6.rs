use advent_2022::{read_aoc_lines, InputIterator, OptionUtils};
use anyhow::Result;
use std::collections::HashSet;

fn chars_to_marker(s: &str, n: usize) -> Option<i64> {
    for i in 0..(s.len() - n) {
        if s[i..(i + n)].chars().collect::<HashSet<_>>().len() == n {
            return Some((i + n) as i64);
        }
    }
    None
}

fn part1<I: InputIterator>(input: I) -> Result<i64> {
    input
        .map(|s| chars_to_marker(s.as_ref(), 4))
        .try_fold(0, |acc, x| Ok(acc + x.ok_or_err()?))
}
fn part2<I: InputIterator>(input: I) -> Result<i64> {
    input
        .map(|s| chars_to_marker(s.as_ref(), 14))
        .try_fold(0, |acc, x| Ok(acc + x.ok_or_err()?))
}

#[cfg(test)]
mod tests {
    use crate::chars_to_marker;
    #[test]
    fn dayn_test() {
        let input = vec![
            "bvwbjplbgvbhsrlpgdmjqwftvncz",
            "nppdvjthqldpwncqszvftbrmjlhg",
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
        ];
        assert_eq!(chars_to_marker(input[0], 4).unwrap(), 5);
        assert_eq!(chars_to_marker(input[1], 4).unwrap(), 6);
        assert_eq!(chars_to_marker(input[2], 4).unwrap(), 10);
        assert_eq!(chars_to_marker(input[3], 4).unwrap(), 11);

        assert_eq!(chars_to_marker(input[0], 14).unwrap(), 23);
        assert_eq!(chars_to_marker(input[1], 14).unwrap(), 23);
        assert_eq!(chars_to_marker(input[2], 14).unwrap(), 29);
        assert_eq!(chars_to_marker(input[3], 14).unwrap(), 26);
    }
}

fn main() {
    print!("{:?}\n", part1(read_aoc_lines!()));
    print!("{:?}\n", part2(read_aoc_lines!()));
}
