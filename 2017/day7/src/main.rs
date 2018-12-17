use std::collections::{BTreeMap, BTreeSet};
use std::io::{self, Read};

use lazy_static::lazy_static;
use regex::Regex;

struct Tower {
    name: String,
    weight: usize,
    children: Vec<Box<Tower>>
}

fn build_node(programs: &BTreeMap<String, (usize, Vec<String>)>, name: &str) -> Tower {
    Tower {
        name: name.to_string(),
        weight: programs[name].0,
        children: programs[name].1.iter().map(|n| Box::new(build_node(programs, n))).collect()
    }
}

fn parse(input: &str) -> Tower {
    lazy_static! {
        static ref RE: Regex = Regex::new("^([a-z]+) \\(([0-9]+)\\)( -> (([a-z]+(, )?)+))?$").unwrap();
    }

    let programs: BTreeMap<String, (usize, Vec<String>)> = input.lines()
        .map(|line| {
            let cap = RE.captures(line).unwrap();
            (cap[1].to_string(),
                (cap[2].parse().unwrap(),
                cap.get(4).map(|children| children.as_str().split(", ").map(|n| n.to_string()).collect()).unwrap_or(Vec::new())))
        })
        .collect();
    let root_name = programs.keys().find(|name| {
            !programs.iter().any(|(_,(_,children))| children.contains(&name))
        }).unwrap();
    build_node(&programs, root_name)
}

fn part1(input: &str) -> String {
    let tower = parse(input);
    tower.name
}

fn total_weight(tower: &Tower) -> usize {
    let a: usize = tower.weight;
    let b: usize = tower.children.iter().map(|child| total_weight(child)).sum();
    a + b
}

fn find_correct_weight(weights: &Vec<usize>) -> usize {
    for w in weights.iter() {
        if weights.iter().filter(|w2| *w2 == w).count() > 1 {
            return *w
        }
    }
    unreachable!();
}

fn are_children_unbalanced(tower: &Tower) -> bool {
    tower.children.iter().map(|child| total_weight(child)).collect::<BTreeSet<usize>>().len() > 1
}

fn find_unbalanced(tower: &Tower) -> usize {
    let child_weights: Vec<usize> = tower.children.iter().map(|child| total_weight(child)).collect();
    if child_weights.is_empty() { panic!() }

    let correct_weight = find_correct_weight(&child_weights);
    let ref wrong_child = tower.children.iter().find(|child| total_weight(child) != correct_weight).unwrap();
    let offset_weight = correct_weight as isize - total_weight(wrong_child) as isize;

    if are_children_unbalanced(wrong_child) {
        find_unbalanced(wrong_child)
    } else {
        (wrong_child.weight as isize + offset_weight) as usize
    }
}

fn part2(input: &str) -> usize {
    let tower = parse(input);
    find_unbalanced(&tower)
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
pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), "tknk".to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EX), 60);
    }

}
