use std::cmp::max;
use std::thread;

use advent_2022::{read_aoc_lines, InputIterator, OptionUtils};
use anyhow::Result;

struct BluePrint {
    ore_cost: i64,
    clay_cost: i64,
    obsidian_cost: (i64, i64),
    geode_cost: (i64, i64),
}

#[derive(Debug, Default, Clone, Copy)]
struct State {
    ore: i64,
    clay: i64,
    obsidian: i64,

    ore_bots: i64,
    clay_bots: i64,
    obsidian_bots: i64,

    minutes_left: i64,
    geodes_opened: i64,
}

fn delay(resource: i64, num_bots: i64) -> i64 {
    (resource as f64 / num_bots as f64).ceil() as i64 + 1
}

impl State {
    fn new(num_minutes: i64) -> State {
        State {
            minutes_left: num_minutes,
            ore_bots: 1,
            ..Default::default()
        }
    }

    fn collect(&mut self, minutes: i64) {
        self.ore += minutes * self.ore_bots;
        self.clay += minutes * self.clay_bots;
        self.obsidian += minutes * self.obsidian_bots;
    }

    fn build_ore_bot(&self, blueprint: &BluePrint) -> Option<State> {
        let missing_ore = max(blueprint.ore_cost - self.ore, 0);
        let minutes_delay = delay(missing_ore, self.ore_bots);
        let minutes_left = self.minutes_left - minutes_delay;
        if minutes_left <= 1 {
            return None;
        }
        let mut s = State {
            minutes_left,
            ..*self
        };
        s.collect(minutes_delay);
        s.ore_bots += 1;
        s.ore -= blueprint.ore_cost;
        Some(s)
    }
    fn build_clay_bot(&self, blueprint: &BluePrint) -> Option<State> {
        let missing_ore = max(blueprint.clay_cost - self.ore, 0);
        let minutes_delay = delay(missing_ore, self.ore_bots);
        let minutes_left = self.minutes_left - minutes_delay;
        if minutes_left <= 1 {
            return None;
        }
        let mut s = State {
            minutes_left,
            ..*self
        };
        s.collect(minutes_delay);
        s.clay_bots += 1;
        s.ore -= blueprint.clay_cost;
        Some(s)
    }
    fn build_obsidian_bot(&self, blueprint: &BluePrint) -> Option<State> {
        if self.clay_bots == 0 {
            return None;
        }
        let missing_ore = max(blueprint.obsidian_cost.0 - self.ore, 0);
        let missing_clay = max(blueprint.obsidian_cost.1 - self.clay, 0);
        let minutes_delay = max(
            delay(missing_ore, self.ore_bots),
            delay(missing_clay, self.clay_bots),
        );
        let minutes_left = self.minutes_left - minutes_delay;
        if minutes_left <= 1 {
            return None;
        }
        let mut s = State {
            minutes_left,
            ..*self
        };
        s.collect(minutes_delay);
        s.obsidian_bots += 1;
        s.ore -= blueprint.obsidian_cost.0;
        s.clay -= blueprint.obsidian_cost.1;
        Some(s)
    }
    fn build_geode_bot(&self, blueprint: &BluePrint) -> Option<State> {
        if self.obsidian_bots == 0 {
            return None;
        }
        let missing_ore = max(blueprint.geode_cost.0 - self.ore, 0);
        let missing_obsidian = max(blueprint.geode_cost.1 - self.obsidian, 0);
        let minutes_delay = max(
            delay(missing_ore, self.ore_bots),
            delay(missing_obsidian, self.obsidian_bots),
        );
        let minutes_left = self.minutes_left - minutes_delay;
        if minutes_left <= 0 {
            return None;
        }
        let mut s = State {
            minutes_left,
            ..*self
        };
        s.collect(minutes_delay);
        s.ore -= blueprint.geode_cost.0;
        s.obsidian -= blueprint.geode_cost.1;
        s.geodes_opened += minutes_left;
        Some(s)
    }
}

