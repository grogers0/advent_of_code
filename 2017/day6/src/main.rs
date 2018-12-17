use std::collections::BTreeSet;
use std::io::{self, Read};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static!{
    static ref WHITESPACE: Regex = Regex::new("\\s+").unwrap();
}

fn redistribute(banks: &mut Vec<usize>) {
    let (mut offset, mut left) = banks.iter()
        .enumerate().map(|(i,cnt)| (i, *cnt))
        .max_by_key(|(i,cnt)| (*cnt, -(*i as isize))).unwrap();
    banks[offset] = 0;
    while left > 0 {
        left -= 1;
        offset = (offset + 1) % banks.len();
        banks[offset] += 1;
    }
}

fn redistribute_until_cycle(banks: &mut Vec<usize>) -> usize {
    let mut seen = BTreeSet::new();
    seen.insert(banks.clone());
    for steps in 1.. {
        redistribute(banks);
        if seen.contains(banks) {
            return steps
        }
        seen.insert(banks.clone());
    }
    unreachable!();
}

fn part1(input: &str) -> usize {
    let mut banks: Vec<usize> = WHITESPACE.split(input).map(|x| x.parse().unwrap()).collect();
    redistribute_until_cycle(&mut banks)
}

fn part2(input: &str) -> usize {
    let mut banks: Vec<usize> = WHITESPACE.split(input).map(|x| x.parse().unwrap()).collect();
    redistribute_until_cycle(&mut banks);
    redistribute_until_cycle(&mut banks)
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

    const EX: &str = "0 2 7 0";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EX), 4);
    }

}
