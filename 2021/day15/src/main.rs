use std::cmp::Ordering;
use std::collections::{HashSet, BinaryHeap};
use std::io::{self, Read};

struct Grid {
    width: usize,
    height: usize,
    data: Vec<u32>
}

impl Grid {
    fn idx(&self, x: usize, y: usize) -> usize {
        assert!(x < self.width && y < self.height);
        y * self.width + x
    }

    fn at(&self, x: usize, y: usize) -> u32 {
        self.data[self.idx(x, y)]
    }

    fn at_mut(&mut self, x: usize, y: usize) -> &mut u32 {
        let i = self.idx(x, y);
        &mut self.data[i]
    }
}

fn parse(puzzle_input: &str) -> Grid {
    let height = puzzle_input.lines().count();
    let width = puzzle_input.lines().next().unwrap().len();
    let data: Vec<u32> = puzzle_input.lines()
        .flat_map(|line| {
            assert_eq!(width, line.len());
            line.chars()
        })
        .map(|ch| ch.to_digit(10).unwrap())
        .collect();
    assert_eq!(data.len(), width * height);
    Grid { width, height, data }
}

fn find_best_path(grid: &Grid) -> u32 {
    #[derive(Eq, PartialEq)]
    struct AggPath { x: usize, y: usize, risk: u32 }
    impl Ord for AggPath {
        fn cmp(&self, other: &Self) -> Ordering {
            other.risk.cmp(&self.risk) // reverse ordering to make a min-heap
        }
    }
    impl PartialOrd for AggPath {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    let mut heap = BinaryHeap::new();
    heap.push(AggPath { x: 0, y: 0, risk: 0 });
    let mut seen = HashSet::new();
    while let Some(AggPath { x, y, risk }) = heap.pop() {
        if !seen.insert((x, y)) { continue }
        if x == grid.width - 1 && y == grid.height - 1 {
            return risk
        }
        let mut push_heap = |x, y| {
            heap.push(AggPath { x, y, risk: risk + grid.at(x, y) });
        };
        if x > 0               { push_heap(x - 1, y) }
        if x < grid.width - 1  { push_heap(x + 1, y) }
        if y > 0               { push_heap(x, y - 1) }
        if y < grid.height - 1 { push_heap(x, y + 1) }
    }
    unreachable!()
}

fn part1(puzzle_input: &str) -> u32 {
    let grid = parse(puzzle_input);
    find_best_path(&grid)
}

fn part2(puzzle_input: &str) -> u32 {
    let orig_grid = parse(puzzle_input);
    let add_risk = |a, b, c| { ((a + b + c - 1) % 9) + 1 };
    let width = orig_grid.width * 5;
    let height = orig_grid.height * 5;
    let mut grid = Grid { width, height, data: vec![0; width * height] };
    for y_rep in 0..5 {
        for x_rep in 0..5 {
            for y in 0..orig_grid.height {
                for x in 0..orig_grid.width {
                    let y_new = y_rep * orig_grid.height + y;
                    let x_new = x_rep * orig_grid.width + x;
                    *grid.at_mut(x_new, y_new) =
                        add_risk(y_rep as u32, x_rep as u32, orig_grid.at(x, y));
                }
            }
        }
    }
    find_best_path(&grid)
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    println!("{}", part1(&puzzle_input));
    println!("{}", part2(&puzzle_input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn test_part1() {
        assert_eq!(40, part1(EX));
    }

    #[test]
    fn test_part2() {
        assert_eq!(315, part2(EX));
    }
}
