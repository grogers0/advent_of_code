use std::cmp::max;
use std::collections::HashSet;
use std::io::{self, Read};

#[derive(Copy, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

struct Motion {
    dir: Dir,
    steps: usize,
}

fn parse(puzzle_input: &str) -> Vec<Motion> {
    puzzle_input.trim_end().lines().map(|line| {
        let mut sp = line.split(" ");
        let dir = match sp.next().unwrap() {
            "U" => Dir::Up,
            "D" => Dir::Down,
            "R" => Dir::Right,
            "L" => Dir::Left,
            _ => panic!(),
        };
        let steps = sp.next().unwrap().parse().unwrap();
        assert!(sp.next().is_none());
        Motion { dir, steps }
    }).collect()
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    const ORIGIN: Pos = Pos { x: 0, y: 0 };
}

fn step(rope: &mut [Pos], dir: Dir) {
    match dir {
        Dir::Up => rope[0].y -= 1,
        Dir::Down => rope[0].y += 1,
        Dir::Left => rope[0].x -= 1,
        Dir::Right => rope[0].x += 1,
    }
    // Snap each subsequent knot towards the one just ahead of it
    for i in 1..rope.len() {
        let head = rope[i - 1];
        let tail = &mut rope[i];
        let dist = max((head.x - tail.x).abs(), (head.y - tail.y).abs());
        if dist <= 1 {
            break; // If this knot doesn't move, the remaining knots don't
        }
        tail.x += (head.x - tail.x).signum();
        tail.y += (head.y - tail.y).signum();
    }
}


fn simulate(motions: &[Motion], mut rope: Vec<Pos>) -> usize {
    let mut seen = HashSet::new();
    seen.insert(*rope.last().unwrap());
    for motion in motions {
        for _ in 0..motion.steps {
            step(&mut rope, motion.dir);
            seen.insert(*rope.last().unwrap());
        }
    }
    seen.len()
}

fn part1(motions: &[Motion]) -> usize {
    simulate(motions, vec![Pos::ORIGIN; 2])
}

fn part2(motions: &[Motion]) -> usize {
    simulate(motions, vec![Pos::ORIGIN; 10])
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let motions = parse(&puzzle_input);
    println!("{}", part1(&motions));
    println!("{}", part2(&motions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";
    const EX2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX1)), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EX1)), 1);
        assert_eq!(part2(&parse(EX2)), 36);
    }
}
