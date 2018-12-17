use std::collections::BTreeMap;
use std::io::{self, Read};
use std::ops::AddAssign;

use lazy_static::lazy_static;
use regex::Regex;

type POS = i16;

#[derive(Copy, Clone, Debug)]
enum Area {
    Finite(usize),
    Infinite
}

impl AddAssign for Area {
    fn add_assign(&mut self, other: Area) {
        *self = match (*self, other) {
            (Area::Infinite, _) => Area::Infinite,
            (_, Area::Infinite) => Area::Infinite,
            (Area::Finite(a), Area::Finite(b)) => Area::Finite(a + b)
        }
    }
}

fn parse(input: &str) -> Vec<(POS, POS)> {
    lazy_static! {
        static ref RE: Regex = Regex::new("^(\\d+), (\\d+)$").unwrap();
    }
    input.lines()
        .map(|line| {
            let cap = RE.captures(line).unwrap();
            (cap[1].parse().unwrap(), cap[2].parse().unwrap())
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let positions = parse(input);
    let min_x = positions.iter().map(|(x,_)| *x).min().unwrap();
    let max_x = positions.iter().map(|(x,_)| *x).max().unwrap();
    let min_y = positions.iter().map(|(_,y)| *y).min().unwrap();
    let max_y = positions.iter().map(|(_,y)| *y).max().unwrap();

    let mut areas = BTreeMap::new();
    for id in 0 ..= positions.len() {
        areas.insert(id, Area::Finite(0));
    }

    for y in min_y ..= max_y {
        for x in min_x ..= max_x {
            let mut distances: Vec<(usize,usize)> = positions.iter().enumerate()
                .map(|(id, (px, py))| (id, ((px - x).abs() + (py - y).abs()) as usize))
                .collect();
            distances.sort_by_key(|(_, d)| *d);
            if distances[0].1 < distances[1].1 {
                let id = distances[0].0;
                let to_add = if x == min_x || x == max_x || y == min_y || y == max_y {
                    Area::Infinite
                } else {
                    Area::Finite(1)
                };
                areas.entry(id).and_modify(|d| *d += to_add);
            }
        }
    }

    areas.values()
        .map(|area| match area {
                Area::Infinite => 0usize, // Ignore infinite areas
                Area::Finite(x) => *x
        })
        .max().unwrap()
}

fn calc_part2(positions: Vec<(POS, POS)>, max_distance: usize) -> usize {
    let min_x = positions.iter().map(|(x,_)| *x).min().unwrap();
    let max_x = positions.iter().map(|(x,_)| *x).max().unwrap();
    let min_y = positions.iter().map(|(_,y)| *y).min().unwrap();
    let max_y = positions.iter().map(|(_,y)| *y).max().unwrap();

    let mut cnt = 0;
    for y in min_y ..= max_y {
        for x in min_x ..= max_x {
            let total_distance: usize = positions.iter()
                .map(|(px, py)| ((px - x).abs() + (py - y).abs()) as usize) // Manhattan distance
                .sum();
            if total_distance < max_distance {
                cnt += 1;
            }
        }
    }

    cnt
}

fn part2(input: &str) -> usize {
    calc_part2(parse(input), 10000)
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
1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), 17);
    }

    #[test]
    fn test_part2() {
        assert_eq!(calc_part2(parse(EX), 32), 16);
    }

}
