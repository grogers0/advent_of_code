use std::cmp::Reverse;
use std::collections::{HashSet, BinaryHeap};
use std::io::{self, Read};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Dir {
    Up, Down, Left, Right
}

impl Dir {
    fn rotate_left(self) -> Dir {
        match self {
            Dir::Up => Dir::Left,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Down,
            Dir::Right => Dir::Up,
        }
    }
    fn rotate_right(self) -> Dir {
        match self {
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Right => Dir::Down,
        }
    }
}


struct Map {
    width: usize,
    height: usize,
    tiles: Vec<u32>,
}

impl Map {
    fn idx(&self, x: usize, y: usize) -> usize {
        self.width * y + x
    }

    fn at(&self, x: usize, y: usize) -> u32 {
        self.tiles[self.idx(x, y)]
    }

    fn can_step(&self, x: usize, y: usize, dir: Dir) -> bool {
        match dir {
            Dir::Up => y > 0,
            Dir::Down => y < self.height - 1,
            Dir::Left => x > 0,
            Dir::Right => x < self.width - 1,
        }
    }

    fn step(&self, x: usize, y: usize, dir: Dir) -> (usize, usize) {
        debug_assert!(self.can_step(x, y, dir));
        match dir {
            Dir::Up => (x, y-1),
            Dir::Down => (x, y+1),
            Dir::Left => (x-1, y),
            Dir::Right => (x+1, y),
        }
    }
}

fn parse(puzzle_input: &str) -> Map {
    let height = puzzle_input.lines().count();
    let width = puzzle_input.lines().next().unwrap().chars().count();
    let mut tiles = Vec::with_capacity(width * height);
    for line in puzzle_input.lines() {
        assert_eq!(line.chars().count(), width);
        for ch in line.chars() {
            tiles.push(ch.to_digit(10).unwrap());
        }
    }
    Map { width, height, tiles }
}

fn find_best_path(map: &Map, min_consec: usize, max_consec: usize) -> u32 {
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, 0, 0, Dir::Down, 0)));
    queue.push(Reverse((0, 0, 0, Dir::Right, 0)));
    let mut seen = HashSet::new();
    while let Some(Reverse((heat_lost, x, y, dir, consec))) = queue.pop() {
        if !seen.insert((x, y, dir, consec)) {
            continue;
        }
        if x == map.width - 1 && y == map.height - 1 {
            return heat_lost;
        }
        let mut dirs = Vec::new();
        if consec < max_consec { dirs.push(dir); }
        if consec >= min_consec {
            dirs.push(dir.rotate_right());
            dirs.push(dir.rotate_left());
        }
        for next_dir in dirs {
            if !map.can_step(x, y, next_dir) { continue }
            let next_consec = if dir == next_dir { consec + 1 } else { 1 };
            let (next_x, next_y) = map.step(x, y, next_dir);
            queue.push(Reverse((
                        heat_lost + map.at(next_x, next_y),
                        next_x,
                        next_y,
                        next_dir,
                        next_consec)));
        }
    }
    panic!()
}

fn part1(map: &Map) -> u32 {
    find_best_path(map, 0, 3)
}

fn part2(map: &Map) -> u32 {
    find_best_path(map, 4, 10)
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

    const EX: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";


    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX)), 102);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EX)), 94);
    }
}
