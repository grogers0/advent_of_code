use std::collections::{HashSet, VecDeque};
use std::io::{self, Read};
use std::ops::Add;

use lazy_static::lazy_static;

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
struct Pos {
    x: i8,
    y: i8,
    z: i8,
}

impl Pos {
    fn new(x: i8, y: i8, z: i8) -> Pos {
        Pos { x, y, z }
    }
}

impl Add<Pos> for Pos {
    type Output = Pos;
    fn add(self, other: Pos) -> Pos {
        Pos::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

fn parse(puzzle_input: &str) -> HashSet<Pos> {
    puzzle_input.trim_end().lines().map(|line| {
        let mut it = line.split(",");
        let x = it.next().unwrap().parse().unwrap();
        let y = it.next().unwrap().parse().unwrap();
        let z = it.next().unwrap().parse().unwrap();
        assert!(it.next().is_none());
        Pos { x, y, z }
    }).collect()
}

lazy_static!{
    static ref OFFSETS: Vec<Pos> = vec![
        Pos::new(-1, 0, 0), Pos::new(1, 0, 0),
        Pos::new(0, -1, 0), Pos::new(0, 1, 0),
        Pos::new(0, 0, -1), Pos::new(0, 0, 1),
    ];
}

fn part1(cubes: &HashSet<Pos>) -> usize {
    let mut cnt = 0;
    for &pos in cubes.iter() {
        for &offset in OFFSETS.iter() {
            if !cubes.contains(&(pos + offset)) {
                cnt += 1;
            }
        }
    }
    cnt
}

fn part2(cubes: &HashSet<Pos>) -> usize {
    // Conservative bounding box for the external flood fill
    let min_x = cubes.iter().map(|pos| pos.x).min().unwrap() - 1;
    let max_x = cubes.iter().map(|pos| pos.x).max().unwrap() + 1;
    let min_y = cubes.iter().map(|pos| pos.y).min().unwrap() - 1;
    let max_y = cubes.iter().map(|pos| pos.y).max().unwrap() + 1;
    let min_z = cubes.iter().map(|pos| pos.z).min().unwrap() - 1;
    let max_z = cubes.iter().map(|pos| pos.z).max().unwrap() + 1;
    let mut seen = HashSet::new();
    let mut cnt = 0;
    let mut queue = VecDeque::new();
    queue.push_back(Pos::new(min_x, min_y, min_z));
    while let Some(pos) = queue.pop_front() {
        if !seen.insert(pos) { continue; }
        for &offset in OFFSETS.iter() {
            let pos = pos + offset;
            if cubes.contains(&pos) {
                cnt += 1;
            } else if pos.x >= min_x && pos.x <= max_x &&
                      pos.y >= min_y && pos.y <= max_y &&
                      pos.z >= min_z && pos.z <= max_z {
                queue.push_back(pos);
            }
        }
    }
    cnt
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();


    let cubes = parse(&puzzle_input);
    println!("{}", part1(&cubes));
    println!("{}", part2(&cubes));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX)), 64);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EX)), 58);
    }
}
