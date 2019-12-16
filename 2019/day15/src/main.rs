use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Write;
use std::io::{self, Read};
use std::ops;
use std::sync::mpsc::channel;
use std::thread;

use intcode::*;

#[derive(Clone, Copy)]
enum Dir {
    North,
    South,
    East,
    West
}

const DIRS: [Dir; 4] = [Dir::North, Dir::South, Dir::East, Dir::West];

impl Dir {
    fn to_command(&self) -> i64 {
        match self {
            Dir::North => 1,
            Dir::South => 2,
            Dir::East => 3,
            Dir::West => 4
        }
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Pos { x: i32, y: i32 }

impl Pos {
    fn origin() -> Pos {
        Pos { x: 0, y: 0 }
    }

    fn new(x: i32, y: i32) -> Pos {
        Pos { x: x, y: y }
    }
}

impl ops::Add<Dir> for Pos {
    type Output = Pos;
    fn add(self, dir: Dir) -> Pos {
        match dir {
            Dir::North => Pos { x: self.x, y: self.y - 1 },
            Dir::South => Pos { x: self.x, y: self.y + 1 },
            Dir::East => Pos { x: self.x + 1, y: self.y },
            Dir::West => Pos { x: self.x - 1, y: self.y }
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Square {
    Wall,
    Oxygen,
    Ground
}

fn dist_to_next_unexplored(start_pos: Pos, map: &HashMap<Pos, Square>) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    queue.push_back((start_pos, 0));
    while let Some((pos, dist)) = queue.pop_front() {
        if !seen.insert(pos) {
            continue;
        }
        match map.get(&pos) {
            None => return Some(dist),
            Some(Square::Wall) => continue,
            Some(Square::Ground) => (),
            Some(Square::Oxygen) => ()
        }
        for dir in DIRS.iter() {
            queue.push_back((pos + *dir, dist + 1));
        }
    }
    None
}

fn dir_to_next_unexplored(pos: Pos, map: &HashMap<Pos, Square>) -> Option<Dir> {
    DIRS.iter().filter(|dir| map.get(&(pos + **dir)).unwrap_or(&Square::Ground) != &Square::Wall)
        .map(|dir| (dir, dist_to_next_unexplored(pos + *dir, map)))
        .filter(|(_, dist)| dist.is_some())
        .map(|(dir, dist)| (dir, dist.unwrap()))
        .min_by_key(|(_, dist)| *dist)
        .map(|(dir, _)| *dir)
}

fn build_map(mem_str: &str) -> HashMap<Pos, Square> {
    let (tx_in, rx_in) = channel();
    let (tx_out, rx_out) = channel();

    let mut mem = parse(mem_str);
    thread::spawn(move || run(&mut mem, rx_in, tx_out));

    let mut map = HashMap::new();
    let mut pos = Pos::origin();
    while let Some(dir) = dir_to_next_unexplored(pos, &map) {
        tx_in.send(dir.to_command()).unwrap();
        match rx_out.recv().unwrap() {
            0 => {
                map.insert(pos + dir, Square::Wall);
            },
            1 => {
                map.insert(pos + dir, Square::Ground);
                pos = pos + dir;
            },
            2 => {
                map.insert(pos + dir, Square::Oxygen);
                pos = pos + dir;
            },
            _ => panic!()
        }
    }
    std::mem::drop(rx_out);
    tx_in.send(Dir::North.to_command()).unwrap();

    map
}

#[allow(dead_code)]
fn map_to_string(map: &HashMap<Pos, Square>) -> String {
    let min_x = map.keys().map(|pos| pos.x).min().unwrap();
    let max_x = map.keys().map(|pos| pos.x).max().unwrap();
    let min_y = map.keys().map(|pos| pos.y).min().unwrap();
    let max_y = map.keys().map(|pos| pos.y).max().unwrap();

    let mut f = String::new();
    for y in min_y..max_y+1 {
        for x in min_x..max_x+1 {
            write!(f, "{}", match map.get(&Pos::new(x, y)) {
                None => " ",
                Some(Square::Wall) => "#",
                Some(Square::Ground) => ".",
                Some(Square::Oxygen) => "O"
            }).unwrap();
        }
        write!(f, "\n").unwrap();
    }
    f
}

fn part1(mem_str: &str) -> usize {
    let map = build_map(mem_str);

    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    queue.push_back((Pos::origin(), 0));
    while let Some((pos, dist)) = queue.pop_front() {
        if !seen.insert(pos) {
            continue;
        }
        match map.get(&pos) {
            Some(Square::Wall) => continue,
            Some(Square::Ground) => (),
            Some(Square::Oxygen) => return dist,
            None => panic!()
        }
        for dir in DIRS.iter() {
            queue.push_back((pos + *dir, dist + 1));
        }
    }
    panic!();
}

fn part2(mem_str: &str) -> usize {
    let mut steps = 0;
    let mut map = build_map(mem_str);
    while map.values().any(|sq| *sq == Square::Ground) {
        let mut nextmap = map.clone();
        for (pos, sq) in map.iter() {
            match sq {
                Square::Oxygen => (),
                _ => continue
            }
            for dir in DIRS.iter() {
                match map.get(&(*pos + *dir)) {
                    Some(Square::Wall) => (),
                    Some(_) => *nextmap.get_mut(&(*pos + *dir)).unwrap() = Square::Oxygen,
                    None => panic!()
                }
            }
        }
        map = nextmap;
        steps += 1;
    }
    steps
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
