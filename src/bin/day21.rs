use std::collections::HashMap;

use advent_2022::{read_aoc_lines, InputIterator, OptionUtils};
use anyhow::Result;

enum Operator {
    Add,
    Sub,
    Mult,
    Div,
}

enum Monkey {
    Int(i64),
    Operation {
        lhs: String,
        rhs: String,
        op: Operator,
    },
    Unknown,
}

fn evaluate(
    monkeys: &HashMap<String, Monkey>,
    cache: &mut HashMap<String, i64>,
    name: &str,
) -> Option<i64> {
    if let Some(s) = cache.get(name) {
        return Some(*s);
    }

    let monkey = monkeys.get(name).unwrap();
    if let Monkey::Unknown = monkey {
        return None;
    }

    let result = match monkeys.get(name).unwrap() {
        Monkey::Int(x) => *x,
        Monkey::Operation { lhs, rhs, op } => {
            let l = evaluate(monkeys, cache, lhs);
            let r = evaluate(monkeys, cache, rhs);
            match op {
                Operator::Add => l? + r?,
                Operator::Sub => l? - r?,
                Operator::Mult => l? * r?,
                Operator::Div => l? / r?,
            }
        }
        _ => unreachable!(),
    };
    cache.insert(name.to_string(), result);
    Some(result)
}

fn parse_line(s: &str) -> Result<(String, Monkey)> {
    let (name_slice, rest) = s.split_once(": ").ok_or_err()?;
    let name = name_slice.to_string();
    if let Ok(i) = rest.parse::<i64>() {
        return Ok((name, Monkey::Int(i)));
    }

    let mut split = rest.split(' ');
    let (left, operator, right) = (
        split.next().ok_or_err()?,
        split.next().ok_or_err()?,
        split.next().ok_or_err()?,
    );

    let op = match operator {
        "+" => Operator::Add,
        "-" => Operator::Sub,
        "*" => Operator::Mult,
        "/" => Operator::Div,
        _ => unreachable!(""),
    };

    Ok((
        name,
        Monkey::Operation {
            lhs: left.to_string(),
            rhs: right.to_string(),
            op,
        },
    ))
}

fn part1<I: InputIterator>(input: I) -> Result<i64> {
    let monkeys = input
        .map(|x| parse_line(x.as_ref()))
        .collect::<Result<HashMap<_, _>>>()?;

    let mut cache = HashMap::new();
    Ok(evaluate(&monkeys, &mut cache, "root").ok_or_err()?)
}

fn find_human_value(
    monkeys: &HashMap<String, Monkey>,
    cache: &HashMap<String, i64>,
    name: &str,
    needed_value: i64,
) -> i64 {
    if name == "humn" {
        return needed_value;
    }

    let (lhs, rhs, op) = if let Monkey::Operation { lhs, rhs, op } = monkeys.get(name).unwrap() {
        (lhs, rhs, op)
    } else {
        unreachable!("");
    };

    match (cache.get(lhs), cache.get(rhs)) {
        (Some(l), None) => match op {
            Operator::Add => find_human_value(monkeys, cache, rhs, needed_value - *l),
            Operator::Sub => find_human_value(monkeys, cache, rhs, *l - needed_value),
            Operator::Mult => find_human_value(monkeys, cache, rhs, needed_value / *l),
            Operator::Div => find_human_value(monkeys, cache, rhs, *l / needed_value),
        },
        (None, Some(r)) => match op {
            Operator::Add => find_human_value(monkeys, cache, lhs, needed_value - *r),
            Operator::Sub => find_human_value(monkeys, cache, lhs, needed_value + *r),
            Operator::Mult => find_human_value(monkeys, cache, lhs, needed_value / *r),
            Operator::Div => find_human_value(monkeys, cache, lhs, needed_value * *r),
        },
        _ => unreachable!(""),
    }
}

fn part2<I: InputIterator>(input: I) -> Result<i64> {
    let mut monkeys = input
        .map(|x| parse_line(x.as_ref()))
        .collect::<Result<HashMap<_, _>>>()?;

    monkeys
        .insert("humn".to_string(), Monkey::Unknown)
        .ok_or_err()?;
    let mut cache = HashMap::new();
    assert!(evaluate(&monkeys, &mut cache, "root").is_none());
    let root = monkeys.remove("root").unwrap();

    let (lhs, rhs) = if let Monkey::Operation { lhs, rhs, op: _ } = root {
        (lhs, rhs)
    } else {
        unreachable!("")
    };

    Ok(match (cache.get(&lhs), cache.get(&rhs)) {
        (Some(l), None) => find_human_value(&monkeys, &cache, &rhs, *l),
        (None, Some(r)) => find_human_value(&monkeys, &cache, &lhs, *r),
        _ => unreachable!(""),
    })
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;
    #[test]
    fn dayn_test() {
        let input = vec![
            "root: pppw + sjmn",
            "dbpl: 5",
            "cczh: sllz + lgvd",
            "zczc: 2",
            "ptdq: humn - dvpt",
            "dvpt: 3",
            "lfqf: 4",
            "humn: 5",
            "ljgn: 2",
            "sjmn: drzm * dbpl",
            "sllz: 4",
            "pppw: cczh / lfqf",
            "lgvd: ljgn * ptdq",
            "drzm: hmdt - zczc",
            "hmdt: 32",
        ];
        assert_eq!(part1(input.iter()).unwrap(), 152);
        assert_eq!(part2(input.iter()).unwrap(), 301);
    }
}

fn main() {
    print!("{:?}\n", part1(read_aoc_lines!()));
    print!("{:?}\n", part2(read_aoc_lines!()));
}
