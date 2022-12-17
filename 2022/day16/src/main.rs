use std::cmp::max;
use std::collections::HashMap;
use std::io::{self, Read};

use regex::Regex;
use bit_set::BitSet;

struct Valve {
    flow: u32,
    tunnels: BitSet,
}

fn parse(puzzle_input: &str) -> Vec<Valve> {
    let re = Regex::new("^Valve ([A-Z]{2}) has flow rate=(\\d+); tunnels? leads? to valves? ([A-Z ,]+)$").unwrap();
    let mut valves_raw: Vec<_> = puzzle_input.trim_end().lines().map(|line| {
        let cap = re.captures(line).unwrap();
        let name = cap[1].to_string();
        let flow = cap[2].parse().unwrap();
        let tunnels: Vec<_> = cap[3].split(", ").map(|valve| valve.to_string()).collect();
        (name, flow, tunnels)
    }).collect();
    valves_raw.sort_by_key(|(name, _, _)| name.clone());
    let names: HashMap<String,usize> = valves_raw.iter().enumerate()
        .map(|(i, (name, _, _))| (name.clone(), i)).collect();
    valves_raw.iter().map(|(_, flow, tunnels_raw)| {
        let mut tunnels = BitSet::new();
        for name in tunnels_raw.iter() {
            tunnels.insert(*names.get(name).unwrap());
        }
        Valve { flow: *flow, tunnels }
    }).collect()
}

fn flow_rate(valves: &[Valve], open: &BitSet) -> u32 {
    open.iter().map(|valve_id| valves[valve_id].flow).sum()
}

fn part1(valves: &[Valve]) -> u32 {
    let mut next_results = HashMap::new();

    const AA: usize = 0;
    next_results.insert((AA, BitSet::new()), 0);
    for _ in 0..30 {
        let prev_results = next_results;
        next_results = HashMap::new();
        for ((valve_id, open), pressure) in prev_results {
            let pressure = pressure + flow_rate(&valves, &open);

            if !open.contains(valve_id) && valves[valve_id].flow != 0 {
                let mut open = open.clone();
                open.insert(valve_id);
                next_results.entry((valve_id, open))
                    .and_modify(|best_pressure| *best_pressure = max(*best_pressure, pressure))
                    .or_insert(pressure);
            }
            for valve_id in valves[valve_id].tunnels.iter() {
                next_results.entry((valve_id, open.clone()))
                    .and_modify(|best_pressure| *best_pressure = max(*best_pressure, pressure))
                    .or_insert(pressure);
            }
        }
    }
    next_results.values().cloned().max().unwrap()
}

// TODO - This takes about 30 mins to run in release mode. A* is probably a lot faster than a
// purely breadth first approach
fn part2(valves: &[Valve]) -> u32 {
    let mut next_results = HashMap::new();

    const AA: usize = 0;
    next_results.insert(([AA].iter().cloned().collect::<BitSet>(), BitSet::new()), 0);
    for _ in 0..26 {
        let prev_results = next_results;
        next_results = HashMap::new();
        for ((curr, open), pressure) in prev_results {
            let pressure = pressure + flow_rate(&valves, &open);
            let mut curr_it = curr.into_iter();
            let valve_id1 = curr_it.next().unwrap();
            let valve_id2 = if let Some(valve_id2) = curr_it.next() { valve_id2 } else { valve_id1 };
            assert!(curr_it.next().is_none());

            let mut first_choices = Vec::new();
            if !open.contains(valve_id1) && valves[valve_id1].flow != 0 {
                let mut open = open.clone();
                open.insert(valve_id1);
                first_choices.push((valve_id1, open));
            }
            for valve_id1 in valves[valve_id1].tunnels.iter() {
                first_choices.push((valve_id1, open.clone()));
            }

            for (valve_id1, open) in first_choices {
                if !open.contains(valve_id2) && valves[valve_id2].flow != 0 {
                    let mut open = open.clone();
                    open.insert(valve_id2);
                    next_results.entry(([valve_id1, valve_id2].iter().cloned().collect(), open))
                        .and_modify(|best_pressure| *best_pressure = max(*best_pressure, pressure))
                        .or_insert(pressure);
                }
                for valve_id2 in valves[valve_id2].tunnels.iter() {
                    next_results.entry(([valve_id1, valve_id2].iter().cloned().collect(), open.clone()))
                        .and_modify(|best_pressure| *best_pressure = max(*best_pressure, pressure))
                        .or_insert(pressure);
                }
            }
        }
    }
    next_results.values().cloned().max().unwrap()
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let valves = parse(&puzzle_input);
    println!("{}", part1(&valves));
    println!("{}", part2(&valves));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX)), 1651);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EX)), 1707);
    }
}
