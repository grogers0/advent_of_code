use std::collections::HashSet;
use std::io::{self, Read};
use std::{mem, ops, fmt};

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
enum Dir {
    Left = 1,
    Right = 2,
    Up = 4,
    Down = 8,
}

impl Dir {
    fn all() -> DirSetIter {
        DirSetIter(15)
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct DirSet(u8);

impl DirSet {
    fn new() -> DirSet {
        DirSet(0)
    }

    fn insert(&mut self, dir: Dir) -> bool {
        let orig = self.0;
        self.0 |= dir as u8;
        self.0 != orig
    }

    fn is_empty(&self) -> bool {
        self.0 == 0
    }

    fn iter(&self) -> DirSetIter {
        DirSetIter(self.0)
    }
}

impl fmt::Debug for DirSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ones = self.0.count_ones();
        match ones {
            0 => write!(f, "."),
            1 => {
                for dir in self.iter() {
                    let ch = match dir {
                        Dir::Left => '<',
                        Dir::Right => '>',
                        Dir::Up => '^',
                        Dir::Down => 'v',
                    };
                    write!(f, "{}", ch)?;
                }
                Ok(())
            },
            _ => write!(f, "{}", ones)
        }
    }
}

struct DirSetIter(u8);

impl Iterator for DirSetIter {
    type Item = Dir;
    fn next(&mut self) -> Option<Dir> {
        if self.0 == 0 {
            return None
        }
        let zeros = self.0.trailing_zeros();
        debug_assert!(zeros <= 3);
        self.0 &= !(1u8 << zeros);
        let ret = unsafe { mem::transmute(1u8 << zeros) };
        Some(ret)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Clone)]
struct Grid {
    width: usize,
    height: usize,
    hurricanes: Vec<DirSet>,
}

impl ops::Index<Pos> for Grid {
    type Output = DirSet;
    fn index(&self, pos: Pos) -> &DirSet {
        debug_assert!(pos.y < self.height);
        debug_assert!(pos.x < self.width);
        &self.hurricanes[pos.y * self.width + pos.x]
    }
}

impl ops::IndexMut<Pos> for Grid {
    fn index_mut(&mut self, pos: Pos) -> &mut DirSet {
        debug_assert!(pos.y < self.height);
        debug_assert!(pos.x < self.width);
        &mut self.hurricanes[pos.y * self.width + pos.x]
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Pos { x, y };
                if self.is_wall(pos) {
                    write!(f, "#")?;
                } else {
                    write!(f, "{:?}", self[pos])?;
                }
            }
            if y != self.height - 1 { write!(f, "\n")?; }
        }
        Ok(())
    }
}

impl Grid {
    fn start(&self) -> Pos {
        let pos = Pos { x: 1, y: 0 };
        debug_assert_eq!(self[pos], DirSet::new());
        pos
    }

    fn finish(&self) -> Pos {
        let pos = Pos { x: self.width - 2, y: self.height - 1 };
        debug_assert_eq!(self[pos], DirSet::new());
        pos
    }

    fn is_wall(&self, pos: Pos) -> bool {
        pos != self.start() && pos != self.finish() && (
            pos.x == 0 || pos.x == self.width - 1 ||
            pos.y == 0 || pos.y == self.height - 1
        )
    }

    fn parse(input: &str) -> Grid {
        let input = input.trim_end();
        let height = input.lines().count();
        let width = input.lines().next().unwrap().chars().count();
        let mut grid = Grid {
            width, height,
            hurricanes: vec![DirSet::new(); width * height]
        };
        for (y, line) in input.lines().enumerate() {
            assert_eq!(width, line.chars().count());
            for (x, ch) in line.chars().enumerate() {
                let pos = Pos { x, y };
                if ch == '#' {
                    assert!(grid.is_wall(pos));
                } else {
                    assert!(!grid.is_wall(pos));
                    match ch {
                        '.' => (),
                        '>' => { grid[pos].insert(Dir::Right); },
                        '<' => { grid[pos].insert(Dir::Left); },
                        '^' => {
                            debug_assert_ne!(grid.start().x, x);
                            debug_assert_ne!(grid.finish().x, x);
                            grid[pos].insert(Dir::Up);
                        }
                        'v' => {
                            debug_assert_ne!(grid.start().x, x);
                            debug_assert_ne!(grid.finish().x, x);
                            grid[pos].insert(Dir::Down);
                        },
                        _ => panic!(),
                    };
                }
            }
        }
        grid
    }