fn optimize_blueprint(blueprint: &BluePrint, num_minutes: i64) -> i64 {
    let mut v = vec![State::new(num_minutes)];
    let mut max_geodes_opened = 0;

    while let Some(s) = v.pop() {
        max_geodes_opened = max(max_geodes_opened, s.geodes_opened);

        let max_clay =
            s.clay + (s.clay_bots * s.minutes_left) + s.minutes_left * (s.minutes_left - 1) / 2;
        let max_built_obs_bots = max_clay / blueprint.obsidian_cost.1;
        let max_obs =
            s.obsidian + (s.obsidian_bots + (max_built_obs_bots + 1) / 2) * s.minutes_left;
        let max_built_geode_bots = max_obs / blueprint.geode_cost.1;

        if max_built_geode_bots == 0 {
            continue;
        }
        if (max_built_geode_bots * s.minutes_left + s.geodes_opened) <= max_geodes_opened {
            continue;
        }


        if let Some(built_ore) = s.build_ore_bot(blueprint) {
            v.push(built_ore);
        }
        if let Some(built_clay) = s.build_clay_bot(blueprint) {
            v.push(built_clay);
        }
        if let Some(built_obsidian) = s.build_obsidian_bot(blueprint) {
            v.push(built_obsidian);
        }
        if let Some(built_geode) = s.build_geode_bot(blueprint) {
            v.push(built_geode);
        }
    }
    max_geodes_opened
}

fn parse_line(s: &str) -> Result<BluePrint> {
    let mut split = s.split(' ');
    let ore_cost = split.nth(6).ok_or_err()?.parse()?;
    let clay_cost = split.nth(5).ok_or_err()?.parse()?;
    let obsidian_cost = (
        split.nth(5).ok_or_err()?.parse()?,
        split.nth(2).ok_or_err()?.parse()?,
    );
    let geode_cost = (
        split.nth(5).ok_or_err()?.parse()?,
        split.nth(2).ok_or_err()?.parse()?,
    );

    Ok(BluePrint {
        ore_cost,
        clay_cost,
        obsidian_cost,
        geode_cost,
    })
}

fn part1<I: InputIterator>(input: I) -> Result<usize> {
    let blueprints = input
        .map(|l| parse_line(l.as_ref()))
        .collect::<Result<Vec<_>>>()?;
    let results: Vec<_> = thread::scope(|s| {
        let threads: Vec<_> = blueprints
            .iter()
            .map(|b| s.spawn(|| optimize_blueprint(b, 24)))
            .collect();
        threads.into_iter().map(|x| x.join().unwrap()).collect()
    });
    Ok(results
        .iter()
        .enumerate()
        .map(|(i, r)| (i + 1) * (*r as usize))
        .sum())
}
fn part2<I: InputIterator>(input: I) -> Result<i64> {
    let blueprints = input
        .map(|l| parse_line(l.as_ref()))
        .collect::<Result<Vec<_>>>()?;
    let results: Vec<_> = thread::scope(|s| {
        let threads: Vec<_> = blueprints[0..3]
            .iter()
            .map(|b| s.spawn(|| optimize_blueprint(b, 32)))
            .collect();
        threads.into_iter().map(|x| x.join().unwrap()).collect()
    });
    Ok(results.iter().fold(1, |acc, x| acc * x))
}

#[cfg(test)]
mod tests {
    use crate::optimize_blueprint;
    use crate::BluePrint;
    #[test]
    fn test_blueprint() {
        let blueprint1 = BluePrint {
            ore_cost: 4,
            clay_cost: 2,
            obsidian_cost: (3, 14),
            geode_cost: (2, 7),
        };
        assert_eq!(optimize_blueprint(&blueprint1, 24), 9);
        assert_eq!(optimize_blueprint(&blueprint1, 32), 56);

        let blueprint2 = BluePrint {
            ore_cost: 2,
            clay_cost: 3,
            obsidian_cost: (3, 8),
            geode_cost: (3, 12),
        };
        assert_eq!(optimize_blueprint(&blueprint2, 24), 12);
    }
}

fn main() {
    print!("{:?}\n", part1(read_aoc_lines!()));
    print!("{:?}\n", part2(read_aoc_lines!()));
}
