use std::fmt;
use std::io::{self, Read};
use std::ops::Add;

use lazy_static::lazy_static;

#[derive(Copy, Clone)]
enum Dir {
    Left, Right
}

impl Dir {
    fn as_offset(&self) -> Offset {
        match self {
            Dir::Left => Offset::new(-1, 0),
            Dir::Right => Offset::new(1, 0),
        }
    }
}

#[derive(Copy, Clone)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Pos {
        Pos { x, y }
    }
}

#[derive(Copy, Clone)]
struct Offset {
    x: isize,
    y: isize,
}

impl Offset {
    const DOWN: Offset = Offset { x: 0, y: -1 };
    fn new(x: isize, y: isize) -> Offset {
        Offset { x, y }
    }
}

impl Add<Offset> for Pos {
    type Output = Pos;
    fn add(self, offset: Offset) -> Pos {
        let x = (self.x as isize) + offset.x;
        let y = (self.y as isize) + offset.y;
        assert!(x >= 0 && y >= 0);
        Pos::new(x as usize, y as usize)
    }
}

impl Add<Offset> for Offset {
    type Output = Offset;
    fn add(self, other: Offset) -> Offset {
        Offset::new(self.x + other.x, self.y + other.y)
    }
}

struct Wind<'a> {
    dirs: &'a [Dir],
    idx: usize,
}

impl <'a> Wind<'a> {
    fn new(dirs: &'a [Dir]) -> Self {
        Self { dirs, idx: 0 }
    }

    fn next(&mut self) -> Dir {
        let ret = self.dirs[self.idx];
        self.idx += 1;
        self.idx %= self.dirs.len();
        ret
    }
}

struct Rock {
    elems: Vec<Pos>,
}

impl Rock {
    fn new(elems: Vec<Pos>) -> Rock {
        Rock { elems }
    }

    // Return true if the rock was pushed
    fn push(&mut self, offset: Offset, grid: &Grid) -> bool {
        if offset.x < 0 && self.elems.iter().any(|pos| (pos.x as isize) < -offset.x) {
            return false
        } else if offset.x > 0 && self.elems.iter().any(|pos| pos.x as isize + offset.x >= 7) {
            return false
        } else if offset.y < 0 && self.elems.iter().any(|pos| (pos.y as isize) < -offset.y) {
            return false
        } else if self.elems.iter().any(|pos| grid.contains(*pos + offset)) {
            return false
        }
        for pos in self.elems.iter_mut() {
            *pos = *pos + offset;
        }
        true
    }
}

lazy_static! {
    static ref ROCKS: Vec<Vec<Offset>> = vec![
        vec![
            Offset::new(0,0), Offset::new(1,0), Offset::new(2,0), Offset::new(3,0),
        ],
        vec![
                              Offset::new(1,2),
            Offset::new(0,1), Offset::new(1,1), Offset::new(2,1),
                              Offset::new(1,0), 
        ],
        vec![
                                                Offset::new(2,2),
                                                Offset::new(2,1),
            Offset::new(0,0), Offset::new(1,0), Offset::new(2,0),
        ],
        vec![
            Offset::new(0,3),
            Offset::new(0,2),
            Offset::new(0,1),
            Offset::new(0,0),
        ],
        vec![
            Offset::new(0,1), Offset::new(1,1),
            Offset::new(0,0), Offset::new(1,0),
        ],
    ];
}

struct RockSpawner {
    idx: usize,
}

impl RockSpawner {
    fn new() -> Self {
        Self { idx: 0 }
    }
    fn next(&mut self, grid: &Grid) -> Rock {
        let offsets = &ROCKS[self.idx];
        self.idx += 1;
        self.idx %= ROCKS.len();
        let spawn = Pos::new(2, 3 + grid.height()); 
        Rock::new(offsets.iter().map(|offset| spawn + *offset).collect())
    }
}

struct Grid {
    rocks: Vec<bool>,
}

impl Grid {
    fn new() -> Grid {
        Grid { rocks: Vec::new() }
    }

    fn height(&self) -> usize {
        self.rocks.len() / 7
    }

    fn contains(&self, pos: Pos) -> bool {
        if pos.y >= self.height() {
            return false
        }
        self.rocks[pos.y * 7 + pos.x]
    }

    fn insert(&mut self, rock: &Rock) {
        for pos in rock.elems.iter() {
            while self.rocks.len() / 7 <= pos.y {
                for _ in 0..7 { self.rocks.push(false); }
            }
            self.rocks[pos.y * 7 + pos.x] = true;
        }
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in (0..self.height()).rev() {
            write!(f, "|")?;
            for x in 0..7 {
                write!(f, "{}", if self.contains(Pos::new(x, y)) { '#' } else { '.' })?;
            }
            write!(f, "|\n")?;
        }
        write!(f, "+-------+\n")
    }
}


fn parse(puzzle_input: &str) -> Vec<Dir> {
    puzzle_input.trim_end().chars().map(|ch| {
        match ch {
            '<' => Dir::Left,
            '>' => Dir::Right,
            _ => panic!(),
        }
    }).collect()
}

fn part1(puzzle_input: &str) -> usize {
    let dirs = parse(puzzle_input);
    let mut wind = Wind::new(&dirs);
    let mut spawner = RockSpawner::new();
    let mut grid = Grid::new();
    for _ in 0..2022 {
        let mut rock = spawner.next(&grid);
        loop {
            rock.push(wind.next().as_offset(), &grid);
            if !rock.push(Offset::DOWN, &grid) {
                grid.insert(&rock);
                break;
            }
        }
    }
    grid.height()
}

fn part2(puzzle_input: &str) -> &str {
    "FIXME"
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

    const EX: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), 3068);
    }

    #[test]
    fn test_part2() {
        // FIXME
    }
}
