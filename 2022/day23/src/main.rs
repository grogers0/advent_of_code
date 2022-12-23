use std::collections::HashSet;
use std::io::{self, Read};
use std::ops::Add;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct Pos {
    x: isize,
    y: isize,
}

impl Add<Pos> for Pos {
    type Output = Pos;
    fn add(self, other: Pos) -> Pos {
        Pos { x: self.x + other.x, y: self.y + other.y }
    }
}

mod offsets {
    use super::Pos;
    pub const NW: Pos = Pos { x: -1, y: -1 };
    pub const N:  Pos = Pos { x:  0, y: -1 };
    pub const NE: Pos = Pos { x:  1, y: -1 };
    pub const W: Pos  = Pos { x: -1, y:  0 };
    pub const E: Pos  = Pos { x:  1, y:  0 };
    pub const SW: Pos = Pos { x: -1, y:  1 };
    pub const S: Pos  = Pos { x:  0, y:  1 };
    pub const SE: Pos = Pos { x:  1, y:  1 };
    pub const ALL: [Pos; 8] = [NW, N, NE, W, E, SW, S, SE];
    pub const ALL_NORTH: [Pos; 3] = [NW, N, NE];
    pub const ALL_SOUTH: [Pos; 3] = [SW, S, SE];
    pub const ALL_WEST:  [Pos; 3] = [NW, W, SW];
    pub const ALL_EAST:  [Pos; 3] = [NE, E, SE];
}

#[derive(Copy, Clone)]
enum Dir {
    North, South, West, East
}

fn parse(puzzle_input: &str) -> HashSet<Pos> {
    let mut elves = HashSet::new();
    for (y, line) in puzzle_input.trim_end().lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '#' => {
                    elves.insert(Pos { x: x as isize, y: y as isize });
                },
                '.' => (),
                _ => panic!(),
            }
        }
    }
    elves
}

fn propose(elf: Pos, elves: &HashSet<Pos>, dirs: &[Dir]) -> Pos {
    if offsets::ALL.iter().all(|pos| !elves.contains(&(elf + *pos))) {
        return elf;
    }
    for dir in dirs {
        let offsets = match dir {
            Dir::North => offsets::ALL_NORTH,
            Dir::South => offsets::ALL_SOUTH,
            Dir::West  => offsets::ALL_WEST,
            Dir::East  => offsets::ALL_EAST,
        };
        if offsets.iter().all(|pos| !elves.contains(&(elf + *pos))) {
            let offset = match dir {
                Dir::North => offsets::N,
                Dir::South => offsets::S,
                Dir::West  => offsets::W,
                Dir::East  => offsets::E,
            };
            return elf + offset;
        }
    }
    return elf;
}

fn step(elves: &mut HashSet<Pos>, dirs: &[Dir]) -> bool {
    let mut proposed = HashSet::with_capacity(elves.len());
    let mut overlapping = HashSet::with_capacity(elves.len());

    for elf in elves.iter() {
        let pos = propose(*elf, elves, dirs);
        if !proposed.insert(pos) {
            overlapping.insert(pos);
        }
    }

    let mut updated_elves = HashSet::with_capacity(elves.len());
    for elf in elves.iter() {
        let mut pos = propose(*elf, elves, dirs);
        if overlapping.contains(&pos) {
            pos = *elf;
        }
        assert!(updated_elves.insert(pos));
    }

    let ret = elves != &updated_elves;
    *elves = updated_elves;
    ret
}

fn count_empty(elves: &HashSet<Pos>) -> usize {
    let xmin = elves.iter().map(|pos| pos.x).min().unwrap();
    let xmax = elves.iter().map(|pos| pos.x).max().unwrap();
    let ymin = elves.iter().map(|pos| pos.y).min().unwrap();
    let ymax = elves.iter().map(|pos| pos.y).max().unwrap();
    let total = (xmax - xmin + 1) * (ymax - ymin + 1);
    total as usize - elves.len()
}

fn part1(puzzle_input: &str) -> usize {
    let mut elves = parse(puzzle_input);
    let mut dirs = vec![Dir::North, Dir::South, Dir::West, Dir::East];
    for _ in 0..10 {
        step(&mut elves, &dirs);
        let dir = dirs.remove(0);
        dirs.push(dir);
    }
    count_empty(&elves)
}

fn part2(puzzle_input: &str) -> usize {
    let mut elves = parse(puzzle_input);
    let mut dirs = vec![Dir::North, Dir::South, Dir::West, Dir::East];
    for round in 1.. {
        if !step(&mut elves, &dirs) {
            return round;
        }
        let dir = dirs.remove(0);
        dirs.push(dir);
    }
    unreachable!()
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

    const EX: &str = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), 110);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EX), 20);
    }
}
