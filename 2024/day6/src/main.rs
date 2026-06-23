use std::io::{self, Read};
use std::collections::HashSet;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Dir {
    dx: isize,
    dy: isize,
}

impl Pos {
    fn step(self, dir: Dir) -> Pos {
        assert!(dir.dx >= 0 || self.x > 0);
        assert!(dir.dy >= 0 || self.y > 0);
        let add = |a: usize, da: isize| -> usize { ((a as isize) + da) as usize };
        Pos { x: add(self.x, dir.dx), y: add(self.y, dir.dy) }
    }
}

impl Dir {
    fn turn_right(self) -> Dir {
        Dir { dx: self.dy * -1, dy: self.dx }
    }
}

struct Map {
    width: usize,
    height: usize,
    guard_pos: Pos,
    obstructions: Vec<bool>,
}

impl Map {
    fn idx(&self, pos: Pos) -> usize {
        assert!(pos.x < self.width);
        assert!(pos.y < self.height);
        self.width * pos.y + pos.x
    }

    fn would_step_out_of_bounds(&self, pos: Pos, dir: Dir) -> bool {
        if dir.dx < 0 && pos.x == 0               { return true; }
        if dir.dx > 0 && pos.x == self.width - 1  { return true; }
        if dir.dy < 0 && pos.y == 0               { return true; }
        if dir.dy > 0 && pos.y == self.height - 1 { return true; }
        false
    }

    fn blocked_by_obstruction(&self, pos: Pos, dir: Dir) -> bool {
        assert!(!self.would_step_out_of_bounds(pos, dir));
        self.obstructions[self.idx(pos.step(dir))]
    }
}

fn parse(puzzle_input: &str) -> Map {
    let height = puzzle_input.lines().count();
    let width = puzzle_input.lines().next().unwrap().chars().count();
    let mut obstructions = Vec::new();
    let mut guard_pos: Option<Pos> = None;
    for (y, line) in puzzle_input.lines().enumerate() {
        assert_eq!(width, line.chars().count());
        for (x, ch) in line.chars().enumerate() {
            let is_obstruction = match ch {
                '.' => false,
                '#' => true,
                '^' => {
                    assert!(guard_pos.is_none());
                    guard_pos = Some(Pos { x, y });
                    false
                },
                _ => panic!(),
            };
            obstructions.push(is_obstruction);
        }
    }
    let guard_pos = guard_pos.unwrap();
    Map { width, height, guard_pos, obstructions }
}


fn part1(map: &Map) -> usize {
    let mut seen = HashSet::new();
    let mut dir = Dir { dx: 0, dy: -1 }; // Facing up
    let mut pos = map.guard_pos;

    seen.insert(pos);
    while !map.would_step_out_of_bounds(pos, dir) {
        if map.blocked_by_obstruction(pos, dir) {
            dir = dir.turn_right();
        } else {
            pos = pos.step(dir);
            seen.insert(pos);
        }
    }

    seen.len()
}

fn would_loop(map: &Map) -> bool {
    let mut seen = HashSet::new();
    let mut dir = Dir { dx: 0, dy: -1 }; // Facing up
    let mut pos = map.guard_pos;

    seen.insert((pos, dir));
    while !map.would_step_out_of_bounds(pos, dir) {
        if map.blocked_by_obstruction(pos, dir) {
            dir = dir.turn_right();
        } else {
            pos = pos.step(dir);
        }
        if !seen.insert((pos, dir)) { return true; }
    }
    false
}

fn part2(mut map: Map) -> usize {
    let mut cnt = 0;

    // Same as part1 - pre-calculate everywhere the guard goes since placing an obstruction
    // anywhere not visited would not impact the path
    let mut seen = HashSet::new();
    let mut dir = Dir { dx: 0, dy: -1 }; // Facing up
    let mut pos = map.guard_pos;

    seen.insert(pos);
    while !map.would_step_out_of_bounds(pos, dir) {
        if map.blocked_by_obstruction(pos, dir) {
            dir = dir.turn_right();
        } else {
            pos = pos.step(dir);
            seen.insert(pos);
        }
    }

    for pos in seen {
        let idx = map.idx(pos);
        map.obstructions[idx] = true;
        if would_loop(&map) { cnt += 1; }
        map.obstructions[idx] = false;
    }
    cnt
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let map = parse(&puzzle_input);
    println!("{}", part1(&map));
    println!("{}", part2(map));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        
    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX)), 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse(EX)), 6);
    }
}
