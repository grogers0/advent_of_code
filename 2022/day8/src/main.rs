use std::cmp::max;
use std::io::{self, Read};
use std::ops::Index;

struct Grid {
    len_x: usize,
    len_y: usize,
    data: Vec<u8>,
}

impl Index<(usize, usize)> for Grid {
    type Output = u8;
    fn index(&self, (x, y): (usize, usize)) -> &u8 {
        &self.data[y * self.len_x + x]
    }
}

fn parse(puzzle_input: &str) -> Grid {
    let puzzle_input = puzzle_input.trim_end();
    let len_y = puzzle_input.lines().count();
    let len_x = puzzle_input.lines().next().unwrap().chars().count();
    let mut data = Vec::with_capacity(len_y * len_x);
    for line in puzzle_input.lines() {
        assert_eq!(line.chars().count(), len_x);
        for ch in line.chars() {
            data.push(ch as u8 - '0' as u8);
        }
    }
    Grid { len_x, len_y, data }
}

fn part1(grid: &Grid) -> usize {
    let mut visible = vec![false; grid.len_x * grid.len_y];
    let iterate_visibility = |x, y, curr: &mut u8, visible: &mut [bool]| {
        if grid[(x, y)] > *curr {
            *curr = grid[(x, y)];
            visible[y * grid.len_x + x] = true;
        }
    };
    for y in 0..grid.len_y {
        visible[y * grid.len_x] = true;
        let mut curr = grid[(0, y)];
        for x in 1..grid.len_x-1 {
            iterate_visibility(x, y, &mut curr, &mut visible);
        }

        visible[y * grid.len_x + grid.len_y - 1] = true;
        let mut curr = grid[(grid.len_x - 1, y)];
        for x in (1..grid.len_x - 1).rev() {
            iterate_visibility(x, y, &mut curr, &mut visible);
        }
    }
    for x in 0..grid.len_x {
        visible[x] = true;
        let mut curr = grid[(x, 0)];
        for y in 1..grid.len_y-1 {
            iterate_visibility(x, y, &mut curr, &mut visible);
        }

        visible[(grid.len_y - 1) * grid.len_x + x] = true;
        let mut curr = grid[(x, grid.len_y - 1)];
        for y in (1..grid.len_y - 1).rev() {
            iterate_visibility(x, y, &mut curr, &mut visible);
        }
    }

    visible.iter().filter(|&&v| v).count()
}

fn sight_dist_left(mut x: usize, y: usize, grid: &Grid) -> usize {
    let height = grid[(x, y)];
    let mut dist = 0;
    while x > 0 {
        x -= 1;
        dist += 1;
        if grid[(x, y)] >= height { break }
    }
    dist
}

fn sight_dist_right(mut x: usize, y: usize, grid: &Grid) -> usize {
    let height = grid[(x, y)];
    let mut dist = 0;
    while x < grid.len_x - 1 {
        x += 1;
        dist += 1;
        if grid[(x, y)] >= height { break }
    }
    dist
}

fn sight_dist_up(x: usize, mut y: usize, grid: &Grid) -> usize {
    let height = grid[(x, y)];
    let mut dist = 0;
    while y > 0 {
        y -= 1;
        dist += 1;
        if grid[(x, y)] >= height { break }
    }
    dist
}

fn sight_dist_down(x: usize, mut y: usize, grid: &Grid) -> usize {
    let height = grid[(x, y)];
    let mut dist = 0;
    while y < grid.len_y - 1 {
        y += 1;
        dist += 1;
        if grid[(x, y)] >= height { break }
    }
    dist
}

fn part2(grid: &Grid) -> usize {
    let mut best = 0;
    for y in 0..grid.len_y {
        for x in 0..grid.len_x {
            let curr = sight_dist_left(x, y, grid) *
                sight_dist_right(x, y, grid) *
                sight_dist_up(x, y, grid) *
                sight_dist_down(x, y, grid);
            best = max(best, curr);
        }
    }
    best
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

    const EX: &str = "30373
25512
65332
33549
35390
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX)), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EX)), 8);
    }
}
