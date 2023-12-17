use std::cmp::min;
use std::io::{self, Read};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Reflection {
    Horizontal(usize),
    Vertical(usize),
}

#[derive(Clone)]
struct Map {
    width: usize,
    height: usize,
    rocks: Vec<bool>,
}

impl Map {
    fn idx(&self, x: usize, y: usize) -> usize {
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);
        self.width * y + x
    }

    fn is_rock(&self, x: usize, y: usize) -> bool {
        self.rocks[self.idx(x, y)]
    }

    fn smudge(&mut self, x: usize, y: usize) {
        let idx = self.idx(x, y);
        self.rocks[idx] = !self.rocks[idx];
    }
}

fn parse(puzzle_input: &str) -> Vec<Map> {
    puzzle_input.split("\n\n").map(|paragraph| {
        let height = paragraph.lines().count();
        let width = paragraph.lines().next().unwrap().chars().count();
        let mut rocks = Vec::with_capacity(width * height);
        for line in paragraph.lines() {
            assert_eq!(width, line.chars().count());
            for ch in line.chars() {
                let is_rock = match ch {
                    '#' => true,
                    '.' => false,
                    _ => panic!(),
                };
                rocks.push(is_rock);
            }
        }
        assert_eq!(rocks.len(), width * height);
        Map { width, height, rocks }
    }).collect()
}

fn find_reflections(map: &Map) -> Vec<Reflection> {
    let mut reflections = Vec::new();
    for x in 0..(map.width-1) {
        if (0..min(x+1, map.width-x-1)).all(|i|
            (0..map.height).all(|y|
                map.is_rock(x-i, y) == map.is_rock(x+i+1, y))) {
            reflections.push(Reflection::Vertical(x+1));
        }
    }
    for y in 0..(map.height-1) {
        if (0..min(y+1, map.height-y-1)).all(|i|
            (0..map.width).all(|x|
                map.is_rock(x, y-i) == map.is_rock(x, y+i+1))) {
            reflections.push(Reflection::Horizontal(y+1));
        }
    }
    reflections
}

fn score_reflection(reflection: Reflection) -> usize {
    match reflection {
        Reflection::Vertical(v) => v,
        Reflection::Horizontal(v) => 100*v,
    }
}

#[allow(dead_code)]
fn print_map(map: &Map) -> String {
    let mut s = String::new();
    for y in 0..map.height {
        if y != 0 { s.push('\n'); }
        for x in 0..map.width {
            s.push(if map.is_rock(x, y) { '#' } else { '.' });
        }
    }
    s
}

fn part1(maps: &[Map]) -> usize {
    let mut sum = 0;
    for map in maps {
        let reflections = find_reflections(map);
        assert_eq!(reflections.len(), 1);
        for reflection in reflections {
            sum += score_reflection(reflection);
        }
    }
    sum
}

fn part2(maps: &[Map]) -> usize {
    let mut sum = 0;
    'outer: for map in maps {
        let mut map = map.clone();
        let orig_reflection = find_reflections(&map)[0];

        for y in 0..map.height {
            for x in 0..map.width {
                map.smudge(x, y);
                let mut reflections = find_reflections(&map);
                reflections.retain(|r| *r != orig_reflection);
                if !reflections.is_empty() {
                    assert_eq!(reflections.len(), 1);
                    for &reflection in &reflections {
                        sum += score_reflection(reflection);
                    }
                    continue 'outer;
                }
                map.smudge(x, y); // Reset back to the original
            }
        }
        panic!()
    }
    sum
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let maps = parse(&puzzle_input);
    println!("{}", part1(&maps));
    println!("{}", part2(&maps));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX)), 405);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EX)), 400);
    }
}
