use std::mem::take;

use advent_2022::{read_aoc_lines, InputIterator, OptionUtils};
use anyhow::Result;

fn add(x: i64, y: i64) -> i64 {
    x + y
}
fn mult(x: i64, y: i64) -> i64 {
    x * y
}
fn square(x: i64, _y: i64) -> i64 {
    x * x
}

struct Monkey {
    items: Vec<i64>,
    operator: fn(i64, i64) -> i64,
    operand: i64,
    divisible_test: i64,
    true_monkey: usize,
    false_monkey: usize,
}

fn parse_monkey<S: AsRef<str>>(lines: &[S]) -> Result<Monkey> {
    let items = lines[1]
        .as_ref()
        .split_once(": ")
        .ok_or_err()?
        .1
        .split(", ")
        .map(|s| s.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;

    let operation = lines[2].as_ref().split_once("old ").ok_or_err()?.1;
    let (operator, operand) = if operation == "* old" {
        (square as fn(i64, i64) -> i64, 0i64)
    } else {
        let (operator_str, operand_str) = operation.split_once(' ').ok_or_err()?;
        (
            if operator_str == "*" { mult } else { add },
            operand_str.parse::<i64>()?,
        )
    };

    let divisible_test: i64 = lines[3].as_ref().split_once("by ").ok_or_err()?.1.parse()?;

    let true_monkey: usize = lines[4]
        .as_ref()
        .split_once("monkey ")
        .ok_or_err()?
        .1
        .parse()?;

    let false_monkey: usize = lines[5]
        .as_ref()
        .split_once("monkey ")
        .ok_or_err()?
        .1
        .parse()?;

    Ok(Monkey {
        items,
        operator,
        operand,
        divisible_test,
        true_monkey,
        false_monkey,
    })
}

fn parse_file<I: InputIterator>(input: I) -> Result<Vec<Monkey>> {
    Ok(input
        .blank_chunks(|b| parse_monkey(b))
        .collect::<Result<Vec<_>, _>>()?)
}

fn do_it<F: Fn(i64) -> i64>(monkeys: &mut [Monkey], loop_iterations: i64, worry_reducer: F) -> Result<usize> {
    
    let mut activity_levels = vec![0usize; monkeys.len()];
    
    for _ in 0..loop_iterations {
        for mon_index in 0..monkeys.len() {
            let mut items = take(&mut monkeys[mon_index].items);
            activity_levels[mon_index] += items.len();

            for item in &items {
                let cur_monkey = &monkeys[mon_index];
                let new_worry_level = worry_reducer((cur_monkey.operator)(*item, cur_monkey.operand));
                let new_monkey_ind = if new_worry_level % cur_monkey.divisible_test == 0 {
                    cur_monkey.true_monkey
                }
                else {
                    cur_monkey.false_monkey
                };
                monkeys[new_monkey_ind].items.push(new_worry_level);
            }
            items.clear();
            monkeys[mon_index].items = items;
        }
    }
    let mut max_active = [0usize; 2];
    for x in activity_levels {
        let min_index = max_active.iter().enumerate().min_by_key(|v| v.1).unwrap().0;
        if max_active[min_index] < x {
            max_active[min_index] = x;
        }

    }

    Ok(max_active[0] * max_active[1])

}

fn part1<I: InputIterator>(input: I) -> Result<usize> {
    let mut monkeys = parse_file(input)?;
    do_it(&mut monkeys, 20, |x| x / 3)
}
fn part2<I: InputIterator>(input: I) -> Result<usize> {
    let mut monkeys = parse_file(input)?;

    // it happens to be the case for test and real input that all divisibility checks are distinct primes;
    // which saves us having to write an lcm algorithm
    let lcm = monkeys.iter().fold(1, |acc, m| acc * m.divisible_test);

    do_it(&mut monkeys, 10000, |x| x % lcm)
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;
    #[test]
    fn dayn_test() {
        let input = vec![
"Monkey 0:",
"  Starting items: 79, 98",
"  Operation: new = old * 19",
"  Test: divisible by 23",
"    If true: throw to monkey 2",
"    If false: throw to monkey 3",
"",
"Monkey 1:",
"  Starting items: 54, 65, 75, 74",
"  Operation: new = old + 6",
"  Test: divisible by 19",
"    If true: throw to monkey 2",
"    If false: throw to monkey 0",
"",
"Monkey 2:",
"  Starting items: 79, 60, 97",
"  Operation: new = old * old",
"  Test: divisible by 13",
"    If true: throw to monkey 1",
"    If false: throw to monkey 3",
"",
"Monkey 3:",
"  Starting items: 74",
"  Operation: new = old + 3",
"  Test: divisible by 17",
"    If true: throw to monkey 0",
"    If false: throw to monkey 1",
        ];
        assert_eq!(part1(input.iter()).unwrap(), 10605);
        assert_eq!(part2(input.iter()).unwrap(), 2713310158);
    }
}

fn main() {
    print!("{:?}\n", part1(read_aoc_lines!()));
    print!("{:?}\n", part2(read_aoc_lines!()));
}
