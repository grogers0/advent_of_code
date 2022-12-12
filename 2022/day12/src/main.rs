use std::collections::{VecDeque};
use std::io::{self, Read};

#[derive(Copy, Clone, Eq, PartialEq)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Pos {
        Pos { x, y }
    }
}

struct HeightMap {
    width: usize,
    height: usize,
    start: Pos,
    end: Pos,
    grid: Vec<u8>,
}

impl HeightMap {
    fn get(&self, pos: Pos) -> u8 {
        self.grid[self.idx(pos)]
    }

    fn idx(&self, pos: Pos) -> usize {
        self.width * pos.y + pos.x
    }

    fn parse(puzzle_input: &str) -> HeightMap {
        let puzzle_input = puzzle_input.trim_end();
        let height = puzzle_input.lines().count();
        let width = puzzle_input.lines().next().unwrap().chars().count();
        let mut grid = Vec::with_capacity(width * height);
        let mut start: Option<Pos> = None;
        let mut end: Option<Pos> = None;
        for (y, line) in puzzle_input.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                match ch {
                    'S' => {
                        assert!(start.is_none());
                        start = Some(Pos::new(x, y));
                        grid.push(0);
                    },
                    'E' => {
                        assert!(end.is_none());
                        end = Some(Pos::new(x, y));
                        grid.push(25);
                    },
                    'a'..='z' => grid.push(ch as u8 - 'a' as u8),
                    _ => panic!(),
                }
            }
        }
        HeightMap { width, height, start: start.unwrap(), end: end.unwrap(), grid }
    }
}

// Find the shortest path from start to end
fn part1(heightmap: &HeightMap) -> usize {
    let mut queue = VecDeque::new();
    let mut seen = vec![false; heightmap.width * heightmap.height];
    queue.push_back((heightmap.start, 0));
    while let Some((pos, steps)) = queue.pop_front() {
        if seen[heightmap.idx(pos)] { continue }
        seen[heightmap.idx(pos)] = true;
        if pos == heightmap.end { return steps }
        let elevation = heightmap.get(pos);

        if pos.x > 0 {
            let next_pos = Pos::new(pos.x - 1, pos.y);
            if heightmap.get(next_pos) <= elevation + 1 {
                queue.push_back((next_pos, steps + 1));
            }
        }
        if pos.x < heightmap.width - 1 {
            let next_pos = Pos::new(pos.x + 1, pos.y);
            if heightmap.get(next_pos) <= elevation + 1 {
                queue.push_back((next_pos, steps + 1));
            }
        }
        if pos.y > 0 {
            let next_pos = Pos::new(pos.x, pos.y - 1);
            if heightmap.get(next_pos) <= elevation + 1 {
                queue.push_back((next_pos, steps + 1));
            }
        }
        if pos.y < heightmap.height - 1 {
            let next_pos = Pos::new(pos.x, pos.y + 1);
            if heightmap.get(next_pos) <= elevation + 1 {
                queue.push_back((next_pos, steps + 1));
            }
        }
    }
    panic!()
}

// Find the shortest path from the end to any square at 0 elevation (working backwards)
fn part2(heightmap: &HeightMap) -> usize {
    let mut queue = VecDeque::new();
    let mut seen = vec![false; heightmap.width * heightmap.height];
    queue.push_back((heightmap.end, 0));
    while let Some((pos, steps)) = queue.pop_front() {
        if seen[heightmap.idx(pos)] { continue }
        seen[heightmap.idx(pos)] = true;
        let elevation = heightmap.get(pos);
        if elevation == 0 { return steps }

        if pos.x > 0 {
            let next_pos = Pos::new(pos.x - 1, pos.y);
            if heightmap.get(next_pos) + 1 >= elevation {
                queue.push_back((next_pos, steps + 1));
            }
        }
        if pos.x < heightmap.width - 1 {
            let next_pos = Pos::new(pos.x + 1, pos.y);
            if heightmap.get(next_pos) + 1 >= elevation {
                queue.push_back((next_pos, steps + 1));
            }
        }
        if pos.y > 0 {
            let next_pos = Pos::new(pos.x, pos.y - 1);
            if heightmap.get(next_pos) + 1 >= elevation {
                queue.push_back((next_pos, steps + 1));
            }
        }
        if pos.y < heightmap.height - 1 {
            let next_pos = Pos::new(pos.x, pos.y + 1);
            if heightmap.get(next_pos) + 1 >= elevation {
                queue.push_back((next_pos, steps + 1));
            }
        }
    }
    panic!()
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let heightmap = HeightMap::parse(&puzzle_input);
    println!("{}", part1(&heightmap));
    println!("{}", part2(&heightmap));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&HeightMap::parse(EX)), 31);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&HeightMap::parse(EX)), 29);
    }
}
