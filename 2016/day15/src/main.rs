use std::io::{self, Read};

use regex::Regex;

struct Disc {
    start: usize,
    total: usize
}

fn parse(input: &str) -> Vec<Disc> {
    let re = Regex::new("^Disc #\\d+ has (\\d+) positions; at time=0, it is at position (\\d+).$").unwrap();
    input.lines()
        .map(|line| {
            let cap = re.captures(line).unwrap();
            Disc { start: cap[2].parse().unwrap(), total: cap[1].parse().unwrap() }
        })
        .collect()
}

fn passes_discs(start: usize, discs: &Vec<Disc>) -> bool {
    for (i, disc) in discs.iter().enumerate() {
        if (disc.start + start + i + 1) % disc.total != 0 {
            return false;
        }
    }
    true
}

fn first_time_to_pass_discs(discs: &Vec<Disc>) -> usize {
    for start in 0.. {
        if passes_discs(start, &discs) {
            return start;
        }
    }
    unreachable!()
}

fn part1(input: &str) -> usize {
    let discs = parse(input);
    first_time_to_pass_discs(&discs)
}

fn part2(input: &str) -> usize {
    let mut discs = parse(input);
    discs.push(Disc { start: 0, total: 11 });
    first_time_to_pass_discs(&discs)
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
Disc #1 has 5 positions; at time=0, it is at position 4.
Disc #2 has 2 positions; at time=0, it is at position 1.";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), 5);
    }
}
