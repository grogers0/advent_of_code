use std::cmp::max;
use std::io::{self, Read};

use lazy_static::lazy_static;
use regex::Regex;

struct Claim {
    id: usize,
    x: usize,
    y: usize,
    width: usize,
    height: usize
}

fn parse(input: &str) -> Vec<Claim> {
    input.lines()
        .map(|line| {
            lazy_static! {
                // E.g. #1 @ 1,3: 4x4
                static ref RE: Regex = Regex::new("^#(\\d+) @ (\\d+),(\\d+): (\\d+)x(\\d+)$").unwrap();
            }
            let caps = RE.captures(line).unwrap();
            Claim {
                id: caps[1].parse().unwrap(),
                x: caps[2].parse().unwrap(),
                y: caps[3].parse().unwrap(),
                width: caps[4].parse().unwrap(),
                height: caps[5].parse().unwrap()
            }
        })
        .collect()
}

fn make_fabric(claims: &Vec<Claim>) -> (Vec<usize>, usize) {
    let fabric_size = claims.iter()
        .map(|c| max(c.x + c.width, c.y + c.height))
        .max().unwrap();
    (vec![0usize; fabric_size*fabric_size], fabric_size)
}

fn fill_claims(fabric: &mut Vec<usize>, fabric_size: usize, claims: &Vec<Claim>) {
    for claim in claims {
        for y in claim.y .. (claim.y + claim.height) {
            for x in claim.x .. (claim.x + claim.width) {
                fabric[y * fabric_size + x] += 1;
            }
        }
    }
}

fn part1(input: &str) -> usize {
    let claims = parse(input);
    let (mut fabric, fabric_size) = make_fabric(&claims);
    fill_claims(&mut fabric, fabric_size, &claims);
    fabric.iter()
        .filter(|sq| **sq > 1)
        .count()
}

fn is_overlapping(fabric: &Vec<usize>, fabric_size: usize, claim: &Claim) -> bool {
    for y in claim.y .. (claim.y + claim.height) {
        for x in claim.x .. (claim.x + claim.width) {
            if fabric[y * fabric_size + x] != 1 {
                return false
            }
        }
    }
    return true
}

fn part2(input: &str) -> usize {
    let claims = parse(input);
    let (mut fabric, fabric_size) = make_fabric(&claims);
    fill_claims(&mut fabric, fabric_size, &claims);
    claims.iter()
        .filter(|claim| is_overlapping(&fabric, fabric_size, claim))
        .next().unwrap().id
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
#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), 4);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EX), 3);
    }

}
