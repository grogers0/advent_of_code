use std::collections::BTreeMap;
use std::io::{self, Read};

use lazy_static::lazy_static;
use regex::Regex;

fn parse_scanner_ranges(input: &str) -> BTreeMap<usize, usize> {
    lazy_static!{
        static ref RE: Regex = Regex::new("^(\\d+): (\\d+)$").unwrap();
    }
    input.lines()
        .map(|line| {
            let cap = RE.captures(line).unwrap();
            (cap[1].parse().unwrap(), cap[2].parse().unwrap())
        })
        .collect()
}

fn update_scanner_positions(scanner_positions: &mut BTreeMap<usize, (usize, isize)>,
                            scanner_ranges: &BTreeMap<usize, usize>) {
    scanner_positions.iter_mut()
        .for_each(|(scanner, (position, direction))| {
            let range = scanner_ranges[scanner];
            if range == 1 {
                // Always at position 0
            } else if *position == 0 {
                *position = 1;
                *direction = 1;
            } else if *position == range - 1 {
                *position = range - 2;
                *direction = -1;
            } else {
                *position = (*position as isize + *direction) as usize;
            }
        });
}

fn part1(input: &str) -> usize {
    let scanner_ranges = parse_scanner_ranges(input);
    let mut scanner_positions: BTreeMap<usize, (usize, isize)> = scanner_ranges.keys().map(|scanner| (*scanner, (0, 1))).collect();
    let mut severity = 0;

    for pos in 0 ..= *scanner_ranges.keys().max().unwrap() {
        if scanner_positions.get(&pos).map(|p| p.0 == 0).unwrap_or(false) {
            severity += pos * scanner_ranges[&pos];
        }

        update_scanner_positions(&mut scanner_positions, &scanner_ranges);
    }

    severity
}

fn part2(input: &str) -> usize {
    let scanner_ranges = parse_scanner_ranges(input);
    let mut scanner_positions: BTreeMap<usize, (usize, isize)> = scanner_ranges.keys().map(|scanner| (*scanner, (0, 1))).collect();
    let mut my_positions = BTreeMap::new();
    let max_pos = *scanner_ranges.keys().max().unwrap();

    for time in 0.. {
        for (_, pos) in my_positions.iter_mut() {
            *pos += 1;
        }
        my_positions.insert(time, 0);

        for start in my_positions.keys().map(|x| *x).collect::<Vec<usize>>() {
            if scanner_positions.get(&my_positions[&start]).map(|p| p.0 == 0).unwrap_or(false) {
                my_positions.remove(&start);
            } else if my_positions[&start] > max_pos {
                return start
            }
        }

        update_scanner_positions(&mut scanner_positions, &scanner_ranges);
    }
    unreachable!()
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
0: 3
1: 2
4: 4
6: 4";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), 24);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EX), 10);
    }

}
