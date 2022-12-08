use advent_2022::{read_aoc_lines, Grid, InputIterator, OptionEmptyError, OptionUtils};
use anyhow::Result;

fn parse_trees<I: InputIterator>(mut input: I) -> Result<Grid<i8>> {
    let mut height = 0;
    let mut width = 0;
    let mut v = vec![];
    input.try_for_each(|l| {
        let line = l.as_ref();
        width = line.len() as i64;
        height += 1;
        let r = line
            .chars()
            .map(|c| c.to_digit(10).map(|i| i as i8).ok_or_err())
            .try_for_each(|x| Ok(v.push(x?)));
        Ok::<(), OptionEmptyError>(r?)
    })?;

    Ok(Grid::from_data(height, width, v))
}

fn find_seen<I: Iterator<Item = (i64, i64)>>(trees: &Grid<i8>, seen: &mut Grid<bool>, i: I) {
    let mut max = -1;
    i.for_each(|p| {
        let tree_height = trees[p];
        if tree_height > max {
            seen[p] = true;
            max = tree_height;
        }
    });
}

fn part1<I: InputIterator>(input: I) -> Result<i64> {
    let trees = parse_trees(input)?;
    let (height, width) = (trees.height(), trees.width());
    let mut seen = Grid::new(height, width, false);

    for i in 0..height {
        find_seen(&trees, &mut seen, (0..width).map(|j| (i, j)));
        find_seen(&trees, &mut seen, (0..width).rev().map(|j| (i, j)));
    }
    for j in 0..width {
        find_seen(&trees, &mut seen, (0..height).map(|i| (i, j)));
        find_seen(&trees, &mut seen, (0..height).rev().map(|i| (i, j)));
    }

    Ok(seen.iter().filter(|x| **x).count() as i64)
}

fn find_scenic(trees: &Grid<i8>, start: (i64, i64), dir: (i64, i64)) -> i64 {
    let mut cur = (start.0 + dir.0, start.1 + dir.1);
    let mut scenic = 0;
    let start_height = trees[start];
    while let Some(tree_height) = trees.get(cur.0, cur.1) {
        scenic += 1;
        if *tree_height >= start_height {
            break;
        }
        cur = (cur.0 + dir.0, cur.1 + dir.1);
    }
    scenic
}

fn part2<I: InputIterator>(input: I) -> Result<i64> {
    let trees = parse_trees(input)?;

    let max = (0..trees.height())
        .flat_map(|i| (0..trees.width()).map(move |j| (i, j)))
        .map(|p| {
            find_scenic(&trees, p, (1, 0))
                * find_scenic(&trees, p, (-1, 0))
                * find_scenic(&trees, p, (0, 1))
                * find_scenic(&trees, p, (0, -1))
        })
        .max();
    Ok(max.ok_or_err()?)
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;
    #[test]
    fn day8_test() {
        let input = vec!["30373", "25512", "65332", "33549", "35390"];
        assert_eq!(part1(input.iter()).unwrap(), 21);
        assert_eq!(part2(input.iter()).unwrap(), 8);
    }
}

fn main() {
    print!("{:?}\n", part1(read_aoc_lines!()));
    print!("{:?}\n", part2(read_aoc_lines!()));
}
