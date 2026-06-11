use std::io::{self, Read};

struct Map {
    width: usize,
    height: usize,
    startx: usize,
    // Where the splitters are
    grid: Vec<bool>,
}

impl Map {
    fn idx(&self, x: usize, y: usize) -> usize {
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);
        y * self.width + x
    }

    fn at(&self, x: usize, y: usize) -> bool {
        self.grid[self.idx(x, y)]
    }
}

fn parse(puzzle_input: &str) -> Map {
    let height = puzzle_input.lines().count();
    let width = puzzle_input.lines().next().unwrap().chars().count();
    let mut startx = width;
    let mut grid = Vec::with_capacity(width * height);
    for (y, line) in puzzle_input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            grid.push(match ch {
                '.' => false,
                '^' => true,
                'S' => {
                    assert_eq!(y, 0); // Assume starty == 0
                    startx = x;
                    false
                },
                _ => panic!(),
            });
        }
    }
    assert!(startx < width);
    assert_eq!(grid.len(), width * height);
    Map { width, height, startx, grid }
}

fn part1(map: &Map) -> usize {
    let mut split_cnt = 0;
    let mut step_beams = |y: usize, old_beams: Vec<bool>| -> Vec<bool> {
        let mut beams = vec![false; map.width];
        for x in 0..map.width {
            if !old_beams[x] { continue }
            if map.at(x, y) {
                // Splitter
                if x > 0             { beams[x - 1] = true; }
                if x < map.width - 1 { beams[x + 1] = true; }
                split_cnt += 1;
            } else {
                // Straight down
                beams[x] = true;
            }
        }
        beams
    };
    let mut beams = vec![false; map.width];
    beams[map.startx] = true;
    for y in 1..map.height {
        beams = step_beams(y, beams);
    }
    split_cnt
}

fn part2(map: &Map) -> usize {
    let mut split_cnt = 0;
    let mut step_beams = |y: usize, old_beams: Vec<usize>| -> Vec<usize> {
        let mut beams = vec![0; map.width];
        for x in 0..map.width {
            if map.at(x, y) {
                // Splitter
                if x > 0             { beams[x - 1] += old_beams[x]; }
                if x < map.width - 1 { beams[x + 1] += old_beams[x]; }
                split_cnt += 1;
            } else {
                // Straight down
                beams[x] += old_beams[x];
            }
        }
        beams
    };
    let mut beams = vec![0; map.width];
    beams[map.startx] = 1;
    for y in 1..map.height {
        beams = step_beams(y, beams);
    }
    beams.iter().sum()
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

    const EX: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX)), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EX)), 40);
    }
}
