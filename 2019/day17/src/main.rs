use std::fmt::Write;
use std::io::{self, Read};
use std::sync::mpsc::channel;
use std::thread;

use intcode::*;

#[derive(Debug, Copy, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right
}

impl Dir {
    fn turn(self, turn: Turn) -> Dir {
        match (self, turn) {
            (Dir::Up, Turn::Left) => Dir::Left,
            (Dir::Up, Turn::Right) => Dir::Right,
            (Dir::Down, Turn::Left) => Dir::Right,
            (Dir::Down, Turn::Right) => Dir::Left,
            (Dir::Left, Turn::Left) => Dir::Down,
            (Dir::Left, Turn::Right) => Dir::Up,
            (Dir::Right, Turn::Left) => Dir::Up,
            (Dir::Right, Turn::Right) => Dir::Down
        }
    }

    fn dxdy(self) -> (isize, isize) {
        match self {
            Dir::Up => (0, -1),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0)
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Square {
    Open,
    Scaffold,
    Robot(Dir),
    Falling
}

impl Square {
    fn is_scaffold(&self) -> bool {
        match self {
            Square::Open | Square::Falling => false,
            Square::Scaffold | Square::Robot(_) => true
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Turn { Left, Right }

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Move {
    Turn(Turn),
    Step(usize),
}

struct Map {
    width: usize,
    height: usize,
    data: Vec<Square>
}

impl Map {
    fn find_robot(&self) -> (isize, isize, Dir) {
        for y in 0 .. self.height as isize {
            for x in 0 .. self.width as isize {
                if let Square::Robot(dir) = &self.at(x, y).unwrap() {
                    return (x, y, *dir)
                }
            }
        }
        panic!()
    }

    fn at(&self, x: isize, y: isize) -> Option<&Square> {
        if x < 0 || x >= self.width as isize {
            None
        } else if y < 0 || y >= self.height as isize {
            None
        } else {
            Some(&self.data[self.width * y as usize + x as usize])
        }
    }
}

fn snapshot_to_map(snapshot: &str) -> Map {
    let width = snapshot.trim().lines().next().unwrap().chars().count();
    let data = snapshot.trim().lines().flat_map(|line| {
        assert_eq!(width, line.chars().count());
        line.chars().map(|ch| {
            match ch {
                '.' => Square::Open,
                '#' => Square::Scaffold,
                '^' => Square::Robot(Dir::Up),
                'v' => Square::Robot(Dir::Down),
                '<' => Square::Robot(Dir::Left),
                '>' => Square::Robot(Dir::Right),
                'X' => Square::Falling,
                _ => panic!()
            }
        })
    }).collect::<Vec<_>>();
    Map { width: width, height: data.len() / width, data: data }
}

fn take_snapshot(mem_str: &str) -> String {
    let (tx_out, rx_out) = channel();
    run(&mut parse(mem_str), &channel().1, tx_out);
    let mut ret = String::new();
    while let Ok(val) = rx_out.recv() {
        ret.push(val as u8 as char);
    }
    ret
}

fn serialize_moves(moves: &[Move]) -> String {
    let mut ret = String::new();
    let mut first = true;
    for m in moves {
        if first { first = false } else { ret.push(',') }
        match m {
            Move::Turn(Turn::Left) => write!(ret, "L").unwrap(),
            Move::Turn(Turn::Right) => write!(ret, "R").unwrap(),
            Move::Step(steps) => write!(ret, "{}", steps).unwrap()
        };
    }
    ret
}

fn get_path(map: &Map) -> Vec<Move> {
    fn get_steps(x: isize, y: isize, dir: Dir, map: &Map) -> isize {
        let (dx, dy) = dir.dxdy();
        let mut steps = 1;
        while map.at(x + steps * dx, y + steps * dy).map_or(false, |sq| sq.is_scaffold()) {
            steps += 1;
        }
        steps - 1
    }
    fn can_turn(x: isize, y: isize, newdir: Dir, map: &Map) -> bool {
        let (dx, dy) = newdir.dxdy();
        map.at(x + dx, y + dy).map_or(false, |sq| sq.is_scaffold())
    }
    let mut ret = Vec::new();
    let (mut x, mut y, mut dir) = map.find_robot();
    loop {
        let steps = get_steps(x, y, dir, map);
        if steps > 0 { // Try to follow the path first, then turn, otherwise you'd spin around
            ret.push(Move::Step(steps as usize));
            let (dx, dy) = dir.dxdy();
            x += steps * dx;
            y += steps * dy;
        } else if can_turn(x, y, dir.turn(Turn::Left), map) {
            ret.push(Move::Turn(Turn::Left));
            dir = dir.turn(Turn::Left);
        } else if can_turn(x, y, dir.turn(Turn::Right), map) {
            ret.push(Move::Turn(Turn::Right));
            dir = dir.turn(Turn::Right);
        } else {
            return ret;
        }
    }
}

// NOTE - This is naive and takes a second. It's not obvious how you could make it faster though.
fn get_move_commands(path: Vec<Move>) -> (String, String, String, String) {
    fn make_path(a: &[Move], b: &[Move], c: &[Move], path: &[Move]) -> Option<String> {
        if path.is_empty() {
            return Some(String::new())
        }
        if path[0 .. a.len()] == *a {
            if let Some(rest) = make_path(a, b, c, &path[a.len() .. ]) {
                return Some(if rest.is_empty() { "A".into() } else { format!("A,{}", rest) })
            }
        }
        if path[0 .. b.len()] == *b {
            if let Some(rest) = make_path(a, b, c, &path[b.len() .. ]) {
                return Some(if rest.is_empty() { "B".into() } else { format!("B,{}", rest) })
            }
        }
        if path[0 .. c.len()] == *c {
            if let Some(rest) = make_path(a, b, c, &path[c.len() .. ]) {
                return Some(if rest.is_empty() { "C".into() } else { format!("C,{}", rest) })
            }
        }
        None
    }

    let start_a = 0;
    for end_a in start_a + 1 .. path.len() {
        let a = &path[start_a .. end_a];
        if serialize_moves(a).chars().count() > 20 { continue }

        for start_b in end_a .. path.len() {
            for end_b in start_b + 1 .. path.len() {
                let b = &path[start_b .. end_b];
                if serialize_moves(b).chars().count() > 20 { continue }

                for start_c in end_b .. path.len() {
                    for end_c in start_c + 1 .. path.len() {
                        let c = &path[start_c .. end_c];
                        if serialize_moves(c).chars().count() > 20 { continue }

                        if let Some(main) = make_path(a, b, c, &path) {
                            return (serialize_moves(a), serialize_moves(b), serialize_moves(c), main)
                        }
                    }
                }
            }
        }
    }
    panic!()
}

fn part1(mem_str: &str) -> isize {
    let map = snapshot_to_map(&take_snapshot(mem_str));
    let mut sum = 0;
    for y in 1 .. map.height as isize - 1 {
        for x in 1 .. map.width as isize - 1 {
            if map.at(x, y).unwrap().is_scaffold()
                && map.at(x + 1, y).unwrap().is_scaffold()
                && map.at(x - 1, y).unwrap().is_scaffold()
                && map.at(x, y + 1).unwrap().is_scaffold()
                && map.at(x, y - 1).unwrap().is_scaffold() {
                sum += x * y;
            }
        }
    }
    sum
}

fn part2(mem_str: &str) -> i64 {
    let snapshot = take_snapshot(mem_str);
    let map = snapshot_to_map(&snapshot);
    let path = get_path(&map);
    let (a, b, c, main) = get_move_commands(path);
    let video = "n".into();

    let (tx_in, rx_in) = channel();
    let (tx_out, rx_out) = channel();
    let mut mem = parse(mem_str);
    mem[0] = 2;
    thread::spawn(move || run(&mut mem, &rx_in, tx_out));
    for line in &[main, a, b, c, video] {
        send_line(&tx_in, line)
    }
    let mut ret = 0;
    while let Ok(val) = rx_out.recv() {
        ret = val; // Only the very last value output is what we want
    }
    ret
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
