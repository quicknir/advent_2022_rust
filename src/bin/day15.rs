use std::cmp::{max, min};

use advent_2022::{read_aoc_lines, Coord, InputIterator, OptionUtils};
use anyhow::Result;

struct Sensor {
    loc: Coord,
    dist: i64,
}

fn parse_coord(s: &str) -> Result<Coord> {
    let (x, y) = s.split_once(", ").ok_or_err()?;
    Ok(Coord {
        j: x.split_once('=').ok_or_err()?.1.parse()?,
        i: y.split_once('=').ok_or_err()?.1.parse()?,
    })
}

fn parse_line(s: &str) -> Result<Sensor> {
    let mut split = s.split("at ");
    split.next();
    let sensor_loc = parse_coord(split.next().ok_or_err()?.split_once(":").ok_or_err()?.0)?;
    let beacon_loc = parse_coord(split.next().ok_or_err()?)?;
    Ok(Sensor {
        loc: sensor_loc,
        dist: (sensor_loc.i - beacon_loc.i).abs() + (sensor_loc.j - beacon_loc.j).abs(),
    })
}

fn sensor_to_coverage(sensor: &Sensor, iline: i64) -> Option<(i64, i64)> {
    let remaining_distance = sensor.dist - (sensor.loc.i - iline).abs();
    if remaining_distance < 0 {
        None
    } else {
        Some((
            sensor.loc.j - remaining_distance,
            sensor.loc.j + remaining_distance,
        ))
    }
}

fn do_part1<I: InputIterator>(input: I, iline: i64) -> Result<i64> {
    let mut coverage = input
        .filter_map(|s| {
            let sensor = match parse_line(s.as_ref()) {
                Err(e) => return Some(Err(e)),
                Ok(s) => s,
            };
            sensor_to_coverage(&sensor, iline).map(|x| Ok(x))
        })
        .collect::<Result<Vec<_>>>()?;

    coverage.sort();

    let mut total_coverage = 0;
    let null_cover = (i64::MIN, i64::MIN);
    let mut current_cover = null_cover;

    for cover in coverage.iter() {
        if (cover.0 - current_cover.1) > 1 {
            // what we have is disjoint from the rest
            total_coverage += current_cover.1 - current_cover.0;
            current_cover = *cover;
        } else {
            // what we have can be combined with the next cover
            current_cover = (min(cover.0, current_cover.0), max(cover.1, current_cover.1));
        }
    }
    total_coverage += current_cover.1 - current_cover.0;

    Ok(total_coverage)
}

fn part1<I: InputIterator>(input: I) -> Result<i64> {
    do_part1(input, 2000000)
}

fn do_part2<I: InputIterator>(input: I, max_coord: i64) -> Result<i64> {
    let sensors = input
        .map(|s| parse_line(s.as_ref()))
        .collect::<Result<Vec<_>>>()?;
    for row in 0..=max_coord {
        let mut coverage: Vec<_> = sensors
            .iter()
            .filter_map(|s| sensor_to_coverage(s, row))
            .collect();
        coverage.sort();
        let mut extent = 0;
        for cover in coverage.iter() {
            if (cover.0 - extent) > 1 {
                // we found a gap
                return Ok(row + (extent + 1) * 4000000);
            } else {
                extent = max(extent, cover.1);
            }
        }
    }
    Ok(0)
}

fn part2<I: InputIterator>(input: I) -> Result<i64> {
    do_part2(input, 4000000)
}

#[cfg(test)]
mod tests {
    use crate::do_part1;
    use crate::do_part2;
    #[test]
    fn day15_test() {
        let input = vec![
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15",
            "Sensor at x=9, y=16: closest beacon is at x=10, y=16",
            "Sensor at x=13, y=2: closest beacon is at x=15, y=3",
            "Sensor at x=12, y=14: closest beacon is at x=10, y=16",
            "Sensor at x=10, y=20: closest beacon is at x=10, y=16",
            "Sensor at x=14, y=17: closest beacon is at x=10, y=16",
            "Sensor at x=8, y=7: closest beacon is at x=2, y=10",
            "Sensor at x=2, y=0: closest beacon is at x=2, y=10",
            "Sensor at x=0, y=11: closest beacon is at x=2, y=10",
            "Sensor at x=20, y=14: closest beacon is at x=25, y=17",
            "Sensor at x=17, y=20: closest beacon is at x=21, y=22",
            "Sensor at x=16, y=7: closest beacon is at x=15, y=3",
            "Sensor at x=14, y=3: closest beacon is at x=15, y=3",
            "Sensor at x=20, y=1: closest beacon is at x=15, y=3",
        ];
        assert_eq!(do_part1(input.iter(), 10).unwrap(), 26);
        assert_eq!(do_part2(input.iter(), 20).unwrap(), 56000011);
    }
}

fn main() {
    print!("{:?}\n", part1(read_aoc_lines!()));
    print!("{:?}\n", part2(read_aoc_lines!()));
}
