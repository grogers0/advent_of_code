use std::io::{self, Read};

struct Map {
    width: usize,
    height: usize,
    trees: Vec<bool>
}

impl Map {
    fn parse(puzzle_input: &str) -> Map {
        let height = puzzle_input.lines().count();
        let width = puzzle_input.lines().next().unwrap().chars().count();
        let mut trees = Vec::with_capacity(width * height);
        for line in puzzle_input.lines() {
            assert_eq!(width, line.chars().count());
            for ch in line.chars() {
                let is_tree = match ch {
                    '.' => false,
                    '#' => true,
                    _ => panic!()
                };
                trees.push(is_tree);
            }
        }
        assert_eq!(height * width, trees.len());
        Map { width: width, height: height, trees: trees }
    }

    fn tree_at(&self, x: usize, y: usize) -> bool {
        if y >= self.height { panic!() }
        self.trees[(y * self.width) + (x % self.width)]
    }

    fn trees_on_slope(&self, x_offset: usize, y_offset: usize) -> usize {
        (0 .. (self.height / y_offset)).filter(|&i| self.tree_at(i*x_offset, i*y_offset)).count()
    }
}

fn part1(map: &Map) -> usize {
    map.trees_on_slope(3, 1)
}

fn part2(map: &Map) -> usize {
    map.trees_on_slope(1, 1) *
        map.trees_on_slope(3, 1) *
        map.trees_on_slope(5, 1) *
        map.trees_on_slope(7, 1) *
        map.trees_on_slope(1, 2)
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();
    let map = Map::parse(&puzzle_input);

    println!("{}", part1(&map));
    println!("{}", part2(&map));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn test_part1() {
        assert_eq!(7, part1(&Map::parse(EX)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(336, part2(&Map::parse(EX)));
    }
}
