use std::collections::HashSet;
use std::io::{self, Read};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn new(x: isize, y: isize) -> Pos {
        Pos { x, y }
    }
}

#[derive(Copy, Clone)]
enum Dir {
    North, South, East, West
}

struct Map {
    width: usize,
    height: usize,
    start: Pos,
    rocks: Vec<bool>,
}

fn positive_modulus(n: isize, d: usize) -> usize {
    ((n % d as isize) + d as isize) as usize % d
}

impl Map {
    fn idx(&self, pos: Pos) -> usize {
        self.width * positive_modulus(pos.y, self.height) +
            positive_modulus(pos.x, self.width)
    }

    fn is_rock(&self, pos: Pos) -> bool {
        self.rocks[self.idx(pos)]
    }

    fn step(&self, pos: Pos, dir: Dir, is_infinite: bool) -> Option<Pos> {
        let in_bounds = is_infinite || match dir {
            Dir::North => pos.y > 0,
            Dir::South => pos.y < self.height as isize - 1,
            Dir::West => pos.x > 0,
            Dir::East => pos.x < self.width as isize - 1,
        };
        if !in_bounds { return None }
        let next_pos = match dir {
            Dir::North => Pos::new(pos.x, pos.y - 1),
            Dir::South => Pos::new(pos.x, pos.y + 1),
            Dir::West => Pos::new(pos.x - 1, pos.y),
            Dir::East => Pos::new(pos.x + 1, pos.y),
        };
        if self.is_rock(next_pos) {
            None
        } else {
            Some(next_pos)
        }
    }
}

fn parse(puzzle_input: &str) -> Map {
    let height = puzzle_input.lines().count();
    let width = puzzle_input.lines().next().unwrap().chars().count();
    let mut start = None;
    let mut rocks = Vec::with_capacity(width * height);
    for (y, line) in puzzle_input.lines().enumerate() {
        assert_eq!(width, line.chars().count());
        for (x, ch) in line.chars().enumerate() {
            let is_rock = match ch {
                '#' => true,
                '.' => false,
                'S' => {
                    assert!(start.is_none());
                    start = Some(Pos::new(x as isize, y as isize));
                    false
                },
                _ => panic!(),
            };
            rocks.push(is_rock);
        }
    }
    Map { width, height, start: start.unwrap(), rocks }
}

fn steps_can_reach(map: &Map, steps: usize, is_infinite: bool) -> usize {
    let mut positions = HashSet::new();
    positions.insert(map.start);

    for _ in 0..steps {
        let mut next_positions = HashSet::new();
        for pos in positions {
            for dir in [Dir::North, Dir::South, Dir::East, Dir::West] {
                if let Some(next_pos) = map.step(pos, dir, is_infinite) {
                    next_positions.insert(next_pos);
                }
            }

        }
        positions = next_positions;
    }

    positions.len()
}

fn part1(map: &Map) -> usize {
    steps_can_reach(map, 64, false)
}

fn part2(map: &Map) -> usize {
    steps_can_reach(map, 1 /* FIXME */, true)
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

    const EX: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn test_part1() {
        assert_eq!(steps_can_reach(&parse(EX), 6, false), 16);
    }

    #[test]
    fn test_part2() {
        assert_eq!(steps_can_reach(&parse(EX), 6, true), 16);
        assert_eq!(steps_can_reach(&parse(EX), 10, true), 50);
        assert_eq!(steps_can_reach(&parse(EX), 50, true), 1594);
        assert_eq!(steps_can_reach(&parse(EX), 100, true), 6536);
        assert_eq!(steps_can_reach(&parse(EX), 500, true), 167004);
        //assert_eq!(steps_can_reach(&parse(EX), 1000, true), 668697);
        //assert_eq!(steps_can_reach(&parse(EX), 5000, true), 16733044);
    }
}
