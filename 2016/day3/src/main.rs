use std::io::{self, Read};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static!{
    static ref WHITESPACE: Regex = Regex::new("\\s+").unwrap();
}

fn possible_triangle(mut sides: [u32; 3]) -> bool {
    sides.sort();
    sides[0] + sides[1] > sides[2]
}

fn part1(input: &str) -> usize {
    input.lines()
        .map(|line| {
            let (a, b, c) = WHITESPACE.split(line).skip(1).map(|x| x.parse().unwrap()).tuples().next().unwrap();
            [a, b, c]
        })
        .filter(|triangle| possible_triangle(*triangle))
        .count()
}

fn part2(input: &str) -> usize {
    input.lines()
        .map(|line| WHITESPACE.split(line).skip(1).map(|x| x.parse().unwrap()).tuples().next().unwrap())
        .tuples()
        .map(|((a1, b1, c1), (a2, b2, c2), (a3, b3, c3))| [[a1, a2, a3], [b1, b2, b3], [c1, c2, c3]])
        .collect::<Vec<[[u32; 3]; 3]>>()
        .iter()
        .flat_map(|x| x)
        .filter(|triangle| possible_triangle(**triangle))
        .count()

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
        assert_eq!(possible_triangle([5, 10, 25]), false);
        assert_eq!(possible_triangle([5, 10, 10]), true);
    }
}
