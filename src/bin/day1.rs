use advent_2022::{InputIterator, read_aoc_lines};
use std::cmp::max;
use std::num::ParseIntError;

fn get_elf_calories<S: AsRef<str>>(elf: &Vec<S>) -> Result<i64, ParseIntError> {
    elf.iter()
        .map(|snack| snack.as_ref().parse::<i64>())
        .try_fold(0, |acc, x| -> Result<i64, ParseIntError> { Ok(acc + x?) })
}

fn part1<I: InputIterator>(input: I) -> Result<i64, ParseIntError> {
    input
        .blank_chunks(|elf| get_elf_calories(elf))
        .try_fold(0, |acc, x| -> Result<i64, ParseIntError> {
            Ok(max(acc, x?))
        })
}

fn part2<I: InputIterator>(input: I) -> Result<i64, ParseIntError> {
    input
        .blank_chunks(|elf| get_elf_calories(elf))
        .try_fold(
            &mut vec![0i64; 3],
            |acc, x| -> Result<&mut Vec<i64>, ParseIntError> {
                let new_val = x?;
                let min_index = acc.iter().enumerate().min_by_key(|v| v.1).unwrap().0;
                if acc[min_index] < new_val {
                    acc[min_index] = new_val;
                }
                Ok(acc)
            },
        )
        .map(|x| x.iter().sum())
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;
    #[test]
    fn day1_test() {
        let input = vec![
            "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
            "10000",
        ];
        assert_eq!(part1(input.iter()).unwrap(), 24000);
        assert_eq!(part2(input.iter()).unwrap(), 45000);
    }
}


fn main() {
    print!("{:?}\n", part1(read_aoc_lines!()));
    print!("{:?}\n", part2(read_aoc_lines!()));
}
