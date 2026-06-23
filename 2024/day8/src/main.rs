use std::io::{self, Read};
use std::ops::Sub;
use std::collections::{HashSet, HashMap};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone)]
struct Delta {
    dx: isize,
    dy: isize,
}

impl Sub for Pos {
    type Output = Delta;
    fn sub(self, other: Pos) -> Delta {
        Delta {
            dx: self.x as isize - other.x as isize,
            dy: self.y as isize - other.y as isize,
        }
    }
}

impl Delta {
    fn scale_by(self, n: isize) -> Delta {
        Delta { dx: self.dx * n, dy: self.dy * n }
    }
}

struct Map {
    width: usize,
    height: usize,
    antennae: HashMap<char, Vec<Pos>>,
}

impl Map {
    fn checked_add(&self, pos: Pos, delta: Delta) -> Option<Pos> {
        let x = pos.x as isize + delta.dx;
        if x < 0 || x as usize >= self.width { return None; }

        let y = pos.y as isize + delta.dy;
        if y < 0 || y as usize >= self.height { return None; }

        Some(Pos { x: x as usize, y: y as usize })
    }
}


fn parse(puzzle_input: &str) -> Map {
    let height = puzzle_input.lines().count();
    let width = puzzle_input.lines().next().unwrap().chars().count();
    let mut antennae = HashMap::new();
    for (y, line) in puzzle_input.lines().enumerate() {
        assert_eq!(width, line.chars().count());
        for (x, ch) in line.chars().enumerate() {
            if ch == '.' { continue; }
            antennae.entry(ch).or_insert(vec![]).push(Pos { x, y })
        }
    }
    Map { width, height, antennae }
}

fn part1(map: &Map) -> usize {
    let mut antinodes = HashSet::new();
    for (_, positions) in &map.antennae {
        for i in 0..(positions.len() - 1) {
            for j in (i + 1)..positions.len() {
                let a = positions[i];
                let b = positions[j];
                if let Some(x) = map.checked_add(a, a - b) {
                    antinodes.insert(x);
                }
                if let Some(y) = map.checked_add(b, b - a) {
                    antinodes.insert(y);
                }
            }
        }
    }

    antinodes.len()
}

fn part2(map: &Map) -> usize {
    let mut antinodes = HashSet::new();
    for (_, positions) in &map.antennae {
        for &pos in positions {
            antinodes.insert(pos);
        }
        for i in 0..(positions.len() - 1) {
            for j in (i + 1)..positions.len() {
                let a = positions[i];
                let b = positions[j];
                for n in 1.. {
                    if let Some(x) = map.checked_add(a, (a - b).scale_by(n)) {
                        antinodes.insert(x);
                    } else {
                        break;
                    }
                }
                for n in 1.. {
                    if let Some(y) = map.checked_add(b, (b - a).scale_by(n)) {
                        antinodes.insert(y);
                    } else {
                        break;
                    }
                }
            }
        }
    }
    antinodes.len()
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let map = parse(&puzzle_input);
    println!("{}", part1(&map));
    println!("{}", part2(&map));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX)), 14);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EX)), 34);
    }
}
