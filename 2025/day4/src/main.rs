use std::io::{self, Read};

#[derive(Clone)]
struct Map {
    width: usize,
    height: usize,
    grid: Vec<bool>,
}

impl Map {
    fn at(&self, x: usize, y: usize) -> bool {
        debug_assert!(x < self.width && y < self.height);
        self.grid[y * self.width + x]
    }

    fn count_adjacent(&self, x: usize, y: usize) -> usize {
        let mut cnt = 0;
        if x > 0              && y > 0               && self.at(x - 1, y - 1) { cnt += 1; }
        if                       y > 0               && self.at(x    , y - 1) { cnt += 1; }
        if x < self.width - 1 && y > 0               && self.at(x + 1, y - 1) { cnt += 1; }
        if x > 0                                     && self.at(x - 1, y    ) { cnt += 1; }
        if x < self.width - 1                        && self.at(x + 1, y    ) { cnt += 1; }
        if x > 0              && y < self.height - 1 && self.at(x - 1, y + 1) { cnt += 1; }
        if                       y < self.height - 1 && self.at(x    , y + 1) { cnt += 1; }
        if x < self.width - 1 && y < self.height - 1 && self.at(x + 1, y + 1) { cnt += 1; }
        cnt
    }

    fn is_accessible(&self, x: usize, y: usize) -> bool {
        self.count_adjacent(x, y) < 4
    }

    fn remove(&mut self, x: usize, y: usize) {
        debug_assert!(x < self.width && y < self.height);
        self.grid[y * self.width + x] = false;
    }
}

fn parse(puzzle_input: &str) -> Map {
    let height = puzzle_input.lines().count();
    let width = puzzle_input.lines().next().unwrap().chars().count();
    let mut grid = Vec::with_capacity(width * height);
    for line in puzzle_input.lines() {
        assert_eq!(width, line.chars().count());
        for ch in line.chars() {
            grid.push(match ch {
                '@' => true,
                '.' => false,
                _ => panic!(),
            });
        }
    }
    Map { width, height, grid }
}

fn part1(map: &Map) -> usize {
    let mut cnt = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            if map.at(x, y) && map.is_accessible(x, y) {
                cnt += 1;
            }
        }
    }
    cnt
}

fn part2(mut map: Map) -> usize {
    let mut cnt = 0;
    // Could do BFS if needed but it's good enough
    loop {
        let mut any_changed = false;
        for y in 0..map.height {
            for x in 0..map.width {
                if map.at(x, y) && map.is_accessible(x, y) {
                    map.remove(x, y);
                    cnt += 1;
                    any_changed = true;
                }
            }
        }
        if !any_changed { break; }
    }
    cnt
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let map = parse(&puzzle_input);
    println!("{}", part1(&map));
    println!("{}", part2(map.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX)), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse(EX)), 43);
    }
}
