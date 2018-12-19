use std::collections::{BTreeMap, BTreeSet};
use std::io::{self, Read};

use lazy_static::lazy_static;
use regex::Regex;

fn parse(input: &str) -> Vec<(char, char)> {
    lazy_static! {
        static ref RE: Regex = Regex::new("^Step ([a-zA-Z]+) must be finished before step ([a-zA-Z]+) can begin\\.$").unwrap();
    }

    input.lines()
        .map(|line| {
            let cap = RE.captures(line).unwrap();
            (cap[1].parse().unwrap(), cap[2].parse().unwrap())
        })
        .collect()
}

fn all_nodes(edges: &Vec<(char, char)>) -> BTreeSet<char> {
    edges.iter().map(|(c,_)| *c).chain(edges.iter().map(|(_,c)| *c)).collect()
}

fn part1(input: &str) -> String {
    let mut edges = parse(input);
    let mut nodes = all_nodes(&edges);
    let mut ret = String::new();

    while !nodes.is_empty() {
        let node = nodes.iter()
            .map(|ch| *ch)
            .filter(|ch| edges.iter().all(|edge| edge.1 != *ch))
            .next().unwrap();
        ret.push(node);
        nodes.remove(&node);
        edges.retain(|edge| edge.0 != node);
    }

    ret
}

fn step_time(node: char, base_time: usize) -> usize {
    (node as usize - 'A' as usize) + base_time + 1
}

fn calc_part2(input: &str, num_workers: usize, base_time: usize) -> usize {
    let mut edges = parse(input);
    let mut nodes = all_nodes(&edges);
    let mut workers: BTreeMap<char, usize> = BTreeMap::new();
    let mut time = 0;

    while !nodes.is_empty() || !workers.is_empty() {
        let available_node = nodes.iter()
            .map(|node| *node)
            .filter(|node| !workers.contains_key(node))
            .filter(|node| edges.iter().all(|edge| edge.1 != *node))
            .next();
        if workers.len() < num_workers && available_node.is_some() {
            let node = available_node.unwrap();
            workers.insert(node, time + step_time(node, base_time));
        } else {
            assert!(!workers.is_empty());
            let (node, next_time) = workers.iter()
                .map(|(n,t)| (*n, *t)).min_by_key(|(_,t)| *t).unwrap();
            assert!(next_time >= time);
            time = next_time;
            workers.remove(&node);
            nodes.remove(&node);
            edges.retain(|edge| edge.0 != node);
        }
    }

    time
}

fn part2(input: &str) -> usize {
    calc_part2(input, 5, 60)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}


#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "\
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), "CABDFE".to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(calc_part2(EX, 2, 0), 15);
    }

}
