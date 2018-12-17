use std::collections::BTreeSet;
use std::io::{self, Read};
use std::mem;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Copy, Clone, Debug)]
enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char)
}

fn parse(input: &str) -> Vec<Move> {
    lazy_static!{
        static ref SPIN_RE: Regex = Regex::new("^s(\\d+)$").unwrap();
        static ref EXCHANGE_RE: Regex = Regex::new("^x(\\d+)/(\\d+)$").unwrap();
        static ref PARTNER_RE: Regex = Regex::new("^p([a-z])/([a-z])$").unwrap();
    }
    input.trim_end().split(",")
        .map(|line| {
            if let Some(cap) = SPIN_RE.captures(line) {
                Move::Spin(cap[1].parse::<usize>().unwrap())
            } else if let Some(cap) = EXCHANGE_RE.captures(line) {
                Move::Exchange(cap[1].parse::<usize>().unwrap(), cap[2].parse::<usize>().unwrap())
            } else if let Some(cap) = PARTNER_RE.captures(line) {
                Move::Partner(cap[1].chars().next().unwrap(), cap[2].chars().next().unwrap())
            } else {
                unreachable!()
            }
        })
        .collect()
}

fn build_dancers(n: usize) -> Vec<char> {
    let mut dancers = Vec::new();
    for i in 0..n {
        dancers.push(('a' as u8 + i as u8) as char);
    }
    dancers
}

fn exchange(dancers: &mut Vec<char>, i: usize, j: usize) {
    let tmp = dancers[i];
    dancers[i] = dancers[j];
    dancers[j] = tmp;
}

fn spin(dancers: &mut Vec<char>, n: usize) {
    for _ in 0..n {
        let val = dancers.remove(dancers.len() - 1);
        dancers.insert(0, val);
    }
}

fn partner(dancers: &mut Vec<char>, a: char, b: char) {
    let i = dancers.iter().enumerate().find(|(_,x)| **x == a).unwrap().0;
    let j = dancers.iter().enumerate().find(|(_,x)| **x == b).unwrap().0;
    exchange(dancers, i, j);
}

fn dance(dancers: &mut Vec<char>, moves: &Vec<Move>) {
    for m in moves {
        match m {
            Move::Spin(x) => spin(dancers, *x),
            Move::Exchange(i, j) => exchange(dancers, *i, *j),
            Move::Partner(a, b) => partner(dancers, *a, *b)
        }
    }
}

fn calc_part1(input: &str, n: usize) -> String {
    let moves = parse(input);
    let mut dancers = build_dancers(n);
    dance(&mut dancers, &moves);

    let mut out = String::new();
    for ch in dancers {
        out.push(ch);
    }
    out
}

fn part1(input: &str) -> String {
    calc_part1(input, 16)
}

fn part2(input: &str) -> String {
    let moves = parse(input);
    let mut dancers = build_dancers(16);

    let mut i = 0;
    let mut seen = BTreeSet::new();
    while !seen.contains(&dancers) {
        i += 1;
        seen.insert(dancers.clone());

        dance(&mut dancers, &moves);
    }
    mem::drop(seen);

    let mut cycle_len = 0;
    let saved = dancers.clone();
    while {
        i += 1;
        cycle_len += 1;
        dance(&mut dancers, &moves);

        saved != dancers
    } /*do*/ { } 
    mem::drop(saved);

    let remaining = (1000000000 - i) % cycle_len;
    for _ in 0..remaining {
        dance(&mut dancers, &moves);
    }

    let mut out = String::new();
    for ch in dancers {
        out.push(ch);
    }
    out
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

    const EX: &str = "s1,x3/4,pe/b";

    #[test]
    fn test_part1() {
        assert_eq!(calc_part1(EX, 5), "baedc".to_string());
    }
}
