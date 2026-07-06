use std::collections::HashMap;
use std::io::{self, Read};

use regex::Regex;

#[derive(Copy, Clone)]
enum Dir {
    L, R
}

impl Dir {
    fn parse(ch: char) -> Dir {
        match ch {
            'L' => Dir::L,
            'R' => Dir::R,
            _ => panic!(),
        }
    }

    fn offset(&self) -> usize {
        match *self { Dir::L => 0, Dir::R => 1 }
    }
}

struct Map {
    dirs: Vec<Dir>,
    edges: HashMap<String, [String; 2]>,
}

fn parse(puzzle_input: &str) -> Map {
    let mut paragraphs = puzzle_input.split("\n\n");
    let dirs_str = paragraphs.next().unwrap();
    let edges_str = paragraphs.next().unwrap();
    assert!(paragraphs.next().is_none());
    let dirs = dirs_str.chars().map(|ch| Dir::parse(ch)).collect();
    let mut edges = HashMap::new();
    let regex = Regex::new("([A-Z]{3}) = \\(([A-Z]{3}), ([A-Z]{3})\\)").unwrap();
    for line in edges_str.lines() {
        let cap = regex.captures(line).unwrap();
        edges.insert(cap[1].to_string(), [cap[2].to_string(), cap[3].to_string()]);
    }
    Map { dirs, edges }

}

fn part1(map: &Map) -> usize {
    let mut curr = "AAA".to_string();
    for i in 0.. {
        if curr == "ZZZ" {
            return i;
        }
        let dir = map.dirs[i % map.dirs.len()];
        curr = map.edges[&curr][dir.offset()].to_string();
    }
    unreachable!()
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    (a / gcd(a, b)) * b
}

fn find_period(map: &Map, mut key: String) -> u64 {
    let mut offset = 0;
    for i in 0.. {
        if key.ends_with("Z") {
            if offset == 0 {
                offset = i;
            } else {
                let period = i - offset;
                // A peculiar quirk about the input is that the offset == period so we don't need
                // anything complicated to track the offset and where they line up when the
                // combined period is calculated. Also, the period for a given key is always the
                // same, multiple keys ending in "Z" are not part of the same cycle.
                assert_eq!(offset, period);
                return period;
            }
        }
        let dir = map.dirs[i as usize % map.dirs.len()];
        key = map.edges[&key][dir.offset()].to_string();
    }
    unreachable!()
}

fn part2(map: &Map) -> u64 {
    let periods: Vec<_> = map.edges.keys()
        .filter(|key| key.ends_with("Z"))
        .map(|key| find_period(map, key.to_string()))
        .collect();

    let mut combined_period = 1;
    for period in periods {
        combined_period = lcm(combined_period, period);
    }
    combined_period
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let map = parse(&puzzle_input);
    println!("{}", part1(&map));
    println!("{}", part2(&map));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const EX2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const EX3: &str = "LR

DDA = (DDB, XXX)
DDB = (XXX, DDZ)
DDZ = (DDB, XXX)
EEA = (EEB, XXX)
EEB = (EEC, EEC)
EEC = (EEZ, EEZ)
EEZ = (EEB, EEB)
XXX = (XXX, XXX)";


    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX1)), 2);
        assert_eq!(part1(&parse(EX2)), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EX3)), 6);
    }
}
