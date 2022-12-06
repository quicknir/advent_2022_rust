use advent_2022::{check, read_aoc_lines, InputError, InputIterator, OptionUtils};
use anyhow::Result;

struct Assignment {
    start: i64,
    end: i64,
}

impl Assignment {
    fn new(s: &str) -> Result<Assignment> {
        let mut start_end = s.split('-');
        let start = start_end.next().ok_or_err()?.parse()?;
        let end = start_end.next().ok_or_err()?.parse()?;
        Ok(Assignment { start, end })
    }

    fn subset_of(&self, other: &Assignment) -> bool {
        self.start >= other.start && self.end <= other.end
    }

    fn intersects_with(&self, other: &Assignment) -> bool {
        let r = self.start..=self.end;
        r.contains(&other.start) || r.contains(&other.end) || self.subset_of(other)
    }
}

fn parse_line(line: &str) -> Result<(Assignment, Assignment)> {
    let mut assignments = line.split(',');
    let first_assignment = assignments.next().ok_or_err()?;
    let second_assignment = assignments.next().ok_or_err()?;
    check(assignments.next().is_none(), || {
        InputError::new("More than one comma on line!")
    })?;
    Ok((
        Assignment::new(first_assignment)?,
        Assignment::new(second_assignment)?,
    ))
}

fn count_assignments_if<I: InputIterator, F: Fn(&Assignment, &Assignment) -> bool>(input: I, f: F) -> Result<i64> {
    input
        .map(|l| parse_line(l.as_ref()))
        .filter(|a| {
            a.as_ref()
                .map_or(true, |x| f(&x.0, &x.1))
        })
        .try_fold(0, |acc, x| {
            x?;
            Ok(acc + 1)
        })
}

fn part1<I: InputIterator>(input: I) -> Result<i64> {
    count_assignments_if(input, |x, y| x.subset_of(&y) || y.subset_of(&x))
}
fn part2<I: InputIterator>(input: I) -> Result<i64> {
    count_assignments_if(input, |x, y| x.intersects_with(&y))
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;
    use crate::Assignment;
    #[test]
    fn intersect_test() {
        assert!(Assignment { start: 2, end: 8 }.intersects_with(&Assignment { start: 3, end: 7 }));
    }
    #[test]
    fn day4_test() {
        let input = vec![
            "2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8",
        ];
        assert_eq!(part1(input.iter()).unwrap(), 2);
        assert_eq!(part2(input.iter()).unwrap(), 4);
    }
}

fn main() {
    print!("{:?}\n", part1(read_aoc_lines!()));
    print!("{:?}\n", part2(read_aoc_lines!()));
}
