use std::{cmp::Ordering, iter::zip};

use advent_2022::{check, read_aoc_lines, InputError, InputIterator, OptionUtils};
use anyhow::Result;

#[derive(Default, Debug)]
struct List {
    list: Vec<PacketEntry>,
}

#[derive(Debug, Clone, Copy)]
enum PacketEntry {
    Int(i64),
    List(usize),
}

fn parse_line_helper<'a>(
    mut s: &'a str,
    store_index: usize,
    list_store: &mut Vec<List>,
) -> Result<&'a str> {
    loop {
        match s.chars().nth(0).ok_or_err()? {
            ']' => return Ok(&s[1..]),
            '[' => {
                let recurse_index = list_store.len();
                list_store.push(Default::default());
                list_store[store_index]
                    .list
                    .push(PacketEntry::List(recurse_index));
                s = parse_line_helper(&s[1..], recurse_index, list_store)?;
            }
            ',' => s = &s[1..],
            _ => {
                // parsing an integer
                let end_index = s
                    .chars()
                    .enumerate()
                    .find(|x| x.1 == ']' || x.1 == ',')
                    .ok_or_err()?
                    .0;
                list_store[store_index]
                    .list
                    .push(PacketEntry::Int(s[..end_index].parse()?));
                s = &s[end_index..];
            }
        }
    }
}

fn parse_line(line: &str) -> Result<Vec<List>> {
    let mut list_store = vec![];
    list_store.push(Default::default());
    parse_line_helper(&line[1..], 0, &mut list_store)?;
    Ok(list_store)
}

fn compare_helper(
    left: &List,
    right: &List,
    left_store: &Vec<List>,
    right_store: &Vec<List>,
) -> Ordering {

    for (left_el, right_el) in zip(left.list.iter(), right.list.iter()) {
        let o = match (left_el, right_el) {
            (PacketEntry::Int(left_int), PacketEntry::Int(right_int)) => left_int.cmp(&right_int),
            (PacketEntry::List(l), PacketEntry::List(r)) => {
                compare_helper(&left_store[*l], &right_store[*r], left_store, right_store)
            }
            (PacketEntry::Int(left_int), PacketEntry::List(r)) => {
                let v = List {
                    list: vec![PacketEntry::Int(*left_int)],
                };
                compare_helper(&v, &right_store[*r], left_store, right_store)
            }
            (PacketEntry::List(l), PacketEntry::Int(right_int)) => {
                let v = List {
                    list: vec![PacketEntry::Int(*right_int)],
                };
                compare_helper(&left_store[*l], &v, left_store, right_store)
            }
        };
        if !o.is_eq() {
            return o;
        }
    }
    left.list.len().cmp(&right.list.len())
}

fn compare(lhs: &Vec<List>, rhs: &Vec<List>) -> Ordering {
    compare_helper(&lhs[0], &rhs[0], lhs, rhs)
}

fn parse_and_compare<S: AsRef<str>>(v: &Vec<S>) -> Result<bool> {
    check(v.len() == 2, || {
        InputError::new("Input chunk vector wrong length")
    })?;
    Ok(compare(&parse_line(v[0].as_ref())?, &parse_line(v[1].as_ref())?) == Ordering::Less)
}

fn part1<I: InputIterator>(input: I) -> Result<usize> {
    input
        .blank_chunks(|v| Ok(parse_and_compare(v)?))
        .enumerate()
        .try_fold(0, |acc, x: (usize, Result<bool>)| {
            Ok(if x.1? { acc + x.0 + 1 } else { acc })
        })
}
fn part2<I: InputIterator>(input: I) -> Result<i64> {
    let two = parse_line("[[2]]")?;
    let six = parse_line("[[6]]")?;

    let num_smaller = input
        .filter(|s| s.as_ref() != "")
        .map(|s| parse_line(s.as_ref()))
        .try_fold([0; 2], |mut acc, x| -> Result<[i64; 2]>
        {
            let r = x?;
            if compare(&r, &two) == Ordering::Less {
                acc[0] += 1;
            } 
            if compare(&r, &six) == Ordering::Less {
                acc[1] += 1;
            } 
            Ok(acc)
    })?;

    let smaller_index = num_smaller.iter().min().unwrap();
    let larger_index = num_smaller.iter().max().unwrap();
        
    Ok((smaller_index+1) * (larger_index+2))
}

#[cfg(test)]
mod tests {
    use crate::parse_and_compare;
    #[test]
    fn test_parse_compare() {
        let input1 = vec!["[1,1,3,1,1]", "[1,1,5,1,1]"];
        assert_eq!(parse_and_compare(&input1).unwrap(), true);
        let input2 = vec!["[[1],[2,3,4]]", "[[1],4]"];
        assert_eq!(parse_and_compare(&input2).unwrap(), true);
        let input3 = vec!["[9]", "[[8,7,6]]"];
        assert_eq!(parse_and_compare(&input3).unwrap(), false);
        let input4 = vec!["[[4,4],4,4]", "[[4,4],4,4,4]"];
        assert_eq!(parse_and_compare(&input4).unwrap(), true);
        let input5 = vec!["[7,7,7,7]", "[7,7,7]"];
        assert_eq!(parse_and_compare(&input5).unwrap(), false);
        let input6 = vec!["[]", "[3]"];
        assert_eq!(parse_and_compare(&input6).unwrap(), true);
        let input7 = vec!["[[[]]]", "[[]]"];
        assert_eq!(parse_and_compare(&input7).unwrap(), false);
        let input8 = vec!["[1,[2,[3,[4,[5,6,7]]]],8,9]", "[1,[2,[3,[4,[5,6,0]]]],8,9]"];
        assert_eq!(parse_and_compare(&input8).unwrap(), false);
    }
}

fn main() {
    print!("{:?}\n", part1(read_aoc_lines!()));
    print!("{:?}\n", part2(read_aoc_lines!()));
}
