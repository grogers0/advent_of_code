use std::collections::BTreeSet;
use std::io::{self, Read};

use lazy_static::lazy_static;
use regex::Regex;

fn parse(input: &str) -> Vec<(usize, usize)> {
    lazy_static!{
        static ref RE: Regex = Regex::new("^(\\d+)/(\\d+)$").unwrap();
    }

    input.lines()
        .map(|line| {
            let cap = RE.captures(line).unwrap();
            (cap[1].parse().unwrap(), cap[2].parse().unwrap())
        })
        .collect()
}

fn all_bridges(pieces: &Vec<(usize, usize)>, curr: usize, used: BTreeSet<usize>) -> Vec<Vec<(usize, usize)>> {
    let mut ret = Vec::new();
    for (i, piece) in pieces.iter().enumerate() {
        if used.contains(&i) { continue }
        if piece.0 == curr {
            let mut used2 = used.clone();
            used2.insert(i);
            ret.push(used2.iter().map(|j| pieces[*j]).collect());
            ret.append(&mut all_bridges(pieces, piece.1, used2));
        } else if piece.1 == curr {
            let mut used2 = used.clone();
            used2.insert(i);
            ret.push(used2.iter().map(|j| pieces[*j]).collect());
            ret.append(&mut all_bridges(pieces, piece.0, used2));
        }
    }
    ret
}

fn part1(input: &str) -> usize {
    let pieces = parse(input);
    let bridges = all_bridges(&pieces, 0, BTreeSet::new());
    bridges.iter()
        .map(|bridge| bridge.iter().map(|piece| piece.0 + piece.1).sum())
        .max()
        .unwrap()
}

fn part2(input: &str) -> usize {
    let pieces = parse(input);
    let bridges = all_bridges(&pieces, 0, BTreeSet::new());
    let max_length = bridges.iter()
        .map(|bridge| bridge.len())
        .max()
        .unwrap();
    bridges.iter()
        .filter(|bridge| bridge.len() == max_length)
        .map(|bridge| bridge.iter().map(|piece| piece.0 + piece.1).sum())
        .max()
        .unwrap()
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
0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), 31);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EX), 19);
    }

}
