use advent_2022::{read_aoc_lines, InputIterator};

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

fn first_col_move(s: &str) -> Move {
    match s {
        "A" => Move::Rock,
        "B" => Move::Paper,
        "C" => Move::Scissors,
        _ => unreachable!(),
    }
}

fn shape_score(my: Move) -> i64 {
    (my as i64) + 1
}

fn part1<I: InputIterator>(input: I) -> i64 {
    input
        .map(|r| {
            let opp = first_col_move(&r.as_ref()[0..1]);
            let my = match &r.as_ref()[2..3] {
                "X" => Move::Rock,
                "Y" => Move::Paper,
                "Z" => Move::Scissors,
                _ => unreachable!(),
            };
            let win_score = if opp == my {
                3
            } else if my == winning_move(opp) {
                6
            } else {
                0
            };
            shape_score(my) + win_score
        })
        .sum()
}

fn part2<I: InputIterator>(input: I) -> i64 {
    input
        .map(|r| {
            let opp = first_col_move(&r.as_ref()[0..1]);
            match &r.as_ref()[2..3] {
                "X" => shape_score(losing_move(opp)),
                "Y" => shape_score(opp) + 3,
                "Z" => shape_score(winning_move(opp)) + 6,
                _ => unreachable!(),
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;
    #[test]
    fn day2_test() {
        let i = vec!["A Y", "B X", "C Z"];
        assert_eq!(part1(i.iter()), 15);
        assert_eq!(part2(i.iter()), 12);
    }
}

fn main() {
    print!("{}\n", part1(read_aoc_lines!()));
    print!("{}\n", part2(read_aoc_lines!()));
}
