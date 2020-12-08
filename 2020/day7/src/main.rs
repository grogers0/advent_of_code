use std::collections::HashMap;
use std::io::{self, Read};

use lazy_static::lazy_static;
use regex::Regex;

type Rules = HashMap<String, Vec<(usize, String)>>;

fn parse(puzzle_input: &str) -> Rules {
    lazy_static! {
        static ref OUTER_RE: Regex = Regex::new("^([a-z]+ [a-z]+) bags contain (.*)\\.$").unwrap();
        static ref INNER_RE: Regex = Regex::new("^(\\d+) ([a-z]+ [a-z]+) bags?$").unwrap();
    }
    puzzle_input.lines().map(|line| {
        let cap = OUTER_RE.captures(line).unwrap();
        let outer_bag = cap[1].to_string();
        let mut inner_bags = Vec::new();
        for inner_str in cap[2].split(", ") {
            if inner_str == "no other bags" { break }
            let cap = INNER_RE.captures(inner_str).unwrap();
            inner_bags.push((cap[1].parse().unwrap(), cap[2].to_string()));
        }
        (outer_bag, inner_bags)
    }).collect()
}

fn part1(rules: &Rules) -> usize {
    fn contained_within(outer_bag: &str, inner_bag: &str, rules: &Rules) -> bool {
        for (_, bag) in rules.get(outer_bag).unwrap() {
            if bag == inner_bag || contained_within(bag, inner_bag, rules) {
                return true
            }
        }
        false
    }

    rules.keys()
        .filter(|outer_bag| contained_within(outer_bag, "shiny gold", rules))
        .count()
}

fn part2(rules: &Rules) -> usize {
    fn num_contained(outer_bag: &str, rules: &Rules) -> usize {
        let mut sum = 0;
        for (cnt, bag) in rules.get(outer_bag).unwrap() {
            sum += cnt * (1 + num_contained(bag, rules));
        }
        sum
    }

    num_contained("shiny gold", rules)
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();
    let rules = parse(&puzzle_input);

    println!("{}", part1(&rules));
    println!("{}", part2(&rules));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    const EX2: &str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    #[test]
    fn test_part1() {
        assert_eq!(4, part1(&parse(EX1)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(32, part2(&parse(EX1)));
        assert_eq!(126, part2(&parse(EX2)));
    }
}
