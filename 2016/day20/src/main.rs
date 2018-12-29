use std::io::{self, Read};
use std::ops::RangeInclusive;

fn parse(input: &str) -> Vec<RangeInclusive<u32>> {
    input.lines()
        .map(|line| {
            let mut sp_iter = line.split('-');
            sp_iter.next().unwrap().parse().unwrap() ..= sp_iter.next().unwrap().parse().unwrap()
        })
        .collect()
}

fn contains(elem: u32, range: &RangeInclusive<u32>) -> bool {
    elem >= *range.start() && elem <= *range.end()
}

// OK, this and part2 are O(n^2) but the input size is small enough that it runs instantly. To
// scale better we'd need to use an interval tree.
fn part1(input: &str) -> u32 {
    let blocked_ranges = parse(input);
    let mut cur = 0;
    'outer: loop {
        for range in blocked_ranges.iter() {
            if contains(cur, range) {
                cur = range.end() + 1;
                continue 'outer;
            }
        }
        break;
    }
    cur
}

fn part2(input: &str) -> u32 {
    let blocked_ranges = parse(input);
    let mut allowed = 0;
    let mut cur = 0;
    'outer: loop {
        for range in blocked_ranges.iter() {
            if contains(cur, range) {
                if *range.end() == std::u32::MAX {
                    break 'outer;
                }
                cur = range.end() + 1;
                continue 'outer;
            }
        }

        let next_blocked_opt = blocked_ranges.iter()
            .map(|range| *range.start())
            .filter(|start| *start > cur)
            .min();
        if let Some(next) = next_blocked_opt {
            allowed += next - cur;
            cur = next;
        } else {
            allowed += std::u32::MAX - cur + 1;
            break;
        }
    }
    allowed
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
5-8
0-2
4-7";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), 3);
    }
}
