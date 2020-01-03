use std::collections::{HashSet, VecDeque};
use std::fmt;
use std::io::{self, Read};
use std::ops::Range;

use bit_vec::BitVec;

const WIDTH: usize = 5;
const HEIGHT: usize = 5;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Map(BitVec);

impl Map {
    fn parse(puzzle_input: &str) -> Self {
        let map = puzzle_input.trim().lines().flat_map(|line| {
            assert_eq!(WIDTH, line.chars().count());
            line.chars().map(|ch| {
                match ch {
                    '#' => true,
                    '.' => false,
                    _ => panic!()
                }
            })
        }).collect::<BitVec>();
        assert_eq!(WIDTH * HEIGHT, map.len());
        Self(map)
    }

    fn empty() -> Self {
        let mut map = BitVec::new();
        for _ in 0..HEIGHT*WIDTH {
            map.push(false)
        }
        Map(map)
    }

    fn bug_at(&self, x: usize, y: usize) -> bool {
        self.0[WIDTH * y + x]
    }

    fn adjacent_bugs(&self, x: usize, y: usize) -> usize {
        let mut ret = 0;
        if x > 0          && self.bug_at(x - 1, y) { ret += 1 }
        if x < WIDTH - 1  && self.bug_at(x + 1, y) { ret += 1 }
        if y > 0          && self.bug_at(x, y - 1) { ret += 1 }
        if y < HEIGHT - 1 && self.bug_at(x, y + 1) { ret += 1 }
        ret
    }

    fn step(&self) -> Self {
        let mut map = BitVec::new();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let num_adjacent = self.adjacent_bugs(x, y);
                map.push(if self.bug_at(x, y) {
                    num_adjacent == 1
                } else {
                    num_adjacent == 1 || num_adjacent == 2
                });
            }
        }
        Self(map)
    }

    fn first_repeated_step(mut self) -> Self {
        let mut seen = HashSet::new();
        while seen.insert(self.clone()) {
            self = self.step();
        }
        self
    }

    fn biodiversity(&self) -> u64 {
        let mut inc = 1;
        let mut sum = 0;
        for bug in self.0.iter() {
            if bug { sum += inc }
            inc *= 2;
        }
        sum
    }
}

#[allow(dead_code)]
impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                write!(f, "{}", if self.bug_at(x, y) { '#' } else { '.' })?
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[allow(dead_code)]
impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct RecursiveMap {
    offset: usize,
    data: VecDeque<Map>
}

impl RecursiveMap {
    fn parse(puzzle_input: &str) -> Self {
        let mut data = VecDeque::new();
        data.push_back(Map::parse(puzzle_input));
        Self { offset: 0, data: data }
    }

    fn bug_at(&self, x: usize, y: usize, depth: isize) -> bool {
        assert!(x != WIDTH/2 || y != HEIGHT/2);
        if !self.depths().contains(&depth) { return false }
        self.data[(depth + self.offset as isize) as usize].bug_at(x, y)
    }

    fn depths(&self) -> Range<isize> {
        -(self.offset as isize)..(self.data.len() - self.offset) as isize
    }

    fn adjacent_bugs(&self, x: usize, y: usize, depth: isize) -> usize {
        let mut ret = 0;
        if x == 0 {
            if self.bug_at(1, 2, depth - 1) { ret += 1 }
        } else if y != 2 || x == 1 || x == 4 {
            if self.bug_at(x - 1, y, depth) { ret += 1 }
        } else if x == 3 {
            for i in 0..WIDTH {
                if self.bug_at(4, i, depth + 1) { ret += 1 }
            }
        }

        if x == 4 {
            if self.bug_at(3, 2, depth - 1) { ret += 1 }
        } else if y != 2 || x == 3 || x == 0 {
            if self.bug_at(x + 1, y, depth) { ret += 1 }
        } else if x == 1 {
            for i in 0..WIDTH {
                if self.bug_at(0, i, depth + 1) { ret += 1 }
            }
        }

        if y == 0 {
            if self.bug_at(2, 1, depth - 1) { ret += 1 }
        } else if x != 2 || y == 1 || y == 4 {
            if self.bug_at(x, y - 1, depth) { ret += 1 }
        } else if y == 3 {
            for i in 0..WIDTH {
                if self.bug_at(i, 4, depth + 1) { ret += 1 }
            }
        }

        if y == 4 {
            if self.bug_at(2, 3, depth - 1) { ret += 1 }
        } else if x != 2 || y == 3 || y == 0 {
            if self.bug_at(x, y + 1, depth) { ret += 1 }
        } else if y == 1 {
            for i in 0..WIDTH {
                if self.bug_at(i, 0, depth + 1) { ret += 1 }
            }
        }

        ret
    }

