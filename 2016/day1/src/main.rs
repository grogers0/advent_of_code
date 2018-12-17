use std::collections::BTreeSet;
use std::io::{self, Read};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Copy, Clone, Debug)]
enum Turn {
    Left, Right
}

fn parse(input: &str) -> Vec<(Turn, usize)> {
    lazy_static!{
        static ref RE: Regex = Regex::new("^(L|R)(\\d+)$").unwrap();
    }
    input.trim_end().split(", ")
        .map(|line| {
            let cap = RE.captures(line).unwrap();
            let turn = match &cap[1] {
                "L" => Turn::Left,
                "R" => Turn::Right,
                _ => unreachable!()
            };
            let steps = cap[2].parse().unwrap();
            (turn, steps)
        })
        .collect()
}

fn turn_left(dir: &mut (i32, i32)) {
    if dir.0 == 0 { // vertical
        *dir = (dir.1, 0);
    } else { // horizontal
        *dir = (0, -dir.0);
    }
}

fn turn_right(dir: &mut (i32, i32)) {
    if dir.0 == 0 { // vertical
        *dir = (-dir.1, 0);
    } else { // horizontal
        *dir = (0, dir.0);
    }
}

fn turn(dir: &mut (i32, i32), turn: Turn) {
    match turn {
        Turn::Left => turn_left(dir),
        Turn::Right => turn_right(dir)
    }
}

fn walk(pos: &mut (i32, i32), dir: (i32, i32)) {
    pos.0 += dir.0;
    pos.1 += dir.1;
}

fn part1(instructions: &Vec<(Turn,usize)>) -> i32 {
    let mut pos = (0, 0);
    let mut dir = (0, -1);
    for inst in instructions.iter() {
        turn(&mut dir, inst.0);
        for _ in 0..inst.1 {
            walk(&mut pos, dir);
        }
    }

    pos.0.abs() + pos.1.abs()
}

fn part2(instructions: &Vec<(Turn,usize)>) -> i32 {
    let mut pos: (i32, i32) = (0, 0);
    let mut dir = (0, -1);
    let mut seen = BTreeSet::new();
    for inst in instructions.iter() {
        turn(&mut dir, inst.0);
        for _ in 0..inst.1 {
            if !seen.insert(pos) {
                return pos.0.abs() + pos.1.abs()
            }
            walk(&mut pos, dir);
        }
    }
    unreachable!()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let instructions = parse(&input);

    println!("{}", part1(&instructions));
    println!("{}", part2(&instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse("R2, L3")), 5);
        assert_eq!(part1(&parse("R2, R2, R2")), 2);
        assert_eq!(part1(&parse("R5, L5, R5, R3")), 12);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse("R8, R4, R4, R8")), 4);
    }

}
