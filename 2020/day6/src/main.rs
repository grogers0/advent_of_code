use std::collections::HashSet;
use std::io::{self, Read};

fn parse(puzzle_input: &str) -> Vec<Vec<HashSet<char>>> {
    puzzle_input.split("\n\n").map(|group| {
        group.lines().map(|person| {
            person.chars().collect()
        }).collect()
    }).collect()
}

fn part1(groups: &Vec<Vec<HashSet<char>>>) -> usize {
    groups.iter().map(|group| {
        let mut union = HashSet::<char>::new();
        for person in group {
            union.extend(person.iter());
        }
        union.len()
    }).sum()
}

fn part2(groups: &Vec<Vec<HashSet<char>>>) -> usize {
    groups.iter().map(|group| {
        let first = &group[0];
        let rest = &group[1..];
        first.iter().filter(|ch| rest.iter().all(|p| p.contains(ch))).count()
    }).sum()
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();
    let groups = parse(&puzzle_input);

    println!("{}", part1(&groups));
    println!("{}", part2(&groups));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn test_part1() {
        assert_eq!(11, part1(&parse(EX)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(6, part2(&parse(EX)));
    }
}
