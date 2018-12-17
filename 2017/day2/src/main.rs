use std::cmp::{min, max};
use std::io::{self, Read};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static!{
    static ref WHITESPACE: Regex = Regex::new("\\s+").unwrap();
}


fn part1(input: &str) -> usize {
    input.lines()
        .map(|line| {
            let row: Vec<usize> = WHITESPACE.split(line).map(|n| n.parse().unwrap()).collect();
            row.iter().max().unwrap() - row.iter().min().unwrap()
        })
        .sum()
}

fn get_divisble_result(row: &Vec<usize>) -> usize {
    for i in 0..row.len() {
        for j in (i+1)..row.len() {
            let x = min(row[i], row[j]);
            let y = max(row[i], row[j]);

            if y == y / x * x {
                return y / x
            }
        }
    }
    unreachable!();
}

fn part2(input: &str) -> usize {
    input.lines()
        .map(|line| {
            let row = WHITESPACE.split(line).map(|n| n.parse().unwrap()).collect();
            get_divisble_result(&row)
        })
        .sum()
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
        let ex = "\
5 1 9 5
7 5 3
2 4 6 8";

        assert_eq!(part1(ex), 18);
    }

    #[test]
    fn test_part2() {
        let ex = "\
5 9 2 8
9 4 7 3
3 8 6 5";
        assert_eq!(part2(ex), 9);
    }

}
