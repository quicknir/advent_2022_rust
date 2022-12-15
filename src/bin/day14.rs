use std::cmp::{max, min};

use advent_2022::{read_aoc_lines, Coord, Grid, InputIterator, OptionUtils};
use anyhow::Result;

fn parse_line(s: &str) -> Result<Vec<Coord>> {
    s.split(" -> ")
        .map(|p| {
            let ps = p.split_once(',').ok_or_err()?;
            Ok(Coord {
                i: ps.1.parse()?,
                j: ps.0.parse()?,
            })
        })
        .collect()
}

fn parse<I: InputIterator>(input: I) -> Result<(i64, i64, i64, Vec<Vec<Coord>>)> {
    let mut max_depth = 0;
    let mut min_x = 500;
    let mut max_x = 500;

    let lines = input
        .map(|line| {
            let ps = parse_line(line.as_ref())?;
            max_depth = max(max_depth, ps.iter().map(|x| x.i).max().ok_or_err()?);
            min_x = min(min_x, ps.iter().map(|x| x.j).min().ok_or_err()?);
            max_x = max(max_x, ps.iter().map(|x| x.j).max().ok_or_err()?);
            Ok(ps)
        })
        .collect::<Result<Vec<_>>>()?;
    Ok((max_depth, min_x, max_x, lines))
} 

fn normalize(i: i64) -> i64 {
    if i == 0 {
        0
    } else {
        i.signum()
    }
}

fn populate_grid(lines: Vec<Vec<Coord>>, grid: &mut Grid<bool>, offset: i64) { 
    for line in lines {
        for i in 0..(line.len() - 1) {
            let start = line[i];
            let vector = line[i + 1] - line[i];
            let norm = Coord {
                i: normalize(vector.i),
                j: normalize(vector.j),
            };
            let magnitude = max(vector.i.abs(), vector.j.abs());
            for m in 0..=magnitude {
                let p = start
                    + Coord {
                        i: norm.i * m,
                        j: norm.j * m - offset,
                    };
                grid[p] = true;
            }
        }
    }
}

fn run_sim(grid: &mut Grid<bool>, start: Coord) -> i64 {
    let moves = [
        Coord { i: 1, j: 0 },
        Coord { i: 1, j: -1 },
        Coord { i: 1, j: 1 },
    ];
    let mut num_grains = 0;
    loop {
        let mut sand_coord = start;
        'grain_movement: loop {
            for m in moves {
                let new_coord = sand_coord + m;

                match grid.get(new_coord) {
                    Some(occupied) => {
                        if !occupied {
                            sand_coord = new_coord;
                            continue 'grain_movement;
                        }
                    }
                    None => return num_grains,
                }
            }
            num_grains += 1;
            grid[sand_coord] = true;
            if sand_coord == start {
                return num_grains
            }
            break 'grain_movement;
        }
    }
}

fn part1<I: InputIterator>(input: I) -> Result<i64> {
    let (max_depth, min_x, max_x, lines) = parse(input)?;

    let mut grid = Grid::new(max_depth + 1, max_x - min_x + 1, false);

    populate_grid(lines, &mut grid, min_x);

    let num_grains = run_sim(&mut grid, Coord{i: 0, j:500 - min_x});

    Ok(num_grains)
}

fn part2<I: InputIterator>(input: I) -> Result<i64> {
    let (max_depth, min_x, max_x, lines) = parse(input)?;
    let floor_depth = max_depth + 2;

    let needed_min = min(min_x, 500 - max_depth - 2);
    let needed_max = max(max_x, 500 + max_depth + 2);

    let mut grid = Grid::new(floor_depth + 1, needed_max - needed_min + 1, false);

    populate_grid(lines, &mut grid, needed_min);

    for x in 0..grid.width() {
        grid[(floor_depth, x)] = true;
    }

    let num_grains = run_sim(&mut grid, Coord{i: 0, j:500 - needed_min});

    Ok(num_grains)
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;
    #[test]
    fn day14_test() {
        let input = vec![
            "498,4 -> 498,6 -> 496,6",
            "503,4 -> 502,4 -> 502,9 -> 494,9",
        ];
        assert_eq!(part1(input.iter()).unwrap(), 24);
        assert_eq!(part2(input.iter()).unwrap(), 93);
    }
}

fn main() {
    print!("{:?}\n", part1(read_aoc_lines!()));
    print!("{:?}\n", part2(read_aoc_lines!()));
}
