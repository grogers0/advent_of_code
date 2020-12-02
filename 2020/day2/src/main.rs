use std::io::{self, Read};

use lazy_static::lazy_static;
use regex::Regex;

struct Policy(usize, usize, char);

fn parse(puzzle_input: &str) -> Vec<(Policy, String)> {
    lazy_static! {
        static ref RE: Regex = Regex::new("(\\d+)-(\\d+) ([a-z]): ([a-z]+)").unwrap();
    }
    puzzle_input.lines().map(|line| {
        let cap = RE.captures(line).unwrap();
        let policy = Policy(
            cap[1].parse().unwrap(),
            cap[2].parse().unwrap(),
            cap[3].chars().next().unwrap());
        let password = cap[4].to_string();
        (policy, password)
    }).collect()
}

fn part1(password_entries: &Vec<(Policy, String)>) -> usize {
    fn is_valid((policy, password): &(Policy, String)) -> bool {
        let cnt = password.chars().filter(|&ch| ch == policy.2).count();
        cnt >= policy.0 && cnt <= policy.1
    }
    password_entries.iter().filter(|entry| is_valid(entry)).count()
}

fn part2(password_entries: &Vec<(Policy, String)>) -> usize {
    fn valid_at_pos(pos: usize, letter: char, password: &str) -> bool {
        password.chars().nth(pos - 1).unwrap() == letter
    }

    fn is_valid((policy, password): &(Policy, String)) -> bool {
        let v1 = valid_at_pos(policy.0, policy.2, password);
        let v2 = valid_at_pos(policy.1, policy.2, password);
        v1 != v2
    }
    password_entries.iter().filter(|entry| is_valid(entry)).count()
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();
    let password_entries = parse(&puzzle_input);

    println!("{}", part1(&password_entries));
    println!("{}", part2(&password_entries));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    #[test]
    fn test_part1() {
        assert_eq!(2, part1(&parse(EX)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1, part2(&parse(EX)));
    }
}
