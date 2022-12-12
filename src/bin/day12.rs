use std::mem::swap;

use advent_2022::{check, read_aoc_lines, Coord, Grid, InputError, InputIterator, OptionUtils};
use anyhow::Result;

fn parse_file<I: InputIterator>(input: I) -> Result<(Grid<char>, Coord, Coord)> {
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

    let mut find = |c: char, r: char| -> Result<Coord>{
        let index = heights
            .iter()
            .enumerate()
            .find(|x| *x.1 == c)
            .ok_or_err()?
            .0;
        heights[index] = r;
        Ok(Coord {
            i: index as i64 / width,
            j: index as i64 % width,
        })
    };

    let start = find('S', 'a')?;
    let end = find('E', 'z')?;

    Ok((Grid::from_data(height, width, heights), start, end))
}

fn do_it(heights: &Grid<char>, end: Coord) -> Grid<i64> {
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
            let cur_height = heights[*p];
            for dir in &step_dirs {
                let cur_coord = *p + *dir;
                if !heights.contains_coord(cur_coord) {
                    continue;
                }
                if cur_height as i32 - heights[(cur_coord.i, cur_coord.j)] as i32 > 1 {
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
    steps
}

fn part1<I: InputIterator>(input: I) -> Result<i64> {
    let (heights, start, end) = parse_file(input)?;
    let steps = do_it(&heights, end);
    Ok(steps[start])
}

fn part2<I: InputIterator>(input: I) -> Result<i64> {
    let (heights, _, end) = parse_file(input)?;
    let steps = do_it(&heights, end);
    Ok((0..heights.height())
        .flat_map(|i| (0..heights.width()).map(move |j| (i, j)))
        .filter(|p| heights[*p] == 'a')
        .map(|p| steps[p])
        .min()
        .unwrap())
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
