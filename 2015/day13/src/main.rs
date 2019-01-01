use std::collections::{BTreeMap, BTreeSet};
use std::io::{self, Read};

use permutohedron::LexicalPermutation;
use regex::Regex;

fn parse(input: &str) -> BTreeMap<[String; 2], i64> {
    let re = Regex::new("^([A-Za-z]+) would (gain|lose) (\\d+) happiness units by sitting next to ([A-Za-z]+).$").unwrap();
    let mut pairs = BTreeMap::new();
    for line in input.lines() {
        let cap = re.captures(line).unwrap();
        let a = cap[1].to_string();
        let mut amount: i64 = cap[3].parse().unwrap();
        if &cap[2] == "lose" { amount = -amount; }
        let b = cap[4].to_string();
        pairs.insert([a, b], amount);
    }
    pairs
}

fn sum_happiness(happiness_pairs: &BTreeMap<[String; 2], i64>, people: &Vec<String>) -> i64 {
    let mut sum = 0;
    let n = people.len();
    for i in 0..n {
        sum += happiness_pairs[&[people[i].to_string(), people[(i + 1) % n].to_string()]];
        sum += happiness_pairs[&[people[i].to_string(), people[(i + n - 1) % n].to_string()]];
    }
    sum
}

fn best_happiness_sum(happiness_pairs: &BTreeMap<[String; 2], i64>) -> i64 {
    let people: BTreeSet<_> = happiness_pairs.keys().flat_map(|[a,b]| vec![a.clone(), b.clone()]).collect();
    let mut people: Vec<_> = people.into_iter().collect();
    let mut best_sum = i64::min_value();
    while {
        let sum = sum_happiness(happiness_pairs, &people);
        if sum > best_sum { best_sum = sum; }
        people.next_permutation()
    } {}
    best_sum
}

fn part1(input: &str) -> i64 {
    let happiness_pairs = parse(input);
    best_happiness_sum(&happiness_pairs)
}

fn part2(input: &str) -> i64 {
    let mut happiness_pairs = parse(input);
    let people: BTreeSet<_> = happiness_pairs.keys().flat_map(|[a,b]| vec![a.clone(), b.clone()]).collect();
    let myself = "myself";
    for person in people {
        happiness_pairs.insert([myself.to_string(), person.to_string()], 0);
        happiness_pairs.insert([person.to_string(), myself.to_string()], 0);
    }

    best_happiness_sum(&happiness_pairs)
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
Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), 330);
    }
}
