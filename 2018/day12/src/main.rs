use std::collections::{BTreeSet};
use std::io::{self, Read};
use std::mem;

use lazy_static::lazy_static;
use regex::Regex;

// output is (plants, spread)
fn parse(input: &str) -> (BTreeSet<i32>, BTreeSet<u8>) {
    let mut plants = BTreeSet::new();
    let mut spread = BTreeSet::new();
    lazy_static!{
        static ref PLANTS_RE: Regex = Regex::new("^initial state: ([#\\.]+)$").unwrap();
        static ref SPREAD_RE: Regex = Regex::new("^([#\\.]{5}) => #$$").unwrap();
    }

    let mut lines_iter = input.lines();

    let cap = PLANTS_RE.captures(lines_iter.next().unwrap()).unwrap();
    for (i, ch) in cap[1].chars().enumerate() {
        match ch {
            '#' => { plants.insert(i as i32); },
            '.' => (),
            _ => unreachable!()
        }
    }

    for line in lines_iter {
        if let Some(cap) = SPREAD_RE.captures(line) {
            let mut config = 0u8;
            for ch in cap[1].chars() {
                config <<= 1;
                match ch {
                    '#' => config |= 1,
                    '.' => (),
                    _ => unreachable!()
                }
            }
            spread.insert(config);
        }
    }

    (plants, spread)
}

fn plant_next_gen(i: i32, plants: &BTreeSet<i32>, spread: &BTreeSet<u8>) -> bool {
    let config = if plants.contains(&(i - 2)) { 16 } else { 0 }
        | if plants.contains(&(i - 1)) { 8 } else { 0 }
        | if plants.contains(&i) { 4 } else { 0 }
        | if plants.contains(&(i + 1)) { 2 } else { 0 }
        | if plants.contains(&(i + 2)) { 1 } else { 0 };
    spread.contains(&config)
}

fn write_plants(plants: &BTreeSet<i32>) -> String {
    let min = *plants.iter().next().unwrap();
    let max = *plants.iter().rev().next().unwrap();
    let mut out = String::new();
    for i in min..=max {
        if plants.contains(&i) {
            out.push('#')
        } else {
            out.push('.')
        }
    }
    out
}

fn step(plants: BTreeSet<i32>, spread: &BTreeSet<u8>) -> BTreeSet<i32> {
    let min = plants.iter().next().unwrap() - 2;
    let max = plants.iter().rev().next().unwrap() + 2;
    let mut out = BTreeSet::new();
    for i in min..=max {
        if plant_next_gen(i, &plants, spread) {
            out.insert(i);
        }
    }
    out
}

fn part1(input: &str) -> i32 {
    let (mut plants, spread) = parse(input);
    for _ in 0..20 {
        plants = step(plants, &spread);
    }
    plants.iter().sum()
}

fn part2(input: &str) -> i64 {
    let (mut plants, spread) = parse(input);
    let mut seen = BTreeSet::new();
    let mut i: i64 = 0;
    while !seen.contains(&write_plants(&plants)) {
        i += 1;
        seen.insert(write_plants(&plants));

        plants = step(plants, &spread);
    }
    mem::drop(seen);

    let mut cycle_len: i64 = 0;
    let saved = plants.clone();
    while {
        i += 1;
        cycle_len += 1;
        plants = step(plants, &spread);

        write_plants(&saved) != write_plants(&plants)
    } /*do*/ {}
    let diff_per_cycle: i64 = (plants.iter().sum::<i32>() - saved.iter().sum::<i32>()) as i64;
    mem::drop(saved);

    let cycles_left = (50000000000 - i) / cycle_len;
    let remaining = (50000000000 - i) % cycle_len;
    for _ in 0..remaining {
        plants = step(plants, &spread);
    }

    plants.iter().sum::<i32>() as i64 + (diff_per_cycle * cycles_left)
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
initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), 325);
    }
}
