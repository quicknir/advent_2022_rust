use std::{
    cmp::max,
    collections::{HashMap, HashSet, VecDeque},
};

use advent_2022::{read_aoc_lines, InputIterator, OptionUtils};
use anyhow::Result;

#[derive(Default, Clone)]
struct Node {
    flow_rate: i64,
    distances: Vec<i64>,
}

fn split_between<'a>(source: &'a str, left: &str, right: &str) -> Option<&'a str> {
    Some(source.split_once(left)?.1.split_once(right)?.0)
}

fn find_all_distances(
    init_graph: &HashMap<String, (Option<usize>, i64, Vec<String>)>,
    node: &str,
    highest_index: usize,
) -> Vec<i64> {
    let mut visited = HashSet::from([node]);
    let mut nodes = VecDeque::from([(node, 0)]);
    let mut result = vec![0; highest_index + 1];

    while let Some((name, distance)) = nodes.pop_front() {
        for nn in init_graph.get(name).unwrap().2.iter() {
            if visited.contains(nn.as_str()) {
                continue;
            }
            visited.insert(nn.as_str());
            let (index, _, _) = init_graph.get(nn).unwrap();
            match index {
                Some(i) => {
                    result[*i] = distance + 1;
                    nodes.push_back((nn, distance + 1))
                }
                None => nodes.push_back((nn, distance + 1)),
            }
        }
    }
    result
}

fn parse_graph<I: InputIterator>(input: I) -> Result<Vec<Node>> {
    let mut index = 0;

    let init_graph = input
        .map(|l| {
            let line = l.as_ref();
            let name = split_between(line, "Valve ", " ").ok_or_err()?.to_string();
            let flow_rate: i64 = split_between(line, "rate=", ";").ok_or_err()?.parse()?;
            let connections = match line.split_once("valves ") {
                Some(s) => s.1.split(", ").map(|x| x.to_string()).collect(),
                None => {
                    vec![line.split_once("valve ").ok_or_err()?.1.to_string()]
                }
            };
            let i = if name == "AA" {
                Some(0)
            } else if flow_rate == 0 {
                None
            } else {
                index += 1;
                Some(index)
            };
            Ok((name, (i, flow_rate, connections)))
        })
        .collect::<Result<HashMap<_, _>>>()?;

    let mut final_graph: Vec<Node> = vec![Default::default(); index + 1];

    for (k, v) in init_graph.iter() {
        let i = if let Some(i) = v.0 { i } else { continue };
        final_graph[i].flow_rate = v.1;
        final_graph[i].distances = find_all_distances(&init_graph, k, index);
    }

    Ok(final_graph)
}

fn part1_helper(
    graph: &Vec<Node>,
    current_index: usize,
    valves_on: &mut Vec<bool>,
    num_valves_on: usize,
    minutes_left: i64,
) -> i64 {
    if num_valves_on == valves_on.len() {
        return 0;
    }
    if minutes_left <= 0 {
        return 0;
    }

    graph[current_index]
        .distances
        .iter()
        .enumerate()
        .map(|(neighbor, distance)| {
            if valves_on[neighbor] {
                return 0;
            }
            valves_on[neighbor] = true;
            let rest = part1_helper(
                graph,
                neighbor,
                valves_on,
                num_valves_on + 1,
                minutes_left - *distance - 1,
            );
            let result = rest + (minutes_left - *distance - 1) * graph[neighbor].flow_rate;
            valves_on[neighbor] = false;
            result
        })
        .max()
        .unwrap()
}

fn part1<I: InputIterator>(input: I) -> Result<i64> {
    let graph = parse_graph(input)?;
    let mut valves_on = vec![false; graph.len()];
    valves_on[0] = true;
    return Ok(part1_helper(&graph, 0, &mut valves_on, 1, 30));
}

fn part2_helper(
    graph: &Vec<Node>,
    current_index: (usize, usize),
    valves_on: &mut Vec<bool>,
    num_valves_on: usize,
    minutes_left: (i64, i64),
) -> i64 {
    if num_valves_on == valves_on.len() {
        return 0;
    }
    if minutes_left.0 == 0 && minutes_left.1 == 0 {
        return 0;
    }

    (1..valves_on.len())
        .map(|i| {
            if valves_on[i] {
                return 0;
            }

            let my_minutes = minutes_left.0 - graph[current_index.0].distances[i] - 1;
            let ele_minutes = minutes_left.1 - graph[current_index.1].distances[i] - 1;

            if max(my_minutes, ele_minutes) < 0 {
                return 0;
            }

            valves_on[i] = true;

            let result = if my_minutes >= ele_minutes {
                let rest = part2_helper(
                    graph,
                    (i, current_index.1),
                    valves_on,
                    num_valves_on + 1,
                    (my_minutes, minutes_left.1),
                );
                rest + my_minutes * graph[i].flow_rate
            } else {
                let rest = part2_helper(
                    graph,
                    (current_index.0, i),
                    valves_on,
                    num_valves_on + 1,
                    (minutes_left.0, ele_minutes),
                );
                rest + ele_minutes * graph[i].flow_rate
            };

            valves_on[i] = false;
            result
        })
        .max()
        .unwrap()
}

fn part2<I: InputIterator>(input: I) -> Result<i64> {
    let graph = parse_graph(input)?;
    let mut valves_on = vec![false; graph.len()];
    valves_on[0] = true;
    return Ok(part2_helper(&graph, (0, 0), &mut valves_on, 1, (26, 26)));
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;
    #[test]
    fn dayn_test() {
        let input = vec![
            "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB",
            "Valve BB has flow rate=13; tunnels lead to valves CC, AA",
            "Valve CC has flow rate=2; tunnels lead to valves DD, BB",
            "Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE",
            "Valve EE has flow rate=3; tunnels lead to valves FF, DD",
            "Valve FF has flow rate=0; tunnels lead to valves EE, GG",
            "Valve GG has flow rate=0; tunnels lead to valves FF, HH",
            "Valve HH has flow rate=22; tunnel leads to valve GG",
            "Valve II has flow rate=0; tunnels lead to valves AA, JJ",
            "Valve JJ has flow rate=21; tunnel leads to valve II",
        ];
        assert_eq!(part1(input.iter()).unwrap(), 1651);
        assert_eq!(part2(input.iter()).unwrap(), 1707);
    }
}

fn main() {
    print!("{:?}\n", part1(read_aoc_lines!()));
    print!("{:?}\n", part2(read_aoc_lines!()));
}
