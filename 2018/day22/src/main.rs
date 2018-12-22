use std::collections::{BTreeMap, VecDeque};
use std::io::{self, Read};

use lazy_static::lazy_static;
use regex::Regex;

// Returns (depth, target_pos)
fn parse(input: &str) -> (usize, (usize, usize)) {
    lazy_static!{
        static ref DEPTH_RE: Regex = Regex::new("^depth: (\\d+)$").unwrap();
        static ref TARGET_RE: Regex = Regex::new("^target: (\\d+),(\\d+)").unwrap();
    }
    let mut lines_iter = input.lines();
    let depth = DEPTH_RE.captures(lines_iter.next().unwrap()).unwrap()[1].parse().unwrap();
    let cap = TARGET_RE.captures(lines_iter.next().unwrap()).unwrap();
    let target_pos = (cap[1].parse().unwrap(), cap[2].parse().unwrap());
    (depth, target_pos)
}

fn erosion_level(depth: usize, geologic_index: usize) -> usize {
    (geologic_index + depth) % 20183
}

fn risk_level(depth: usize, geologic_index: usize) -> usize {
    erosion_level(depth, geologic_index) % 3
}

fn build_geologic_index(depth: usize, target: (usize, usize), width: usize, height: usize) -> Vec<Vec<usize>> {
    let mut geologic_index = vec![vec![0usize; width]; height];
    for y in 0 .. height {
        for x in 0 .. width {
            geologic_index[y][x] = if (x, y) == (0, 0) {
                0
            } else if (x, y) == target {
                0 
            } else if y == 0 {
                x * 16807
            } else if x == 0 {
                y * 48271
            } else {
                erosion_level(depth, geologic_index[y-1][x]) *
                    erosion_level(depth, geologic_index[y][x-1])
            };
        }
    }
    geologic_index
}

fn build_map(depth: usize, mut geologic_index: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    for row in geologic_index.iter_mut() {
        for sq in row.iter_mut() {
            *sq = risk_level(depth, *sq);
        }
    }
    geologic_index
}

fn part1(input: &str) -> usize {
    let (depth, target) = parse(input);
    let geologic_index = build_geologic_index(depth, target, target.0 + 1, target.1 + 1);
    let map = build_map(depth, geologic_index);

    map.iter().map(|row| row.iter().sum::<usize>()).sum::<usize>()
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Tool {
    Torch,
    ClimbingGear,
    Neither
}

fn tool_allowed(square: usize, tool: Tool) -> bool {
    match square {
        0 => tool == Tool::Torch || tool == Tool::ClimbingGear,
        1 => tool == Tool::ClimbingGear || tool == Tool::Neither,
        2 => tool == Tool::Torch || tool == Tool::Neither,
        _ => unreachable!()
    }
}

fn part2(input: &str) -> usize {
    let (depth, target) = parse(input);
    let extra = 100; // Arbitrary extra amount, can we calculate the exact right amount to use?
    let width = target.0 + extra;
    let height = target.1 + extra;
    let geologic_index = build_geologic_index(depth, target, width, height);
    let map = build_map(depth, geologic_index);

    let mut visits = BTreeMap::new();
    visits.insert(Tool::Torch, vec![vec![std::usize::MAX; width]; height]);
    visits.insert(Tool::ClimbingGear, vec![vec![std::usize::MAX; width]; height]);
    visits.insert(Tool::Neither, vec![vec![std::usize::MAX; width]; height]);

    let mut queue = VecDeque::new();
    queue.push_back((0, 0, Tool::Torch, 0));

    while let Some((x, y, tool, dist)) = queue.pop_front() {
        if !tool_allowed(map[y][x], tool) { continue }
        if visits[&tool][y][x] <= dist { continue }
        visits.get_mut(&tool).unwrap()[y][x] = dist;

        queue.push_back((x, y, Tool::Torch, dist+7));
        queue.push_back((x, y, Tool::ClimbingGear, dist+7));
        queue.push_back((x, y, Tool::Neither, dist+7));
        if x > 0          { queue.push_back((x-1, y, tool, dist+1)); }
        if x < width - 1  { queue.push_back((x+1, y, tool, dist+1)); }
        if y > 0          { queue.push_back((x, y-1, tool, dist+1)); }
        if y < height - 1 { queue.push_back((x, y+1, tool, dist+1)); }
    }
    visits[&Tool::Torch][target.1][target.0]
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

    const EX: &str = "depth: 510\ntarget: 10,10\n";
    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), 114);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EX), 45);
    }
}
