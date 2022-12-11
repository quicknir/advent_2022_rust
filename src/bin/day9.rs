use std::{cmp::max, collections::HashSet};

use advent_2022::{read_aoc_lines, Coord, InputError, InputIterator, OptionUtils};
use anyhow::Result;

fn parse_line(s: &str) -> Result<(Coord, i64)> {
    let mut split = s.split(" ");
    let c = match split.next().ok_or_err()? {
        "R" => Coord { i: 0, j: 1 },
        "L" => Coord { i: 0, j: -1 },
        "U" => Coord { i: 1, j: 0 },
        "D" => Coord { i: -1, j: 0 },
        _ => return Err(InputError::new("Unrecognized direction").into()),
    };

    Ok((c, split.next().ok_or_err()?.parse()?))
}

fn do_it<const N: usize>(input: impl InputIterator) -> Result<usize> {
    let mut rope = [Coord::default(); N];
    let mut visited = HashSet::from([*rope.last().unwrap()]);
    let mut max_distance = 0;

    for s in input {
        let (dir, num) = parse_line(s.as_ref())?;

        for _ in 0..num {
            rope[0] = rope[0] + dir;
            max_distance = max(max_distance, rope[0].i.abs() + rope[0].j.abs());
            for rope_ind in 1..rope.len() {
                let delta = rope[rope_ind - 1] - rope[rope_ind];
                if max(delta.i.abs(), delta.j.abs()) <= 1 {
                    continue;
                }
                rope[rope_ind] = rope[rope_ind]
                    + Coord {
                        i: delta.i.signum(),
                        j: delta.j.signum(),
                    };
            }
            visited.insert(*rope.last().unwrap());
        }
    }
    print!("{}\n", max_distance);

    Ok(visited.len())
}

fn part1<I: InputIterator>(input: I) -> Result<usize> {
    Ok(do_it::<2>(input)?)
}
fn part2<I: InputIterator>(input: I) -> Result<usize> {
    Ok(do_it::<10>(input)?)
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;
    #[test]
    fn dayn_test() {
        let input = vec!["R 4", "U 4", "L 3", "D 1", "R 4", "D 1", "L 5", "R 2"];
        assert_eq!(part1(input.iter()).unwrap(), 13);
        assert_eq!(part2(input.iter()).unwrap(), 1);

        let input2 = vec!["R 5", "U 8", "L 8", "D 3", "R 17", "D 10", "L 25", "U 20"];
        assert_eq!(part2(input2.iter()).unwrap(), 36);

    }
}

fn main() {
    print!("{:?}\n", part1(read_aoc_lines!()));
    print!("{:?}\n", part2(read_aoc_lines!()));
}
