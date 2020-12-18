use std::collections::HashSet;
use std::io::{self, Read};

type Point = [i32; 4];
type Map = HashSet<Point>; // The active points on the infinite grid

fn parse(puzzle_input: &str) -> Map {
    let mut active = Map::new();
    for (y, line) in puzzle_input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '.' => (),
                '#' => { active.insert([x as i32, y as i32, 0, 0]); },
                _ => panic!()
            }
        }
    }
    active
}

fn points_of_interest(map: &Map, is_4d: bool) -> HashSet<Point> {
    let mut points = HashSet::new();
    for &[x, y, z, w] in map {
        for xn in (x-1)..=(x+1) {
            for yn in (y-1)..=(y+1) {
                for zn in (z-1)..=(z+1) {
                    for wn in (w-1)..=(w+1) {
                        if !is_4d && wn != w { continue }
                        points.insert([xn, yn, zn, wn]);
                    }
                }
            }
        }
    }
    points
}

fn count_active_neighbors(map: &Map, [x, y, z, w]: Point, is_4d: bool) -> usize {
    let mut cnt = 0;
    for xn in (x-1)..=(x+1) {
        for yn in (y-1)..=(y+1) {
            for zn in (z-1)..=(z+1) {
                for wn in (w-1)..=(w+1) {
                    if !is_4d && w != wn { continue }
                    if x == xn && y == yn && z == zn && w == wn { continue }
                    if map.contains(&[xn, yn, zn, wn]) {
                        cnt += 1;
                    }
                }
            }
        }
    }
    cnt
}

fn cycle(prev_map: Map, is_4d: bool) -> Map {
    let mut next_map = Map::new();
    for p in points_of_interest(&prev_map, is_4d) {
        let cnt = count_active_neighbors(&prev_map, p, is_4d);
        let was_active = prev_map.contains(&p);
        let is_active = match (was_active, cnt) {
            (true, 2..=3) => true,
            (false, 3) => true,
            _ => false
        };
        if is_active {
            next_map.insert(p);
        }
    }
    next_map
}

fn part1(mut map: Map) -> usize {
    for _ in 0..6 { map = cycle(map, false) }
    map.len()
}

fn part2(mut map: Map) -> usize {
    for _ in 0..6 { map = cycle(map, true) }
    map.len()
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();
    let map = parse(&puzzle_input);

    println!("{}", part1(map.clone()));
    println!("{}", part2(map));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = ".#.
..#
###";

    #[test]
    fn test_part1() {
        assert_eq!(112, part1(parse(EX)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(848, part2(parse(EX)));
    }
}
