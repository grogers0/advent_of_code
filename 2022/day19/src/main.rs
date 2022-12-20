use std::cmp::Reverse;
use std::collections::HashSet;
use std::io::{self, Read};
use std::ops::{Index, IndexMut};

use regex::Regex;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Resource {
    Ore, Clay, Obsidian, Geode
}

const NUM_RESOURCES: usize = 4;
const RESOURCES: [Resource; NUM_RESOURCES] = [
    Resource::Ore, Resource::Clay, Resource::Obsidian, Resource::Geode,
];

impl Resource {
    fn parse(s: &str) -> Self {
        match s {
            "ore" => Resource::Ore,
            "clay" => Resource::Clay,
            "obsidian" => Resource::Obsidian,
            "geode" => Resource::Geode,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq, Hash)]
struct ResourceMap<T>([T; NUM_RESOURCES]);

impl <T> Index<Resource> for ResourceMap<T> {
    type Output = T;
    fn index(&self, resource: Resource) -> &T {
        &self.0[resource as usize]
    }
}

impl <T> IndexMut<Resource> for ResourceMap<T> {
    fn index_mut(&mut self, resource: Resource) -> &mut T {
        &mut self.0[resource as usize]
    }
}

// Resource collected by robot -> cost by resource
struct Blueprint(ResourceMap<ResourceMap<usize>>);

fn parse(puzzle_input: &str) -> Vec<Blueprint> {
    let robot_re = Regex::new("^Each ([a-z]+) robot costs (.+?)\\.?$").unwrap();
    let cost_re = Regex::new("^(\\d+) ([a-z]+)$").unwrap();
    puzzle_input.trim_end().lines().enumerate().map(|(i, line)| {
        let blueprint_prefix = format!("Blueprint {}: ", i + 1);
        assert!(line.starts_with(&blueprint_prefix));
        let line = &line[blueprint_prefix.len()..];
        let mut robots = ResourceMap::<ResourceMap<usize>>::default();
        for robot_input in line.split(". ") {
            let robot_cap = robot_re.captures(robot_input).unwrap();
            let collect_resource = Resource::parse(&robot_cap[1]);
            let robot = &mut robots[collect_resource];
            for cost_str in robot_cap[2].split(" and ") {
                let cost_cap = cost_re.captures(cost_str).unwrap();
                let cost_num: usize = cost_cap[1].parse().unwrap();
                let cost_resource = Resource::parse(&cost_cap[2]);
                robot[cost_resource] += cost_num;
            }
        }
        Blueprint(robots)
    }).collect()
}

fn has_enough(collected: &ResourceMap<usize>, cost: &ResourceMap<usize>) -> bool {
    for resource in RESOURCES {
        if collected[resource] < cost[resource] {
            return false
        }
    }
    return true
}

fn subtract_cost(collected: &mut ResourceMap<usize>, cost: &ResourceMap<usize>) {
    for resource in RESOURCES {
        collected[resource] -= cost[resource];
    }
}

// TODO - this is approximating a best first search by doing a breadth first search and throwing
// away all but the best N so far at each step. This is definitely not guaranteed to find the
// optimal solution, but using all nodes was far too expensive, and with high enough N it seems to
// work ok. I couldn't think of a decent heuristic like with day16. Think about how this can be
// improved.
fn max_geodes(blueprint: &Blueprint, mins_left: usize) -> usize {
    let mut next = HashSet::new();
    {
        let mut robots = ResourceMap::<usize>::default();
        robots[Resource::Ore] += 1usize;
        next.insert((robots, ResourceMap::<usize>::default()));
    }
    for _ in 0..mins_left {
        let prev = next;
        next = HashSet::new();
        for (robots, orig_collected) in prev {
            let mut collected = orig_collected.clone();
            for resource in RESOURCES {
                collected[resource] += robots[resource];
            }

            next.insert((robots.clone(), collected.clone()));
            for resource in RESOURCES {
                let cost = &blueprint.0[resource];
                if has_enough(&orig_collected, cost) {
                    let mut collected = collected.clone();
                    subtract_cost(&mut collected, cost);
                    let mut robots = robots.clone();
                    robots[resource] += 1;
                    next.insert((robots, collected));
                }
            }
        }

        let mut sorted = next.into_iter().collect::<Vec<_>>();
        sorted.sort_by_key(|(robots, collected)|
            Reverse((
                    collected[Resource::Geode], robots[Resource::Geode],
                    collected[Resource::Obsidian], robots[Resource::Obsidian],
                    collected[Resource::Clay], robots[Resource::Clay],
                    collected[Resource::Ore], robots[Resource::Ore],
            )));
        next = sorted.into_iter().take(10_000).collect();
    }
    next.iter().map(|(_, collected)| collected[Resource::Geode])
        .max().unwrap()
}

fn part1(blueprints: &[Blueprint]) -> usize {
    blueprints.iter().enumerate()
        .map(|(i, blueprint)| (i + 1) * max_geodes(blueprint, 24))
        .sum()
}

fn part2(blueprints: &[Blueprint]) -> usize {
    blueprints.iter().take(3)
        .map(|blueprint| max_geodes(blueprint, 32))
        .product()
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let blueprints = parse(&puzzle_input);
    println!("{}", part1(&blueprints));
    println!("{}", part2(&blueprints));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX)), 33);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EX)), 56 * 62);
    }
}
