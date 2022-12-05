use advent_2022::{read_aoc_lines, InputIterator, IteratorUtils};
use std::collections::HashSet;

fn char_to_value(c: char) -> i64 {
    let i = c as i64;
    if i >= 97 {
        i - 96
    } else {
        i - 38
    }
}

fn part1<I: InputIterator>(input: I) -> i64 {
    input
        .map(|rucksack| {
            let r = rucksack.as_ref();
            let size = r.len();
            assert_eq!(size % 2, 0);
            let first = r[0..(size / 2)].chars().collect::<HashSet<_>>();
            let second = r[(size / 2)..size].chars().collect::<HashSet<_>>();
            char_to_value(*first.intersection(&second).only().unwrap())
        })
        .sum()
}
fn part2<I: InputIterator>(input: I) -> i64 {
    input
        .fixed_chunks(3, |group| {
            assert_eq!(group.len(), 3);
            let set = group
                .iter()
                .map(|x| x.as_ref().chars().collect::<HashSet<_>>())
                .reduce(|x, y| &x & &y)
                .unwrap();
            char_to_value(*set.iter().only().unwrap())
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::char_to_value;
    use crate::part1;
    use crate::part2;
    #[test]
    fn day3_char_to_value() {
        assert_eq!(char_to_value('a'), 1);
        assert_eq!(char_to_value('z'), 26);
        assert_eq!(char_to_value('A'), 27);
        assert_eq!(char_to_value('Z'), 52);
    }

    #[test]
    fn day3_test() {
        let input = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ];
        assert_eq!(part1(input.iter()), 157);
        assert_eq!(part2(input.iter()), 70);
    }
}

fn main() {
    print!("{}\n", part1(read_aoc_lines!()));
    print!("{}\n", part2(read_aoc_lines!()));
}
