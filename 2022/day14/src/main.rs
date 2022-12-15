use std::cmp::{max, min};
use std::collections::HashSet;
use std::io::{self, Read};

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Pos {
        Pos { x, y }
    }

    fn parse(pos_str: &str) -> Pos {
        let mut it = pos_str.split(",");
        let x = it.next().unwrap().parse().unwrap();
        let y = it.next().unwrap().parse().unwrap();
        assert!(it.next().is_none());
        Pos { x, y }
    }
}

// Sparse map of filled squares
fn parse(puzzle_input: &str) -> (HashSet<Pos>, usize) {
    let mut grid = HashSet::new();
    let mut max_y = 0;
    for line in puzzle_input.trim_end().lines() {
        let mut last_pos: Option<Pos> = None;
        for pos_str in line.split(" -> ") {
            let next_pos = Pos::parse(pos_str);
            max_y = max(max_y, next_pos.y);
            if let Some(last_pos) = last_pos {
                if last_pos.x == next_pos.x {
                    let first_y = min(last_pos.y, next_pos.y);
                    let last_y = max(last_pos.y, next_pos.y);
                    for y in first_y..=last_y {
                        grid.insert(Pos::new(last_pos.x, y));
                    }
                } else if last_pos.y == next_pos.y {
                    let first_x = min(last_pos.x, next_pos.x);
                    let last_x = max(last_pos.x, next_pos.x);
                    for x in first_x..=last_x {
                        grid.insert(Pos::new(x, last_pos.y));
                    }
                } else {
                    panic!()
                }
            }
            last_pos = Some(next_pos);
        }
    }
    (grid, max_y)
}

fn simulate(mut grid: HashSet<Pos>, max_y: usize, has_floor: bool) -> usize {
    let mut cnt = 0;
    loop {
        let mut pos = Pos::new(500, 0);
        loop {
            if has_floor {
                if pos.y == max_y + 1 {
                    grid.insert(pos);
                    break;
                }
            } else if pos.y > max_y {
                return cnt
            }

            if !grid.contains(&Pos::new(pos.x, pos.y + 1)) {
                pos = Pos::new(pos.x, pos.y + 1);
            } else if !grid.contains(&Pos::new(pos.x - 1, pos.y + 1)) {
                pos = Pos::new(pos.x - 1, pos.y + 1);
            } else if !grid.contains(&Pos::new(pos.x + 1, pos.y + 1)) {
                pos = Pos::new(pos.x + 1, pos.y + 1);
            } else if has_floor && pos == Pos::new(500, 0) {
                return cnt + 1;
            } else {
                grid.insert(pos);
                break;
            }
        }
        cnt += 1;
    }
}

fn part1(grid: HashSet<Pos>, max_y: usize) -> usize {
    simulate(grid, max_y, false)
}

fn part2(grid: HashSet<Pos>, max_y: usize) -> usize {
    simulate(grid, max_y, true)
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let (grid, max_y) = parse(&puzzle_input);
    println!("{}", part1(grid.clone(), max_y));
    println!("{}", part2(grid, max_y));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";

    #[test]
    fn test_part1() {
        let (grid, max_y) = parse(EX);
        assert_eq!(part1(grid, max_y), 24);
    }

    #[test]
    fn test_part2() {
        let (grid, max_y) = parse(EX);
        assert_eq!(part2(grid, max_y), 93);
    }
}
