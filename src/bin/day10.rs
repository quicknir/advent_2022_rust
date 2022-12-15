use advent_2022::{read_aoc_lines, Grid, InputIterator, OptionUtils, show_bool_grid};
use anyhow::Result;

enum Instruction {
    AddX { addend: i64 },
    Noop,
}

fn parse_line(s: &str) -> Result<Instruction> {
    let mut split = s.split(' ');
    let first = split.next().ok_or_err()?;
    if first == "noop" {
        Ok(Instruction::Noop)
    } else {
        Ok(Instruction::AddX {
            addend: split.next().ok_or_err()?.parse()?,
        })
    }
}

fn part1<I: InputIterator>(input: I) -> Result<i64> {
    let mut x: i64 = 1;
    let mut cycles = 0;
    let mut signal_strength_sum: i64 = 0;
    let mut seen20 = false;
    let mut num40 = 0;

    for i in input.map(|s| parse_line(s.as_ref())) {
        let instr = i?;

        let (cycle_add, register_add) = match instr {
            Instruction::Noop => (1, 0),
            Instruction::AddX { addend: a } => (2, a),
        };

        cycles += cycle_add;

        let signal_cycles = if !seen20 && cycles >= 20 {
            seen20 = true;
            20
        } else if (cycles - 20) / 40 > num40 {
            num40 = (cycles - 20) / 40;
            20 + 40 * num40
        } else {
            0
        };
        signal_strength_sum += signal_cycles * x;
        x += register_add;
    }

    Ok(signal_strength_sum)
}

fn draw_pixel(grid: &mut Grid<bool>, sprite: i64, cycles: i64) {
    let j = cycles % 40;
    let i = (cycles / 40) % 6;

    grid[(i, j)] = (sprite - j).abs() <= 1;
}

fn part2<I: InputIterator>(input: I) -> Result<Vec<String>> {
    let mut x: i64 = 1;
    let mut cycles = 0;
    let mut crt = Grid::new(6, 40, false);

    for i in input.map(|s| parse_line(s.as_ref())) {
        let instr = i?;

        match instr {
            Instruction::Noop => {
                draw_pixel(&mut crt, x, cycles);
                cycles += 1;
            }
            Instruction::AddX { addend: a } => {
                draw_pixel(&mut crt, x, cycles);
                draw_pixel(&mut crt, x, cycles + 1);
                cycles += 2;
                x += a;
            }
        };
    }

    Ok(show_bool_grid(&crt))
}

#[cfg(test)]
mod tests {
    use std::iter::zip;

    use crate::part1;
    use crate::part2;
    #[test]
    fn day10_test() {
        let input = vec![
            "addx 15", "addx -11", "addx 6", "addx -3", "addx 5", "addx -1", "addx -8", "addx 13",
            "addx 4", "noop", "addx -1", "addx 5", "addx -1", "addx 5", "addx -1", "addx 5",
            "addx -1", "addx 5", "addx -1", "addx -35", "addx 1", "addx 24", "addx -19", "addx 1",
            "addx 16", "addx -11", "noop", "noop", "addx 21", "addx -15", "noop", "noop",
            "addx -3", "addx 9", "addx 1", "addx -3", "addx 8", "addx 1", "addx 5", "noop", "noop",
            "noop", "noop", "noop", "addx -36", "noop", "addx 1", "addx 7", "noop", "noop", "noop",
            "addx 2", "addx 6", "noop", "noop", "noop", "noop", "noop", "addx 1", "noop", "noop",
            "addx 7", "addx 1", "noop", "addx -13", "addx 13", "addx 7", "noop", "addx 1",
            "addx -33", "noop", "noop", "noop", "addx 2", "noop", "noop", "noop", "addx 8", "noop",
            "addx -1", "addx 2", "addx 1", "noop", "addx 17", "addx -9", "addx 1", "addx 1",
            "addx -3", "addx 11", "noop", "noop", "addx 1", "noop", "addx 1", "noop", "noop",
            "addx -13", "addx -19", "addx 1", "addx 3", "addx 26", "addx -30", "addx 12",
            "addx -1", "addx 3", "addx 1", "noop", "noop", "noop", "addx -9", "addx 18", "addx 1",
            "addx 2", "noop", "noop", "addx 9", "noop", "noop", "noop", "addx -1", "addx 2",
            "addx -37", "addx 1", "addx 3", "noop", "addx 15", "addx -21", "addx 22", "addx -6",
            "addx 1", "noop", "addx 2", "addx 1", "noop", "addx -10", "noop", "noop", "addx 20",
            "addx 1", "addx 2", "addx 2", "addx -6", "addx -11", "noop", "noop", "noop",
        ];
        assert_eq!(part1(input.iter()).unwrap(), 13140);
        let part2_output = vec![
            "##..##..##..##..##..##..##..##..##..##..",
            "###...###...###...###...###...###...###.",
            "####....####....####....####....####....",
            "#####.....#####.....#####.....#####.....",
            "######......######......######......####",
            "#######.......#######.......#######.....",
        ];
        let part2_results = part2(input.iter()).unwrap();
        assert!(zip(part2_output.iter(), part2_results.iter()).all(|x| x.0 == x.1));
    }
}

fn main() {
    print!("{:?}\n", part1(read_aoc_lines!()));
    print!("{}\n", part2(read_aoc_lines!()).unwrap().join("\n"));
}
