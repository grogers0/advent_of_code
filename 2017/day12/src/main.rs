use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::io::{self, Read};

use lazy_static::lazy_static;
use regex::Regex;

fn parse(input: &str) -> BTreeMap<usize, BTreeSet<usize>> {
    lazy_static! {
        static ref RE: Regex = Regex::new("^(\\d+) <-> ((\\d+(, )?)+)$").unwrap();
    }
    input.lines()
        .map(|line| {
            let cap = RE.captures(line).unwrap();
            let id = cap[1].parse().unwrap();
            let others = cap[2].split(", ").map(|o| o.parse().unwrap()).collect();
            (id, others)
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let pipes = parse(input);
    let mut group: BTreeSet<usize> = BTreeSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(0);

    while let Some(id) = queue.pop_front() {
        if group.contains(&id) { continue }
        group.insert(id);

        for other in pipes[&id].iter() {
            queue.push_back(*other);
        }
    }

    group.len()
}

fn part2(input: &str) -> usize {
    let pipes = parse(input);
    let pids: Vec<usize> = pipes.keys().map(|id| *id).collect();
    let mut seen = BTreeSet::new();
    let mut num_groups = 0;
    let mut queue = VecDeque::new();

    for starting_id in pids {
        if seen.contains(&starting_id) { continue }
        num_groups += 1;

        queue.push_back(starting_id);
        while let Some(id) = queue.pop_front() {
            if seen.contains(&id) { continue }
            seen.insert(id);
            for other in pipes[&id].iter() {
                queue.push_back(*other);
            }
        }
    }

    num_groups
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
0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EX), 2);
    }

}
