use std::{
    cmp::max,
    collections::HashMap,
};

use advent_2022::{read_aoc_lines, Coord, Grid, InputError, InputIterator, OptionUtils};
use anyhow::Result;

fn rocks() -> [Vec<Coord>; 5] {
    [
        //horizontal
        vec![
            Coord { i: 0, j: 0 },
            Coord { i: 0, j: 1 },
            Coord { i: 0, j: 2 },
            Coord { i: 0, j: 3 },
        ],
        // star
        vec![
            Coord { i: 1, j: 0 },
            Coord { i: 1, j: 1 },
            Coord { i: 1, j: 2 },
            Coord { i: 0, j: 1 },
            Coord { i: 2, j: 1 },
        ],
        // backwards L
        vec![
            Coord { i: 0, j: 0 },
            Coord { i: 0, j: 1 },
            Coord { i: 0, j: 2 },
            Coord { i: 1, j: 2 },
            Coord { i: 2, j: 2 },
        ],
        // Vertical
        vec![
            Coord { i: 0, j: 0 },
            Coord { i: 1, j: 0 },
            Coord { i: 2, j: 0 },
            Coord { i: 3, j: 0 },
        ],
        // square
        vec![
            Coord { i: 0, j: 0 },
            Coord { i: 0, j: 1 },
            Coord { i: 1, j: 0 },
            Coord { i: 1, j: 1 },
        ],
    ]
}

fn rock_heights() -> [i64; 5] {
    [0, 2, 2, 3, 1]
}

#[allow(dead_code)]
fn debug_draw(chamber: &Grid<bool>) {
    for i in (0..chamber.height()).rev() {
        let line: String = (0..chamber.width())
            .map(|j| chamber[(i, j)])
            .map(|x| if x { '#' } else { '.' })
            .collect();
        println!("{}", line);
    }
}

fn part1<I: InputIterator>(mut input: I) -> Result<i64> {
    let jet_pattern = input
        .next()
        .ok_or_err()?
        .as_ref()
        .chars()
        .map({
            |x| match x {
                '<' => Ok(Coord { i: 0, j: -1 }),
                '>' => Ok(Coord { i: 0, j: 1 }),
                _ => Err(InputError::new("").into()),
            }
        })
        .collect::<Result<Vec<_>>>()?;
    let mut jet_index = 0;
    let rocks = rocks();
    let rock_heights = rock_heights();
    let mut max_height = -1;
    let num_shapes = 2022;
    let mut chamber = Grid::new((num_shapes as i64) * 5, 7, false);

    for rock_index in 0..num_shapes {
        let shape = &rocks[rock_index % 5];
        let shape_height = rock_heights[rock_index % 5];
        let mut shape_coord = Coord {
            i: max_height + 4,
            j: 2,
        };
        loop {
            let blown_coord = shape_coord + jet_pattern[jet_index % jet_pattern.len()];
            jet_index += 1;
            let collision = shape
                .iter()
                .any(|p| *chamber.get(blown_coord + *p).unwrap_or(&true));
            if !collision {
                shape_coord = blown_coord;
            }
            let fall_coord = shape_coord + Coord { i: -1, j: 0 };
            let collision = shape
                .iter()
                .any(|p| *chamber.get(fall_coord + *p).unwrap_or(&true));

            if collision {
                break;
            }
            shape_coord = fall_coord;
        }
        shape.iter().for_each(|p| {
            chamber[shape_coord + *p] = true;
        });
        max_height = max(max_height, shape_coord.i + shape_height);
        // debug_draw(&chamber);
        // println!("");
        // println!("");
    }

    Ok(max_height + 1)
}
fn part2<I: InputIterator>(mut input: I) -> Result<i64> {
    let jet_pattern = input
        .next()
        .ok_or_err()?
        .as_ref()
        .chars()
        .map({
            |x| match x {
                '<' => Ok(Coord { i: 0, j: -1 }),
                '>' => Ok(Coord { i: 0, j: 1 }),
                _ => Err(InputError::new("").into()),
            }
        })
        .collect::<Result<Vec<_>>>()?;
    let mut jet_index = 0;
    let rocks = rocks();
    let rock_heights = rock_heights();
    let num_shapes = 1000000;
    let mut chamber = Grid::new((num_shapes as i64) * 5, 7, false);
    let mut max_heights = vec![];

    // cycle detection state
    let mut cycle_detector = HashMap::new();
    let mut cycle_start_state: Option<_> = None;
    let mut cycle_start: usize = 0;
    let mut cycle_end: usize = 0;

    for rock_index in 0..num_shapes {
        let shape = &rocks[rock_index % 5];
        let shape_height = rock_heights[rock_index % 5];
        let mut shape_coord = Coord {
            i: max_heights.last().unwrap_or(&-1) + 4,
            j: 2,
        };
        loop {
            let blown_coord = shape_coord + jet_pattern[jet_index % jet_pattern.len()];
            jet_index += 1;
            let collision = shape
                .iter()
                .any(|p| *chamber.get(blown_coord + *p).unwrap_or(&true));
            if !collision {
                shape_coord = blown_coord;
            }
            let fall_coord = shape_coord + Coord { i: -1, j: 0 };
            let collision = shape
                .iter()
                .any(|p| *chamber.get(fall_coord + *p).unwrap_or(&true));

            if collision {
                break;
            }
            shape_coord = fall_coord;
        }
        shape.iter().for_each(|p| {
            chamber[shape_coord + *p] = true;
        });

        max_heights.push(max(
            *max_heights.last().unwrap_or(&-1),
            shape_coord.i + shape_height,
        ));

        let state = (
            Coord {
                i: max_heights.last().unwrap() - shape_coord.i,
                j: shape_coord.j,
            },
            rock_index % 5,
            jet_index % jet_pattern.len(),
        );
        if let Some(s) = cycle_start_state {
            if s == state {
                cycle_end = rock_index;
                break;
            }
        }
        if let Some(_i) = cycle_detector.get(&state) { // we have a match
            cycle_start_state = match cycle_start_state {
                None => {  // starting a new cycle
                    cycle_start = rock_index;
                    Some(state)
                }
                Some(s) => {  // continuing an existing one
                    Some(s)
                }
            };
        }
        else {
            cycle_start_state = None; // end any cycle if tracking
        }
        cycle_detector.insert(state, rock_index);
    }

    let cycle_duration = cycle_end - cycle_start;
    let cycle_height = max_heights[cycle_end] - max_heights[cycle_start];
    let shapes_left = 1000000000000 - cycle_end;
    let full_cycles = (shapes_left / cycle_duration) as i64;
    let leftover = shapes_left % cycle_duration;
    let total = max_heights[cycle_end]
        + full_cycles * cycle_height
        + (max_heights[cycle_start + leftover] - max_heights[cycle_start]);

    Ok(total)
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;
    #[test]
    fn day17_test() {
        let input = vec![">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"];
        assert_eq!(part1(input.iter()).unwrap(), 3068);
        assert_eq!(part2(input.iter()).unwrap(), 1514285714288);
    }
}

fn main() {
    print!("{:?}\n", part1(read_aoc_lines!()));
    print!("{:?}\n", part2(read_aoc_lines!()));
}
