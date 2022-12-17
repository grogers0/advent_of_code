use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::io::{self, Read};

use regex::Regex;

struct Valve {
    flow: u32,
    tunnels: BitSet64,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct BitSet64(u64);

impl BitSet64 {
    fn new() -> Self {
        Self(0)
    }

    fn contains(&self, idx: usize) -> bool {
        self.0 & (1 << idx) != 0
    }

    fn insert(&mut self, idx: usize) -> bool {
        let prev = self.0;
        self.0 |= 1 << idx;
        prev != self.0
    }

    fn singleton(idx: usize) -> Self {
        let mut ret = Self::new();
        ret.insert(idx);
        ret
    }

    fn pair(idx1: usize, idx2: usize) -> Self {
        let mut ret = Self::new();
        ret.insert(idx1);
        ret.insert(idx2);
        ret
    }
}

impl IntoIterator for BitSet64 {
    type Item = usize;
    type IntoIter = BitSet64IntoIter;
    fn into_iter(self) -> BitSet64IntoIter {
        BitSet64IntoIter(self.0)
    }
}

struct BitSet64IntoIter(u64);

impl Iterator for BitSet64IntoIter {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        if self.0 == 0 {
            return None
        }
        let zeros = self.0.trailing_zeros();
        self.0 &= !(1 << (zeros));
        Some(zeros as usize)
    }
}

fn parse(puzzle_input: &str) -> (Vec<Valve>, Vec<usize>) {
    let re = Regex::new("^Valve ([A-Z]{2}) has flow rate=(\\d+); tunnels? leads? to valves? ([A-Z ,]+)$").unwrap();
    let mut valves_raw: Vec<_> = puzzle_input.trim_end().lines().map(|line| {
        let cap = re.captures(line).unwrap();
        let name = cap[1].to_string();
        let flow = cap[2].parse().unwrap();
        let tunnels: Vec<_> = cap[3].split(", ").map(|valve| valve.to_string()).collect();
        (name, flow, tunnels)
    }).collect();
    assert!(valves_raw.len() <= 64);
    valves_raw.sort_by_key(|(name, _, _)| name.clone());
    let names: HashMap<String,usize> = valves_raw.iter().enumerate()
        .map(|(i, (name, _, _))| (name.clone(), i)).collect();
    let valves: Vec<_> = valves_raw.iter().map(|(_, flow, tunnels_raw)| {
        let mut tunnels = BitSet64::new();
        for name in tunnels_raw.iter() {
            tunnels.insert(*names.get(name).unwrap());
        }
        Valve { flow: *flow, tunnels }
    }).collect();
    let mut sorted_valve_ids: Vec<_> = (0..valves.len()).into_iter().collect();
    sorted_valve_ids.sort_by_key(|valve_id| Reverse(valves[*valve_id].flow));
    (valves, sorted_valve_ids)
}

fn flow_rate(valves: &[Valve], open: BitSet64) -> u32 {
    open.into_iter().map(|valve_id| valves[valve_id].flow).sum()
}

// Pick the best possible valves in as little time as possible to be an admissable heuristic for A*
fn part1_heuristic(valves: &[Valve], sorted_valve_ids: &[usize], mut mins_left: u32, mut open: BitSet64) -> u32 {
    let mut pressure = 0;
    if mins_left == 0 { return pressure; }
    mins_left -= 1;
    pressure += flow_rate(valves, open);
    if mins_left == 0 { return pressure; }
    for valve_id in sorted_valve_ids {
        if open.insert(*valve_id) {
            mins_left -= 1;
            pressure += flow_rate(&valves, open);
            if mins_left == 0 { return pressure; }
        }
    }
    for _ in 0..mins_left {
        pressure += flow_rate(&valves, open);
    }
    pressure
}

fn part2_heuristic(valves: &[Valve], sorted_valve_ids: &[usize], mut mins_left: u32, mut open: BitSet64) -> u32 {
    let mut pressure = 0;
    if mins_left == 0 { return pressure; }
    mins_left -= 1;
    pressure += flow_rate(valves, open);
    if mins_left == 0 { return pressure; }
    let mut first = true;
    for valve_id in sorted_valve_ids {
        if open.insert(*valve_id) {
            if first {
                first = false;
            } else {
                mins_left -= 1;
                pressure += flow_rate(&valves, open);
                if mins_left == 0 { return pressure; }
            }
        }
    }
    for _ in 0..mins_left {
        pressure += flow_rate(&valves, open);
    }
    pressure
}


