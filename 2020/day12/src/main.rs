use std::io::{self, Read};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Copy, Clone, PartialEq)]
enum Dir {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward
}

impl Dir {
    fn rotate_left(&self) -> Dir {
        match self {
            Dir::North => Dir::West,
            Dir::South => Dir::East,
            Dir::East => Dir::North,
            Dir::West => Dir::South,
            _ => panic!()
        }
    }

    fn rotate_right(&self) -> Dir {
        match self {
            Dir::North => Dir::East,
            Dir::South => Dir::West,
            Dir::East => Dir::South,
            Dir::West => Dir::North,
            _ => panic!()
        }
    }
}

struct Action {
    dir: Dir,
    val: i32
}

fn parse(puzzle_input: &str) -> Vec<Action> {
    lazy_static! {
        static ref RE: Regex = Regex::new("^([NSEWLRF])([0-9]+)$").unwrap();
    }
    puzzle_input.lines().map(|line| {
        let cap = RE.captures(line).unwrap();
        let dir = match &cap[1] {
            "N" => Dir::North,
            "S" => Dir::South,
            "E" => Dir::East,
            "W" => Dir::West,
            "L" => Dir::Left,
            "R" => Dir::Right,
            "F" => Dir::Forward,
            _ => panic!()
        };
        let val = cap[2].parse().unwrap();
        assert!(val > 0);
        if (dir == Dir::Left || dir == Dir::Right) && (val % 90 != 0) { panic!() }
        Action { dir, val }
    }).collect()
}

fn part1(actions: &Vec<Action>) -> u32 {
    let mut east = 0;
    let mut north = 0;
    let mut facing = Dir::East;
    for action in actions {
        match action.dir {
            Dir::North => { north += action.val },
            Dir::South => { north -= action.val },
            Dir::East => { east += action.val },
            Dir::West => { east -= action.val },
            Dir::Left => {
                for _ in 0..action.val/90 {
                    facing = facing.rotate_left();
                }
            },
            Dir::Right => {
                for _ in 0..action.val/90 {
                    facing = facing.rotate_right();
                }
            },
            Dir::Forward => {
                match facing {
                    Dir::North => { north += action.val },
                    Dir::South => { north -= action.val },
                    Dir::East => { east += action.val },
                    Dir::West => { east -= action.val },
                    _ => panic!()
                }
            }
        }
    }
    (east.abs() + north.abs()) as u32
}

fn part2(actions: &Vec<Action>) -> u32 {
    let mut east = 0;
    let mut north = 0;
    let mut wayp_east = 10;
    let mut wayp_north = 1;
    for action in actions {
        match action.dir {
            Dir::North => { wayp_north += action.val },
            Dir::South => { wayp_north -= action.val },
            Dir::East => { wayp_east += action.val },
            Dir::West => { wayp_east -= action.val },
            Dir::Left => {
                for _ in 0..action.val/90 {
                    let orig_wayp_north = wayp_north;
                    wayp_north = wayp_east;
                    wayp_east = -orig_wayp_north;
                }
            },
            Dir::Right => {
                for _ in 0..action.val/90 {
                    let orig_wayp_north = wayp_north;
                    wayp_north = -wayp_east;
                    wayp_east = orig_wayp_north;
                }
            },
            Dir::Forward => {
                north += wayp_north * action.val;
                east += wayp_east * action.val;
            }
        }
    }
    (east.abs() + north.abs()) as u32
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();
    let actions = parse(&puzzle_input);

    println!("{}", part1(&actions));
    println!("{}", part2(&actions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "F10
N3
F7
R90
F11";

    #[test]
    fn test_part1() {
        assert_eq!(25, part1(&parse(EX)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(286, part2(&parse(EX)));
    }
}