    fn step_hurricane(&self, pos: Pos, dir: Dir) -> Pos {
        match dir {
            Dir::Up => {
                let mut pos = Pos { x: pos.x, y: pos.y - 1 };
                if self.is_wall(pos) {
                    pos = Pos { x: pos.x, y: self.height - 2 };
                }
                pos
            },
            Dir::Down => {
                let mut pos = Pos { x: pos.x, y: pos.y + 1 };
                if self.is_wall(pos) {
                    pos = Pos { x: pos.x, y: 1 };
                }
                pos
            },
            Dir::Left => {
                let mut pos = Pos { x: pos.x - 1, y: pos.y };
                if self.is_wall(pos) {
                    pos = Pos { x: self.width - 2, y: pos.y };
                }
                pos
            },
            Dir::Right => {
                let mut pos = Pos { x: pos.x + 1, y: pos.y };
                if self.is_wall(pos) {
                    pos = Pos { x: 1, y: pos.y };
                }
                pos
            },
        }
    }

    fn step_hurricanes(self) -> Grid {
        let height = self.height;
        let width = self.width;
        let orig = self;
        let mut ret = Grid { width, height, hurricanes: vec![DirSet::new(); width * height] };
        for y in 1..height-1 {
            for x in 1..width-1 {
                let pos = Pos { x, y };
                for dir in orig[pos].iter() {
                    let pos = ret.step_hurricane(pos, dir);
                    ret[pos].insert(dir);
                }
            }
        }
        ret
    }

    fn walk(&self, pos: Pos, dir: Dir) -> Option<Pos> {
        let pos = match dir {
            Dir::Up if pos != self.start() => Pos { x: pos.x, y: pos.y - 1 },
            Dir::Down if pos != self.finish() => Pos { x: pos.x, y: pos.y + 1 },
            Dir::Left => Pos { x: pos.x - 1, y: pos.y },
            Dir::Right => Pos { x: pos.x + 1, y: pos.y },
            _ => return None,
        };
        if self.is_wall(pos) || !self[pos].is_empty() { None } else { Some(pos) }
    }
}

fn walk_path(mut grid: Grid, start_pos: Pos, finish_pos: Pos) -> (Grid, usize) {
    let mut locations = HashSet::new();
    locations.insert(start_pos);
    for minute in 0.. {
        if locations.is_empty() { panic!() }
        if locations.contains(&finish_pos) { return (grid, minute) }
        let orig_locations = locations;
        locations = HashSet::new();
        grid = grid.step_hurricanes();
        for pos in orig_locations {
            if grid[pos].is_empty() { locations.insert(pos); }
            for dir in Dir::all() {
                if let Some(pos) = grid.walk(pos, dir) {
                    locations.insert(pos);
                }
            }
        }
    }
    panic!()
}

fn part1(puzzle_input: &str) -> usize {
    let grid = Grid::parse(puzzle_input);
    let start_pos = grid.start();
    let finish_pos = grid.finish();
    let (_grid, steps) = walk_path(grid, start_pos, finish_pos);
    steps
}

fn part2(puzzle_input: &str) -> usize {
    let grid = Grid::parse(puzzle_input);
    let start_pos = grid.start();
    let finish_pos = grid.finish();
    let (grid, steps) = walk_path(grid, start_pos, finish_pos);
    let mut total = steps;
    let (grid, steps) = walk_path(grid, finish_pos, start_pos);
    total += steps;
    let (_grid, steps) = walk_path(grid, start_pos, finish_pos);
    total + steps
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

    const EX: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EX), 54);
    }
}
