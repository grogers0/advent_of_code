use std::io::{self, Read};

use lazy_static::lazy_static;
use regex::Regex;

const MUL_A: u64 = 16807;
const MUL_B: u64 = 48271;
const DIV: u64 = 2147483647;

fn parse(input: &str) -> (u64, u64) {
    lazy_static!{
        static ref RE: Regex = Regex::new("starts with (\\d+)$").unwrap();
    }
    let start_state: Vec<u64> = input.lines()
        .map(|line| {
            let cap = RE.captures(line).unwrap();
            cap[1].parse().unwrap()
        })
        .collect();
    (start_state[0], start_state[1])
}

fn calc_part1(mut a: u64, mut b: u64) -> usize {
    let mut count = 0;
    for _ in 0..40000000 {
        a = (a * MUL_A) % DIV;
        b = (b * MUL_B) % DIV;

        if (a & 0xffff) == (b & 0xffff) {
            count += 1;
        }
    }
    count
}

fn calc_part2(mut a: u64, mut b: u64) -> usize {
    let mut count = 0;
    for _ in 0..5000000 {
        loop {
            a = (a * MUL_A) % DIV;
            if a % 4 == 0 { break }
        }
        loop {
            b = (b * MUL_B) % DIV;
            if b % 8 == 0 { break }
        }

        if (a & 0xffff) == (b & 0xffff) {
            count += 1;
        }
    }
    count
}

fn part1(input: &str) -> usize {
    let (a, b) = parse(input);
    calc_part1(a, b)
}

fn part2(input: &str) -> usize {
    let (a, b) = parse(input);
    calc_part2(a, b)
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
        assert_eq!(calc_part1(65, 8921), 588);
    }

    #[test]
    fn test_part2() {
        assert_eq!(calc_part2(65, 8921), 309);
    }

}
