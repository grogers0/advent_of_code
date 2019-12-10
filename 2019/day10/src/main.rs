use std::collections::{BTreeSet, BTreeMap};
use std::cmp::Ordering;
use std::io::{self, Read};

use bit_vec::{BitVec, Iter as BitVecIter};

struct Map {
    width: usize,
    data: BitVec
}

struct AsteroidsIter<'a> {
    width: usize,
    index: usize,
    iter: BitVecIter<'a>
}

impl <'a> Iterator for AsteroidsIter<'a> {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                Some(true) => {
                    let ret = (self.index % self.width, self.index / self.width);
                    self.index += 1;
                    return Some(ret);
                },
                Some(false) => {
                    self.index += 1;
                },
                None => {
                    return None;
                }
            }
        }
    }
}

impl Map {
    fn asteroids<'a>(&'a self) -> AsteroidsIter<'a> {
        AsteroidsIter { width: self.width, index: 0, iter: self.data.iter() }
    }
}

fn parse(input: &str) -> Map {
    let input = input.trim();
    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();
    let mut data = BitVec::with_capacity(width * height);

    for line in input.lines() {
        assert_eq!(line.chars().count(), width);
        for ch in line.chars() {
            data.push(match ch {
                '#' => true,
                '.' => false,
                _ => panic!()
            });
        }
    }
    Map { width: width, data: data }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct RotDir { n: usize, d: usize, q: usize }

impl RotDir {
    fn gcd(mut a: usize, mut b: usize) -> usize {
        while a != b {
            if a > b {
                a = a - b;
            } else {
                b = b - a;
            }
        }
        a
    }

    fn new(origin: (usize, usize), other: (usize, usize)) -> Self {
        let right_hemi = other.0 >= origin.0;
        let bottom_hemi = other.1 >= origin.1;
        let dx = if right_hemi { other.0 - origin.0 } else { origin.0 - other.0 };
        let dy = if bottom_hemi { other.1 - origin.1 } else { origin.1 - other.1 };

        let gcd = if dx == 0 && dy == 0 { unreachable!() }
        else if dx == 0 { dy }
        else if dy == 0 { dx }
        else { Self::gcd(dx, dy) };

        let dx = dx / gcd;
        let dy = dy / gcd;

        if right_hemi && !bottom_hemi {
            Self { n: dx, d: dy, q: 0 }
        } else if right_hemi {
            Self { n: dy, d: dx, q: 1 }
        } else if bottom_hemi {
            Self { n: dx, d: dy, q: 2 }
        } else {
            Self { n: dy, d: dx, q: 3 }
        }
    }
}

impl Ord for RotDir {
    fn cmp(&self, other: &Self) -> Ordering {
        let cmp = self.q.cmp(&other.q);
        if cmp != Ordering::Equal {
            return cmp;
        }
        if self.d == 0 {
            if other.d == 0 {
                return Ordering::Equal;
            } else {
                return Ordering::Less;
            }
        } else if other.d == 0 {
            return Ordering::Greater;
        }
        let gcd = Self::gcd(self.d, other.d);
        let a = self.n * other.d / gcd;
        let b = other.n * self.d / gcd;
        a.cmp(&b)
    }
}

impl PartialOrd for RotDir {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn asteroids_detected(station_pos: (usize, usize), map: &Map) -> usize {
    // We can only see the first asteroid in any direction, the others are not detected
    let mut seen = BTreeSet::new();
    for asteroid_pos in map.asteroids() {
        if station_pos != asteroid_pos {
            seen.insert(RotDir::new(station_pos, asteroid_pos));
        }
    }
    seen.len()
}

fn station_position(map: &Map) -> (usize, usize) {
    map.asteroids().max_by_key(|pos| asteroids_detected(*pos, &map)).unwrap()
}

fn dist(pos1: (usize, usize), pos2: (usize, usize)) -> usize {
    let dx = if pos1.0 >= pos2.0 { pos1.0 - pos2.0 } else { pos2.0 - pos1.0 };
    let dy = if pos1.1 >= pos2.1 { pos1.1 - pos2.1 } else { pos2.1 - pos1.1 };
    dx + dy
}

fn shoot_lasers(station_pos: (usize, usize), map: &mut Map, mut shots: usize) -> (usize, usize) {
    loop {
        let mut seen = BTreeMap::new();
        for asteroid_pos in map.asteroids() {
            if station_pos != asteroid_pos {
                let dir = RotDir::new(station_pos, asteroid_pos);
                let other_pos = *seen.entry(dir).or_insert(asteroid_pos);
                if dist(station_pos, other_pos) < dist(station_pos, asteroid_pos) {
                    seen.insert(dir, other_pos);
                }
            }
        }

        if seen.is_empty() {
            panic!();
        }
        for (_, pos) in seen {
            shots -= 1;
            if shots == 0 {
                return pos;
            }
            map.data.set(map.width * pos.1 + pos.0, false);
        }
    }
}

fn part1(input: &str) -> usize {
    let map = parse(input);
    let station_pos = station_position(&map);
    asteroids_detected(station_pos, &map)
}

fn part2(input: &str) -> usize {
    let mut map = parse(input);
    let station_pos = station_position(&map);
    let last_zapped = shoot_lasers(station_pos, &mut map, 200);
    last_zapped.0 * 100 + last_zapped.1
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
        let ex = "
.#..#
.....
#####
....#
...##";
        assert_eq!(part1(ex), 8);

        let ex = "
......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";
        assert_eq!(part1(ex), 33);

        let ex = "
#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.";
        assert_eq!(part1(ex), 35);

        let ex = "
.#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..";
        assert_eq!(part1(ex), 41);

        let ex = "
.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";
        assert_eq!(part1(ex), 210);
    }

    #[test]
    fn test_part2() {
        let ex = "
.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";
        assert_eq!(part2(ex), 802);
    }
}
