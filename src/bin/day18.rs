use std::{
    cmp::{max, min},
    collections::{HashSet, VecDeque},
    ops::{Add, Range, RangeInclusive, Sub},
};

use advent_2022::{read_aoc_lines, Coord, InputIterator};
use anyhow::Result;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct MultiCoord<const N: usize> {
    pub coords: [i64; N],
}

impl<const N: usize> Default for MultiCoord<N> {
    fn default() -> Self {
        Self { coords: [0i64; N] }
    }
}

impl<const N: usize> Add<MultiCoord<N>> for MultiCoord<N> {
    type Output = MultiCoord<N>;

    fn add(self, rhs: MultiCoord<N>) -> Self::Output {
        let mut r = MultiCoord {
            coords: self.coords,
        };
        for i in 0..N {
            r.coords[i] += rhs.coords[i];
        }
        r
    }
}

fn parse_line(s: &str) -> Result<MultiCoord<3>> {
    let mut r = MultiCoord::<3>::default();
    s.split(',')
        .enumerate()
        .try_for_each(|(i, c)| -> Result<()> {
            r.coords[i] = c.parse()?;
            Ok(())
        })?;

    Ok(r)
}

fn neighbors() -> [MultiCoord<3>; 6] {
    [
        MultiCoord { coords: [1, 0, 0] },
        MultiCoord { coords: [-1, 0, 0] },
        MultiCoord { coords: [0, 1, 0] },
        MultiCoord { coords: [0, -1, 0] },
        MultiCoord { coords: [0, 0, 1] },
        MultiCoord { coords: [0, 0, -1] },
    ]
}

fn part1<I: InputIterator>(input: I) -> Result<usize> {
    let set = input
        .map(|x| parse_line(x.as_ref()))
        .collect::<Result<HashSet<_>>>()?;

    let neighbors = neighbors();

    Ok(set
        .iter()
        .map(|c| {
            neighbors
                .iter()
                .filter(|n| !set.contains(&(**n + *c)))
                .count()
        })
        .sum())
}

fn contains_point(bounds: &[RangeInclusive<i64>; 3], p: &MultiCoord<3>) -> bool {
    (0..3).all(|i| bounds[i].contains(&p.coords[i]))
}

fn steam_fill(
    steam: &mut HashSet<MultiCoord<3>>,
    lava: &HashSet<MultiCoord<3>>,
    bounds: &[RangeInclusive<i64>; 3],
    start: MultiCoord<3>,
) {
    let mut points = vec![start];
    while let Some(p) = points.pop() {
        for n in neighbors() {
            let current = p + n;
            if lava.contains(&current) || steam.contains(&current) {
                continue;
            }
            if !contains_point(&bounds, &current) {
                continue;
            }
            steam.insert(current);
            points.push(current);
        }
    }
}

fn part2<I: InputIterator>(input: I) -> Result<usize> {
    let lava = input
        .map(|x| parse_line(x.as_ref()))
        .collect::<Result<HashSet<_>>>()?;

    let neighbors = neighbors();

    let max = lava.iter().fold([0i64; 3], |mut acc, x| {
        for i in 0..3 {
            acc[i] = max(acc[i], x.coords[i]);
        }
        acc
    });
    let min = lava.iter().fold([0i64; 3], |mut acc, x| {
        for i in 0..3 {
            acc[i] = min(acc[i], x.coords[i]);
        }
        acc
    });

    let bounds = [
        ((min[0] - 1)..=(max[0] + 1)),
        ((min[1] - 1)..=(max[1] + 1)),
        ((min[2] - 1)..=(max[2] + 1)),
    ];

    let start = MultiCoord {
        coords: [0, 0, max[2] + 1],
    };
    let mut steam = HashSet::from([start]);
    steam_fill(&mut steam, &lava, &bounds, start);

    Ok(lava
        .iter()
        .map(|c| {
            neighbors
                .iter()
                .filter(|n| steam.contains(&(**n + *c)))
                .count()
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use crate::contains_point;
    use crate::part1;
    use crate::part2;
    use crate::MultiCoord;
    #[test]
    fn contains() {
        let bounds = [-1..=4, -1..=4, -1..=7];
        let p = MultiCoord { coords: [1, 0, -4] };
        assert_eq!(contains_point(&bounds, &p), false);
    }

    #[test]
    fn day18_test() {
        let input = vec![
            "2,2,2", "1,2,2", "3,2,2", "2,1,2", "2,3,2", "2,2,1", "2,2,3", "2,2,4", "2,2,6",
            "1,2,5", "3,2,5", "2,1,5", "2,3,5",
        ];
        assert_eq!(part1(input.iter()).unwrap(), 64);
        assert_eq!(part2(input.iter()).unwrap(), 58);
    }
}

fn main() {
    print!("{:?}\n", part1(read_aoc_lines!()));
    print!("{:?}\n", part2(read_aoc_lines!()));
}
