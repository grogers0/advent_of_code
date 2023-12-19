use std::cmp::max;
use std::collections::{HashSet, VecDeque};
use std::io::{self, Read};

#[derive(Copy, Clone)]
enum Tile {
    Empty, // .
    ForwardSlashMirror, // /
    BackslashMirror, // \
    VerticalSplitter, // |
    HorizontalSplitter, // -
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
enum Dir {
    Left, Right, Up, Down
}

struct Map {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl Map {
    fn idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn at(&self, x: usize, y: usize) -> Tile {
        self.tiles[self.idx(x, y)]
    }
}

fn parse(puzzle_input: &str) -> Map {
    let height = puzzle_input.lines().count();
    let width = puzzle_input.lines().next().unwrap().chars().count();
    let mut tiles = Vec::with_capacity(width * height);
    for line in puzzle_input.lines() {
        assert_eq!(width, line.chars().count());
        for ch in line.chars() {
            let tile = match ch {
                '.' => Tile::Empty,
                '/' => Tile::ForwardSlashMirror,
                '\\' => Tile::BackslashMirror,
                '|' => Tile::VerticalSplitter,
                '-' => Tile::HorizontalSplitter,
                _ => panic!(),
            };
            tiles.push(tile);
        }
    }
    Map { width, height, tiles }
}

fn next_beams(map: &Map, x: usize, y: usize, dir: Dir) -> Vec<(usize, usize, Dir)> {
    let mut candidate_dirs = Vec::new();
    match map.at(x, y) {
        Tile::Empty => {
            candidate_dirs.push(dir);
        },
        Tile::ForwardSlashMirror => { // /
            candidate_dirs.push(match dir {
                Dir::Left => Dir::Down,
                Dir::Right => Dir::Up,
                Dir::Up => Dir::Right,
                Dir::Down => Dir::Left,
            });
        },
        Tile::BackslashMirror => { // \
            candidate_dirs.push(match dir {
                Dir::Left => Dir::Up,
                Dir::Right => Dir::Down,
                Dir::Up => Dir::Left,
                Dir::Down => Dir::Right,
            });
        },
        Tile::VerticalSplitter => { // |
            match dir {
                Dir::Up | Dir::Down => candidate_dirs.push(dir),
                Dir::Left | Dir::Right => {
                    candidate_dirs.push(Dir::Up);
                    candidate_dirs.push(Dir::Down);
                }
            };
        },
        Tile::HorizontalSplitter => { // -
            match dir {
                Dir::Left | Dir::Right => candidate_dirs.push(dir),
                Dir::Up | Dir::Down => {
                    candidate_dirs.push(Dir::Left);
                    candidate_dirs.push(Dir::Right);
                }
            };
        },
    };
    let mut ret = Vec::new();
    for dir in candidate_dirs {
        match dir {
            Dir::Up if y > 0 => ret.push((x, y-1, dir)),
            Dir::Down if y < map.height - 1 => ret.push((x, y+1, dir)),
            Dir::Left if x > 0 => ret.push((x-1, y, dir)),
            Dir::Right if x < map.width - 1 => ret.push((x+1, y, dir)),
            _ => (),
        };
    }
    ret
}

fn count_energized(map: &Map, x: usize, y: usize, dir: Dir) -> usize {
    let mut energized = HashSet::new();
    let mut seen = HashSet::new();
    let mut deque = VecDeque::new();
    deque.push_back((x, y, dir));
    while let Some((x, y, dir)) = deque.pop_front() {
        if !seen.insert((x, y, dir)) { continue }
        energized.insert((x, y));
        for beam in next_beams(map, x, y, dir) {
            deque.push_back(beam);
        }
    }
    energized.len()
}

fn part1(map: &Map) -> usize {
    count_energized(map, 0, 0, Dir::Right)
}

fn part2(map: &Map) -> usize {
    let mut best = 0;
    for y in 0..map.height {
        best = max(best, count_energized(map, 0, y, Dir::Right));
        best = max(best, count_energized(map, map.width - 1, y, Dir::Left));
    }
    for x in 0..map.width {
        best = max(best, count_energized(map, x, 0, Dir::Down));
        best = max(best, count_energized(map, x, map.height - 1, Dir::Up));
    }
    best
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

    const EX: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX)), 46);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EX)), 51);
    }
}
