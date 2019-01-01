use std::collections::BTreeMap;
use std::io::{self, Read};

use regex::Regex;

fn parse(input: &str) -> Vec<BTreeMap<String, u32>> {
    let re = Regex::new("([a-z]+): (\\d+)").unwrap(); 
    input.lines()
        .map(|line| {
            re.captures_iter(line).map(|cap| {
                (cap[1].to_string(), cap[2].parse().unwrap())
            }).collect()
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let aunts = parse(input);
    let reading = parse("children: 3, cats: 7, samoyeds: 2, pomeranians: 3, akitas: 0, vizslas: 0, goldfish: 5, trees: 3, cars: 2, perfumes: 1").into_iter().next().unwrap();

    for (i, aunt) in aunts.iter().enumerate() {
        let mut matches = true;
        for (k, v) in aunt.iter() {
            if *v != reading[k] {
                matches = false;
            }
        }
        if matches { return i + 1; }
    }
    panic!("No aunt matched");
}

fn part2(input: &str) -> usize {
    let aunts = parse(input);
    let reading = parse("children: 3, cats: 7, samoyeds: 2, pomeranians: 3, akitas: 0, vizslas: 0, goldfish: 5, trees: 3, cars: 2, perfumes: 1").into_iter().next().unwrap();

    for (i, aunt) in aunts.iter().enumerate() {
        let mut matches = true;
        for (k, v) in aunt.iter() {
            if k == "cats" || k == "trees" {
                if *v <= reading[k] { matches = false; }
            } else if k == "pomeranians" || k == "goldfish" {
                if *v >= reading[k] { matches = false; }
            } else if *v != reading[k] {
                matches = false;
            }
        }
        if matches { return i + 1; }
    }
    panic!("No aunt matched");
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
