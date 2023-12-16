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
        let offset = match dir { Dir::L => 0, Dir::R => 1 };
        curr = map.edges[&curr][offset].to_string();
    }
    unreachable!()
}

// FIXME - this is way to slow. Need a non-brute-force method
fn part2(map: &Map) -> usize {
    let mut keys: Vec<_> = map.edges.keys()
        .filter(|key| key.ends_with("A"))
        .map(|key| key.to_string())
        .collect();
    for i in 0.. {
        if keys.iter().all(|key| key.ends_with("Z")) {
            return i;
        }
        let dir = map.dirs[i % map.dirs.len()];
        let offset = match dir { Dir::L => 0, Dir::R => 1 };
        for key in &mut keys {
            *key = map.edges[key][offset].to_string();
        }
    }
    unreachable!()
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
