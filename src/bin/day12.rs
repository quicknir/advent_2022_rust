use std::mem::swap;

use advent_2022::{check, read_aoc_lines, Coord, Grid, InputError, InputIterator, OptionUtils};
use anyhow::Result;

fn parse_file<I: InputIterator>(input: I) -> Result<(Grid<char>, Coord)> {
    let mut heights = vec![];
    let mut width = 0;
    let mut height = 0;

    for line in input {
        width = line.as_ref().len() as i64;
        heights.extend(line.as_ref().chars());
        height += 1;
    }

    check(heights.len() as i64 == height * width, || {
        InputError::new("Error, height * width doesn't match data quantity!")
    })?;

    let end_index = heights
        .iter()
        .enumerate()
        .find(|x| *x.1 == 'E')
        .ok_or_err()?
        .0;

    heights[end_index] = 'z';

    let end = Coord {
        i: end_index as i64 / width,
        j: end_index as i64 % width,
    };

    Ok((Grid::from_data(height, width, heights), end))
}

fn do_it(heights: &Grid<char>, end: Coord) -> Result<i64> {
    let mut steps = Grid::new(heights.height(), heights.width(), i64::MAX);
    steps[end] = 0;
    let mut cur_points = vec![end];
    let mut next_points = Vec::<Coord>::new();

    let step_dirs = [
        Coord { i: 1, j: 0 },
        Coord { i: -1, j: 0 },
        Coord { i: 0, j: 1 },
        Coord { i: 0, j: -1 },
    ];

    let mut cur_distance = 0;

    while !cur_points.is_empty() {
        cur_distance += 1;
        for p in &cur_points {
            let cur_height = heights[*p] as i32;
            for dir in &step_dirs {
                let cur_coord = *p + *dir;
                if !heights.contains_coord(cur_coord) {
                    continue;
                }
                let next_height = heights[cur_coord];
                if next_height == 'S' && cur_height - 'a' as i32 <= 1 {
                    return Ok(cur_distance);
                }
                if cur_height - next_height as i32 > 1 {
                    continue;
                }
                let cur_step = &mut steps[(cur_coord.i, cur_coord.j)];
                if *cur_step <= cur_distance {
                    continue;
                }
                *cur_step = cur_distance;
                next_points.push(cur_coord);
            }
        }
        cur_points.clear();
        swap(&mut cur_points, &mut next_points)
    }
    Err(InputError::new("uh oh").into())
}

fn part1<I: InputIterator>(input: I) -> Result<i64> {
    let (heights, end) = parse_file(input)?;
    do_it(&heights, end)
}

fn part2<I: InputIterator>(input: I) -> Result<i64> {
    let (mut heights, end) = parse_file(input)?;
    for h in heights.iter_mut() {
        if *h == 'a' {
            *h = 'S';
        }
    }
    do_it(&heights, end)
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;
    #[test]
    fn dayn_test() {
        let input = vec!["Sabqponm", "abcryxxl", "accszExk", "acctuvwj", "abdefghi"];
        assert_eq!(part1(input.iter()).unwrap(), 31);
        assert_eq!(part2(input.iter()).unwrap(), 29);
    }
}

fn main() {
    print!("{:?}\n", part1(read_aoc_lines!()));
    print!("{:?}\n", part2(read_aoc_lines!()));
}
