use std::collections::{BTreeMap, BTreeSet};
use std::io::{self, Read};

use regex::Regex;
use permutohedron::LexicalPermutation;

fn parse_distances(input: &str) -> BTreeMap<[String; 2], u32> {
    let re = Regex::new("^([a-zA-Z]+) to ([a-zA-Z]+) = (\\d+)$").unwrap();
    let mut distances = BTreeMap::new();
    for line in input.lines() {
        let cap = re.captures(line).unwrap();
        let x = cap[1].to_string();
        let y = cap[2].to_string();
        let d = cap[3].parse().unwrap();
        distances.insert([x.clone(), y.clone()], d);
        distances.insert([y, x], d);
    }
    distances
}

fn trip_distance(route: &Vec<String>, distances: &BTreeMap<[String; 2], u32>) -> u32 {
    let mut dist = 0;
    for pair in route.windows(2) {
        dist += distances.get(pair).unwrap();
    }
    dist
}

fn find_shortest_longest_routes(input: &str) -> (u32, u32) {
    let distances = parse_distances(input);
    let cities: BTreeSet<_> = distances.keys().flat_map(|[city1, city2]| vec![city1, city2]).cloned().collect();
    let mut cities: Vec<_> = cities.into_iter().collect();
    let mut min_dist = std::u32::MAX;
    let mut max_dist = std::u32::MIN;
    while {
        let dist = trip_distance(&cities, &distances);
        if dist < min_dist { min_dist = dist; }
        if dist > max_dist { max_dist = dist; }
        cities.next_permutation()
    } { }
    (min_dist, max_dist)
}

fn part1(input: &str) -> u32 {
    find_shortest_longest_routes(input).0
}

fn part2(input: &str) -> u32 {
    find_shortest_longest_routes(input).1
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
London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), 605);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EX), 982);
    }
}
