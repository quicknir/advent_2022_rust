use advent_2022::{read_aoc_lines, InputError, InputIterator};
use anyhow::Result;

#[derive(PartialEq, Clone, Copy, Debug)]
enum Move {
    Rock = 0,
    Paper,
    Scissors,
}

fn int_to_move(i: i8) -> Move {
    match i {
        0 => Move::Rock,
        1 => Move::Paper,
        2 => Move::Scissors,
        _ => unreachable!(),
    }
}

fn winning_move(opp: Move) -> Move {
    int_to_move(((opp as i8) + 1).rem_euclid(3))
}

fn losing_move(opp: Move) -> Move {
    int_to_move(((opp as i8) - 1).rem_euclid(3))
}

fn first_col_move(s: &str) -> Result<Move, InputError> {
    let r = match s {
        "A" => Move::Rock,
        "B" => Move::Paper,
        "C" => Move::Scissors,
        _ => {
            return Err(InputError::new(format!(
                "Invalid first column string {}",
                s
            )))
        }
    };
    Ok(r)
}

fn shape_score(my: Move) -> i64 {
    (my as i64) + 1
}

fn part1<I: InputIterator>(input: I) -> Result<i64> {
    input
        .map(|r| {
            let opp = first_col_move(&r.as_ref()[0..1])?;
            let my = match &r.as_ref()[2..3] {
                "X" => Move::Rock,
                "Y" => Move::Paper,
                "Z" => Move::Scissors,
                _ => return Err(InputError::new(format!("Could not parse second column"))),
            };
            let win_score = if opp == my {
                Ok(3)
            } else if my == winning_move(opp) {
                Ok(6)
            } else {
                Ok(0)
            };
            Ok(shape_score(my) + win_score?)
        })
        .try_fold(0, |acc, x| Ok(acc + x?))
}

fn part2<I: InputIterator>(input: I) -> Result<i64> {
    input
        .map(|r| {
            let opp = first_col_move(&r.as_ref()[0..1])?;
            Ok(match &r.as_ref()[2..3] {
                "X" => shape_score(losing_move(opp)),
                "Y" => shape_score(opp) + 3,
                "Z" => shape_score(winning_move(opp)) + 6,
                _ => return Err(InputError::new(format!("Could not parse second column"))),
            })
        })
        .try_fold(0, |acc, x| Ok(acc + x?))
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;
    #[test]
    fn day2_test() {
        let i = vec!["A Y", "B X", "C Z"];
        assert_eq!(part1(i.iter()).unwrap(), 15);
        assert_eq!(part2(i.iter()).unwrap(), 12);
    }
}

fn main() {
    print!("{:?}\n", part1(read_aoc_lines!()));
    print!("{:?}\n", part2(read_aoc_lines!()));
}