    fn step(&self) -> Self {
        let mut map = self.clone();
        map.offset += 1;
        map.data.push_front(Map::empty());
        map.data.push_back(Map::empty());

        for depth in map.depths() {
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    if x == WIDTH/2 && y == HEIGHT/2 { continue }
                    let num_adjacent = self.adjacent_bugs(x, y, depth);
                    let val = if self.bug_at(x, y, depth) {
                        num_adjacent == 1
                    } else {
                        num_adjacent == 1 || num_adjacent == 2
                    };
                    map.data[(depth + map.offset as isize) as usize].0.set(WIDTH * y + x, val);
                }
            }
        }

        if map.data[map.data.len() - 1] == Map::empty() {
            map.data.pop_back();
        }
        if map.data[0] == Map::empty() {
            map.offset -= 1;
            map.data.pop_front();
        }

        map
    }

    fn count_bugs(&self) -> usize {
        self.data.iter().map(|map| {
            map.0.iter().filter(|&bug| bug).count()
        }).sum()
    }
}

#[allow(dead_code)]
impl fmt::Debug for RecursiveMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for depth in self.depths() {
            if depth != -(self.offset as isize) { writeln!(f, "")? }
            writeln!(f, "Depth {}:", depth)?;

            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    if x == WIDTH/2 && y == HEIGHT/2 {
                        write!(f, "?")?
                    } else {
                        write!(f, "{}", if self.bug_at(x, y, depth) { '#' } else { '.' })?
                    }
                }
                writeln!(f, "")?
            }
        }
        Ok(())
    }
}


fn part1(puzzle_input: &str) -> u64 {
    Map::parse(puzzle_input).first_repeated_step().biodiversity()
}

fn part2(puzzle_input: &str) -> usize {
    let mut map = RecursiveMap::parse(puzzle_input);
    for _ in 0..200 {
        map = map.step();
    }
    map.count_bugs()
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

    const EX: &str = "
....#
#..#.
#..##
..#..
#....";

    #[test]
    fn test_part1_step() {
        let mut map = Map::parse(EX);
        map = map.step();
        assert_eq!(map, Map::parse("
#..#.
####.
###.#
##.##
.##.."));

        map = map.step();
        assert_eq!(map, Map::parse("
#####
....#
....#
...#.
#.###"));

        map = map.step();
        assert_eq!(map, Map::parse("
#....
####.
...##
#.##.
.##.#"));

        map = map.step();
        assert_eq!(map, Map::parse("
####.
....#
##..#
.....
##..."));
    }

    #[test]
    fn test_part1_first_repeated() {
        assert_eq!(Map::parse(EX).first_repeated_step(), Map::parse("
.....
.....
.....
#....
.#..."));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), 2129920);
    }

    #[test]
    fn test_part2() {
        let mut map = RecursiveMap::parse(EX);
        for _ in 0..10 {
            map = map.step();
        }

        assert_eq!(format!("{:?}", map).trim(), "
Depth -5:
..#..
.#.#.
..?.#
.#.#.
..#..

Depth -4:
...#.
...##
..?..
...##
...#.

Depth -3:
#.#..
.#...
..?..
.#...
#.#..

Depth -2:
.#.##
....#
..?.#
...##
.###.

Depth -1:
#..##
...##
..?..
...#.
.####

Depth 0:
.#...
.#.##
.#?..
.....
.....

Depth 1:
.##..
#..##
..?.#
##.##
#####

Depth 2:
###..
##.#.
#.?..
.#.##
#.#..

Depth 3:
..###
.....
#.?..
#....
#...#

Depth 4:
.###.
#..#.
#.?..
##.#.
.....

Depth 5:
####.
#..#.
#.?#.
####.
.....".trim());

        assert_eq!(map.count_bugs(), 99);
    }
}
