use std::collections::HashSet;
use std::io::{self, Read};

#[derive(Clone)]
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

    fn get(&self, x: usize, y: usize) -> u32 {
        self.data[self.idx(x, y)]
    }

    fn get_mut(&mut self, x: usize, y: usize) -> &mut u32 {
        let i = self.idx(x, y);
        &mut self.data[i]
    }
}

fn parse(puzzle_input: &str) -> Grid {
    let height = puzzle_input.lines().count();
    let width = puzzle_input.lines().next().unwrap().len();
    let data: Vec<_> = puzzle_input.lines()
        .flat_map(|line| {
            assert_eq!(width, line.len());
            line.chars().map(|ch| ch.to_digit(10).unwrap())
        }).collect();
    assert_eq!(height * width, data.len());
    Grid { width, height, data }
}

// returns how many flashes this step
fn step(grid: &mut Grid) -> usize {
    let mut flashed = HashSet::new();
    fn check_flash(grid: &mut Grid, flashed: &mut HashSet<(usize, usize)>, x: usize, y: usize) {
        if grid.get(x, y) <= 9 { return; } // no flash
        if !flashed.insert((x, y)) { return; } // already flashed
        let min_x = if x > 0               { x - 1 } else { 0 };
        let max_x = if x < grid.width - 1  { x + 1 } else { grid.width - 1 };
        let min_y = if y > 0               { y - 1 } else { 0 };
        let max_y = if y < grid.height - 1 { y + 1 } else { grid.height - 1 };
        for y in min_y .. max_y + 1 {
            for x in min_x .. max_x + 1 {
                *grid.get_mut(x, y) += 1;
                check_flash(grid, flashed, x, y);
            }
        }
    }

    for y in 0..grid.height {
        for x in 0..grid.width {
            *grid.get_mut(x, y) += 1;
            check_flash(grid, &mut flashed, x, y);
        }
    }

    for &(x, y) in flashed.iter() {
        *grid.get_mut(x, y) = 0;
    }

    flashed.len()
}

fn part1(grid: &Grid) -> usize {
    let mut grid = grid.clone();
    let mut sum = 0;
    for _ in 0..100 {
        sum += step(&mut grid);
    }
    sum
}

fn part2(grid: &Grid) -> usize {
    let mut grid = grid.clone();
    for i in 1.. {
        if step(&mut grid) == grid.data.len() {
            return i
        }
    }
    unreachable!()
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let grid = parse(&puzzle_input);
    println!("{}", part1(&grid));
    println!("{}", part2(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn test_part1() {
        assert_eq!(1656, part1(&parse(EX)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(195, part2(&parse(EX)));
    }
}
