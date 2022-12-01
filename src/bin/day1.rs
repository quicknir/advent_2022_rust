use std::cmp::max;
use std::fs::File;
use std::num::ParseIntError;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P: AsRef<Path>>(filename: P) -> impl Iterator<Item = String> {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines().map(|x| x.unwrap())
}

trait InputUtils {
    fn group_lines(self) -> Vec<Vec<String>>;
}

impl<T: Iterator<Item = String>> InputUtils for T {
    fn group_lines(self) -> Vec<Vec<String>> {
        let mut v = vec![Vec::<String>::new()];

        for s in self {
            if s == "" {
                v.push(Default::default());
                continue;
            }
            v.last_mut().unwrap().push(s);
        }

        return v;
    }
}


fn part1<T: Iterator<Item = String>>(input: T) -> Result<i64, ParseIntError> {
    input
        .group_lines()
        .iter()
        .map(|elf| {
            elf.iter()
                .map(|snack| snack.parse::<i64>())
                .try_fold(0, |acc, x| -> Result<i64, ParseIntError> { Ok(acc + x?) })
        })
        .try_fold(0, |acc, x| -> Result<i64, ParseIntError> {
            Ok(max(acc, x?))
        })
}

fn part2<T: Iterator<Item = String>>(input: T) -> Result<i64, ParseIntError> {
    input
        .group_lines()
        .iter()
        .map(|elf| {
            elf.iter()
                .map(|snack| snack.parse::<i64>())
                .try_fold(0, |acc, x| -> Result<i64, ParseIntError> { Ok(acc + x?) })
        })
        .try_fold(&mut vec![0i64; 3], |acc, x| -> Result<&mut Vec<i64>, ParseIntError> {
            let new_val = x?;
            let min_index = acc.iter().enumerate().min_by_key(|v| v.1).unwrap().0;
            if acc[min_index] < new_val {
                acc[min_index] = new_val;
            }
            Ok(acc)
        }).map(|x| x.iter().sum())
}

#[cfg(test)]
mod tests {
    use crate::part1;
    #[test]
    fn day1_test() {
        let mut input: Vec<String> = vec![
            "1000".into(),
            "2000".into(),
            "3000".into(),
            "".into(),
            "4000".into(),
            "".into(),
            "5000".into(),
            "6000".into(),
            "".into(),
            "7000".into(),
            "8000".into(),
            "9000".into(),
            "".into(),
            "10000".into(),
        ];
        assert_eq!(part1(input.into_iter()).unwrap(), 24000i64);
    }
}

fn main() {
    print!("{:?}\n", part1(read_lines("/home/nir/Downloads/day1.txt")));
    print!("{:?}\n", part2(read_lines("/home/nir/Downloads/day1.txt")));
}
