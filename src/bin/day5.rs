use advent_2022::{read_aoc_lines, InputIterator, OptionUtils, check, InputError};
use anyhow::Result;
use std::mem::take;

fn parse_crates<S: AsRef<str>>(v: &Vec<S>) -> Result<Vec<Vec<char>>> {
    let len = v.last().ok_or_err()?.as_ref().len() + 1;
    check(len % 4 == 0, || InputError::new("unexpected line length"))?;
    let num_stacks = len / 4;
    let mut stacks = vec![Vec::<char>::new(); num_stacks];

    for r in v[0..(v.len() - 1)].iter().rev() {
        let row = r.as_ref();
        for i in 0..num_stacks {
            let c = row.chars().nth(1 + 4*i).ok_or_err()?;
            if c != ' ' {
                stacks[i].push(c);
            }
        }
    }

    Ok(stacks)
}

struct Move {
    num: usize,
    from: usize,
    to: usize,
}

fn parse_move(s: &str) -> Result<Move> {
    let mut split = s.split(' ');
    split.next();
    let num = split.next().ok_or_err()?.parse()?;
    split.next();
    let from = split.next().ok_or_err()?.parse()?;
    split.next();
    let to = split.next().ok_or_err()?.parse()?;
    Ok(Move{num, from, to})
}

fn apply_move_1(v: &mut Vec<Vec<char>>, m: Move) -> Result<()> {
    for _ in 0..m.num {
        let e = v[m.from - 1].pop().ok_or_err()?;
        v[m.to - 1].push(e);
    }

    Ok(())
}

fn get_pair_mut<T>(slice: &mut [T], a: usize, b: usize) -> [&mut T; 2] {
    assert_ne!(a, b, "indices must not be equal");
    [a, b].map(|i| unsafe { &mut *(&mut slice[i] as *mut T) })
}

fn apply_move_2(v: &mut Vec<Vec<char>>, m: Move) -> Result<()> {
    let [from_vec, to_vec] = get_pair_mut(v, m.from-1, m.to-1);
    to_vec.extend(from_vec.drain((from_vec.len() - m.num)..));
    Ok(())
}

fn do_it<I: InputIterator, F: Fn(&mut Vec<Vec<char>>, Move) -> Result<()>>(input: I, f: F) -> Result<String> {
    let mut chunks = input.blank_chunks(|x| take(x));
    let mut crates = parse_crates(&chunks.next().ok_or_err()?)?;
    for move_str in chunks.next().ok_or_err()? {
        f(&mut crates, parse_move(move_str.as_ref())?)?;
    }

    check(chunks.next().is_none(), || InputError::new("unexpected extra chunk"))?;

    Ok(crates.iter().map(|stack| stack.last().unwrap_or(&' ')).collect::<String>())
}

fn part1<I: InputIterator>(input: I) -> Result<String> {
    do_it(input, &apply_move_1)
}
fn part2<I: InputIterator>(input: I) -> Result<String> {
    do_it(input, &apply_move_2)
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;
    #[test]
    fn day5_test() {
        let input = vec![
            "    [D]    ",
            "[N] [C]    ",
            "[Z] [M] [P]",
            " 1   2   3 ",
            "",
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ];
        assert_eq!(part1(input.iter()).unwrap(), "CMZ");
        assert_eq!(part2(input.iter()).unwrap(), "MCD");
    }
}

fn main() {
    print!("{:?}\n", part1(read_aoc_lines!()));
    print!("{:?}\n", part2(read_aoc_lines!()));
}
