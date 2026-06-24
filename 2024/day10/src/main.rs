use std::io::{self, Read};
use std::collections::{HashSet, VecDeque};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

struct Map {
    width: usize,
    height: usize,
    grid: Vec<u8>,
}

impl Map {
    fn idx(&self, pos: Pos) -> usize {
        self.width * pos.y + pos.x
    }
}

fn parse(puzzle_input: &str) -> Map {
    let height = puzzle_input.lines().count();
    let width = puzzle_input.lines().next().unwrap().chars().count();
    let mut grid = Vec::new();
    for line in puzzle_input.lines() {
        assert_eq!(width, line.chars().count());
        for ch in line.chars() {
            grid.push(ch.to_digit(10).unwrap() as u8);
        }
    }

    Map { width, height, grid }
}

fn part1(map: &Map) -> usize {
    fn score(map: &Map, init_pos: Pos) -> usize {
        let mut reachable = HashSet::new();
        let mut deque = VecDeque::new();
        deque.push_back(init_pos);
        assert_eq!(map.grid[map.idx(init_pos)], 0);

        while let Some(pos) = deque.pop_front() {
            let curr = map.grid[map.idx(pos)];
            if curr == 9 {
                reachable.insert(pos);
                continue;
            }
            let mut check = |next_pos: Pos| {
                if map.grid[map.idx(next_pos)] == curr + 1 {
                    deque.push_back(next_pos);
                }
            };
            if pos.x > 0              { check(Pos { x: pos.x - 1, y: pos.y     }); }
            if pos.x < map.width - 1  { check(Pos { x: pos.x + 1, y: pos.y     }); }
            if pos.y > 0              { check(Pos { x: pos.x,     y: pos.y - 1 }); }
            if pos.y < map.height - 1 { check(Pos { x: pos.x,     y: pos.y + 1 }); }
        }
        reachable.len()
    }

    let mut sum = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            let pos = Pos { x, y };
            if map.grid[map.idx(pos)] != 0 { continue; }
            sum += score(map, pos);
        }
    }
    sum
}

fn part2(map: &Map) -> usize {
    fn rating(map: &Map, init_pos: Pos) -> usize {
        let mut deque = VecDeque::new();
        deque.push_back(init_pos);
        assert_eq!(map.grid[map.idx(init_pos)], 0);

        let mut cnt = 0;
        while let Some(pos) = deque.pop_front() {
            let curr = map.grid[map.idx(pos)];
            if curr == 9 {
                cnt += 1;
                continue;
            }
            let mut check = |next_pos: Pos| {
                if map.grid[map.idx(next_pos)] == curr + 1 {
                    deque.push_back(next_pos);
                }
            };
            if pos.x > 0              { check(Pos { x: pos.x - 1, y: pos.y     }); }
            if pos.x < map.width - 1  { check(Pos { x: pos.x + 1, y: pos.y     }); }
            if pos.y > 0              { check(Pos { x: pos.x,     y: pos.y - 1 }); }
            if pos.y < map.height - 1 { check(Pos { x: pos.x,     y: pos.y + 1 }); }
        }
        cnt
    }

    let mut sum = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            let pos = Pos { x, y };
            if map.grid[map.idx(pos)] != 0 { continue; }
            sum += rating(map, pos);
        }
    }
    sum
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

    const EX: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX)), 36);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EX)), 81);
    }
}
