use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::io::{self, Read};

use regex::Regex;

fn parse(input: &str) -> (BTreeMap<String, Vec<String>>, String) {
    let re = Regex::new("^([A-Za-z]+) => ([A-Za-z]+)$").unwrap();
    let mut replacements: BTreeMap<_, Vec<_>> = BTreeMap::new();
    let mut molecule = String::new();
    for line in input.lines() {
        if line == "" {
            // Skip
        } else if let Some(cap) = re.captures(line) {
            replacements.entry(cap[1].to_string())
                .and_modify(|strs| strs.push(cap[2].to_string()))
                .or_insert(vec![cap[2].to_string()]);
        } else {
            molecule = line.to_string();
        }
    }
    (replacements, molecule)
}

fn possible_replacements(replacements: &BTreeMap<String, Vec<String>>, molecule: &str) -> BTreeSet<String> {
    let mut ret = BTreeSet::new();
    for (start_str, end_strs) in replacements.iter() {
        for (i, _) in molecule.match_indices(start_str) {
            for end_str in end_strs.iter() {
                ret.insert(format!("{}{}{}", &molecule[..i], end_str, &molecule[i+start_str.len()..]));
            }
        }
    }
    ret
}

// FIXME - takes too long, maybe A* or look at the input for some structure?
fn steps_to_create_molecule(replacements: &BTreeMap<String, Vec<String>>, molecule: &str) -> usize {
    let mut queue = VecDeque::new();
    let mut seen = BTreeSet::new();
    queue.push_back(("e".to_string(), 0));
    while let Some((start, steps)) = queue.pop_front() {
        if !seen.insert(start.clone()) { continue }
        if start == molecule { return steps }
        for end in possible_replacements(replacements, &start).into_iter() {
            queue.push_back((end, steps + 1));
        }
    }
    unreachable!()
}

fn part1(input: &str) -> usize {
    let (replacements, molecule) = parse(input);
    possible_replacements(&replacements, &molecule).len()
}

fn part2(input: &str) -> usize {
    let (replacements, molecule) = parse(input);
    steps_to_create_molecule(&replacements, &molecule)
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

    #[test]
    fn test_part1() {
        let (replacements, _) = parse("\
H => HO
H => OH
O => HH");
        assert_eq!(possible_replacements(&replacements, "HOH").len(), 4);
        assert_eq!(possible_replacements(&replacements, "HOHOHO").len(), 7);
    }

    #[test]
    fn test_part2() {
        let (replacements, _) = parse("\
e => H
e => O
H => HO
H => OH
O => HH");
        assert_eq!(steps_to_create_molecule(&replacements, "HOH"), 3);
        assert_eq!(steps_to_create_molecule(&replacements, "HOHOHO"), 6);
    }
}
