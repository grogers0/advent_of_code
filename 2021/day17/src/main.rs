use std::io::{self, Read};
use std::ops::RangeInclusive;

use regex::Regex;
use lazy_static::lazy_static;

fn parse(puzzle_input: &str) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    lazy_static! {
        static ref RE: Regex = Regex::new("^target area: x=(\\d+)\\.\\.(\\d+), y=(-?\\d+)\\.\\.(-?\\d+)$").unwrap();
    }
    let cap = RE.captures(puzzle_input.trim_end()).unwrap();
    let x1 = cap[1].parse().unwrap();
    let x2 = cap[2].parse().unwrap();
    let y1 = cap[3].parse().unwrap();
    let y2 = cap[4].parse().unwrap();
    (x1..=x2, y1..=y2)
}

fn reaches_target_area(mut dx: i32, mut dy: i32, xtarget: &RangeInclusive<i32>, ytarget: &RangeInclusive<i32>) -> bool {
    let mut x = 0;
    let mut y = 0;
    while x <= *xtarget.end() && y >= *ytarget.start() {
        if xtarget.contains(&x) && ytarget.contains(&y) {
            return true;
        }
        x += dx;
        y += dy;
        if dx > 0 {
            dx -= 1;
        } else if dx < 0 {
            dx += 1;
        }
        dy -= 1;
    }
    false
}

fn target_reaching_initial_velocities(xtarget: RangeInclusive<i32>, ytarget: RangeInclusive<i32>) -> Vec<(i32, i32)> {
    let mut ret = Vec::new();
    for dx in 1 ..= *xtarget.end() {
        for dy in *ytarget.start() .. -*ytarget.start() {
            if reaches_target_area(dx, dy, &xtarget, &ytarget) {
                ret.push((dx, dy));
            }
        }
    }
    ret
}

fn part1(puzzle_input: &str) -> i32 {
    let (xtarget, ytarget) = parse(puzzle_input);
    target_reaching_initial_velocities(xtarget, ytarget).into_iter()
        .filter(|(_, dy)| *dy > 0)
        .map(|(_, dy)| dy * (dy + 1) / 2)
        .max().unwrap()
}

fn part2(puzzle_input: &str) -> usize {
    let (xtarget, ytarget) = parse(puzzle_input);
    target_reaching_initial_velocities(xtarget, ytarget).len()
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    println!("{}", part1(&puzzle_input));
    println!("{}", part2(&puzzle_input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn test_part1() {
        assert_eq!(45, part1(EX));
    }

    #[test]
    fn test_part2() {
        assert_eq!(112, part2(EX));
    }
}