fn part1(valves: &[Valve], sorted_valve_ids: &[usize]) -> u32 {
    let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();

    const AA: usize = 0;
    {
        // I *think* because tiebreakers in the heuristic make us pick by the most mins_left, we're
        // guaranteed to always get the correct answer when we pop the mins_left=0 element.
        let max_pressure = part1_heuristic(valves, sorted_valve_ids, 30, BitSet64::new());
        heap.push((max_pressure, 30, 0, AA, BitSet64::new()));
    }
    while let Some((_, mut mins_left, mut pressure, valve_id, open)) = heap.pop() {
        if mins_left == 0 {
            return pressure;
        }
        if !seen.insert((mins_left, valve_id, open)) {
            continue;
        }
        mins_left -= 1;
        pressure += flow_rate(&valves, open);

        if !open.contains(valve_id) && valves[valve_id].flow != 0 {
            let mut open = open;
            open.insert(valve_id);
            let max_pressure = pressure + part1_heuristic(valves, sorted_valve_ids, mins_left, open);
            heap.push((max_pressure, mins_left, pressure, valve_id, open));
        }
        let max_pressure = pressure + part1_heuristic(valves, sorted_valve_ids, mins_left, open);
        for valve_id in valves[valve_id].tunnels {
            heap.push((max_pressure, mins_left, pressure, valve_id, open));
        }
    }
    panic!()
}

// Doesn't quite finish in under 1 sec in release mode but I can't think of anything else to speed it up
fn part2(valves: &[Valve], sorted_valve_ids: &[usize]) -> u32 {
    let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();

    const AA: usize = 0;
    {
        let max_pressure = part2_heuristic(valves, sorted_valve_ids, 26, BitSet64::new());
        heap.push((max_pressure, 26, 0, BitSet64::singleton(AA), BitSet64::new()));
    }
    while let Some((_, mut mins_left, mut pressure, curr, open)) = heap.pop() {
        if mins_left == 0 {
            return pressure;
        }
        if !seen.insert((mins_left, curr.clone(), open)) {
            continue;
        }
        mins_left -= 1;
        pressure += flow_rate(&valves, open);

        let mut curr_it = curr.into_iter();
        let valve_id1 = curr_it.next().unwrap();
        let valve_id2 = if let Some(valve_id2) = curr_it.next() { valve_id2 } else { valve_id1 };
        assert!(curr_it.next().is_none());

        let mut first_choices = Vec::new();
        if !open.contains(valve_id1) && valves[valve_id1].flow != 0 {
            let mut open = open;
            open.insert(valve_id1);
            first_choices.push((valve_id1, open));
        }
        for valve_id1 in valves[valve_id1].tunnels {
            first_choices.push((valve_id1, open));
        }

        for (valve_id1, open) in first_choices {
            if !open.contains(valve_id2) && valves[valve_id2].flow != 0 {
                let mut open = open;
                open.insert(valve_id2);
                let max_pressure = pressure + part2_heuristic(valves, sorted_valve_ids, mins_left, open);
                let curr = BitSet64::pair(valve_id1, valve_id2);
                heap.push((max_pressure, mins_left, pressure, curr, open));
            }
            let max_pressure = pressure + part2_heuristic(valves, sorted_valve_ids, mins_left, open);
            for valve_id2 in valves[valve_id2].tunnels {
                let curr = BitSet64::pair(valve_id1, valve_id2);
                heap.push((max_pressure, mins_left, pressure, curr, open));
            }
        }
    }
    panic!()
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let (valves, sorted_valve_ids) = parse(&puzzle_input);
    println!("{}", part1(&valves, &sorted_valve_ids));
    println!("{}", part2(&valves, &sorted_valve_ids));
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
        let (valves, sorted_valve_ids) = parse(EX);
        assert_eq!(part1(&valves, &sorted_valve_ids), 1651);
    }

    #[test]
    fn test_part2() {
        let (valves, sorted_valve_ids) = parse(EX);
        assert_eq!(part2(&valves, &sorted_valve_ids), 1707);
    }
}
