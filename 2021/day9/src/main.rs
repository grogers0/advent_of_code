use std::collections::{HashSet, VecDeque};
use std::io::{self, Read};

struct HeightMap {
    width: usize,
    height: usize,
    data: Vec<u32>
}

impl HeightMap {
    fn at(&self, x: usize, y: usize) -> u32 {
        self.data[y * self.width + x]
    }

    fn is_low_point(&self, x: usize, y: usize) -> bool {
        let val = self.at(x, y);
        if x > 0               && val >= self.at(x - 1, y) { return false }
        if x < self.width - 1  && val >= self.at(x + 1, y) { return false }
        if y > 0               && val >= self.at(x, y - 1) { return false }
        if y < self.height - 1 && val >= self.at(x, y + 1) { return false }
        true
    }

    fn basin_size(&self, startx: usize, starty: usize) -> usize {
        let mut basin_points = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((startx, starty));

        while let Some((x, y)) = queue.pop_front() {
            let val = self.at(x, y);
            if val == 9 { continue } // hightest height
            if !basin_points.insert((x, y)) { continue }

            if x > 0               && val < self.at(x - 1, y) { queue.push_back((x - 1, y)) }
            if x < self.width - 1  && val < self.at(x + 1, y) { queue.push_back((x + 1, y)) }
            if y > 0               && val < self.at(x, y - 1) { queue.push_back((x, y - 1)) }
            if y < self.height - 1 && val < self.at(x, y + 1) { queue.push_back((x, y + 1)) }
        }

        basin_points.len()
    }
}

fn parse(puzzle_input: &str) -> HeightMap {
    let width = puzzle_input.lines().next().unwrap().len();
    let height = puzzle_input.lines().count();
    let mut data = Vec::new();
    for line in puzzle_input.lines() {
        for ch in line.chars() {
            data.push(ch.to_digit(10).unwrap());
        }
    }
    assert_eq!(data.len(), width * height);
    HeightMap { width, height, data }
}

fn part1(heightmap: &HeightMap) -> u32 {
    let mut sum = 0;
    for y in 0..heightmap.height {
        for x in 0..heightmap.width {
            if heightmap.is_low_point(x, y) {
                sum += heightmap.at(x, y) + 1;
            }
        }
    }
    sum
}

fn part2(heightmap: &HeightMap) -> usize {
    let mut basin_sizes = Vec::new();
    for y in 0..heightmap.height {
        for x in 0..heightmap.width {
            if heightmap.is_low_point(x, y) {
                basin_sizes.push(heightmap.basin_size(x, y));
            }
        }
    }
    basin_sizes.sort_unstable();
    let n = basin_sizes.len();
    basin_sizes[n - 3] * basin_sizes[n - 2] * basin_sizes[n - 1]
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let heightmap = parse(&puzzle_input);
    println!("{}", part1(&heightmap));
    println!("{}", part2(&heightmap));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test_part1() {
        assert_eq!(15, part1(&parse(EX)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1134, part2(&parse(EX)));
    }
}
