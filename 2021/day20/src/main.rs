use std::cmp::{min, max};
use std::collections::HashSet;
use std::fmt;
use std::io::{self, Read};

struct Image {
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
    lit_infinite: bool,
    lit_pixels: HashSet<[isize; 2]>, // [x, y]
}

impl Image {
    fn parse(s: &str) -> Image {
        let width = s.lines().next().unwrap().len();
        let mut lit_pixels = HashSet::new();
        let mut x_min = isize::MAX;
        let mut x_max = isize::MIN;
        let mut y_min = isize::MAX;
        let mut y_max = isize::MIN;
        for (y, line) in s.lines().enumerate() {
            let y = y as isize;
            assert_eq!(width, line.len());
            for (x, ch) in line.chars().enumerate() {
                let lit = match ch {
                    '#' => true,
                    '.' => false,
                    _ => panic!(),
                };
                if lit {
                    let x = x as isize;
                    x_min = min(x_min, x);
                    x_max = max(x_max, x);
                    y_min = min(y_min, y);
                    y_max = max(y_max, y);
                    lit_pixels.insert([x, y]);
                }
            }
        }
        Image { x_min, x_max, y_min, y_max, lit_infinite: false, lit_pixels }
    }

    fn enhance_pixel(&self, algorithm: &[bool], x: isize, y: isize) -> bool {
        let mut idx = 0;
        for y in y-1 ..= y+1 {
            for x in x-1 ..= x+1 {
                let lit = if x < self.x_min || y < self.y_min || x > self.x_max || y > self.y_max {
                    self.lit_infinite
                } else {
                    self.lit_pixels.contains(&[x, y])
                };

                idx <<= 1;
                if lit {
                    idx |= 1;
                }
            }
        }
        algorithm[idx]
    }

    fn enhance(&self, algorithm: &[bool]) -> Image {
        let mut lit_pixels = HashSet::new();
        let mut x_min = isize::MAX;
        let mut x_max = isize::MIN;
        let mut y_min = isize::MAX;
        let mut y_max = isize::MIN;
        for y in self.y_min-2 ..= self.y_max+2 {
            for x in self.x_min-2 ..= self.x_max+2 {
                if self.enhance_pixel(algorithm, x, y) {
                    x_min = min(x_min, x);
                    x_max = max(x_max, x);
                    y_min = min(y_min, y);
                    y_max = max(y_max, y);
                    lit_pixels.insert([x, y]);
                }
            }
        }
        let lit_infinite = algorithm[if self.lit_infinite { 255 } else { 0 }];
        Image { x_min, x_max, y_min, y_max, lit_infinite, lit_pixels }
    }
}

#[allow(dead_code)]
impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first_line = true;
        let scrolloff = 2;
        for y in self.y_min-scrolloff ..= self.y_max+scrolloff {
            if first_line {
                first_line = false;
            } else {
                write!(f, "\n")?;
            }
            for x in self.x_min-scrolloff ..= self.x_max+scrolloff {
                let lit = if x < self.x_min || y < self.y_min || x > self.x_max || y > self.y_max {
                    self.lit_infinite
                } else {
                    self.lit_pixels.contains(&[x, y])
                };

                write!(f, "{}", if lit { '#' } else { '.' })?;
            }
        }
        Ok(())
    }
}

fn parse(puzzle_input: &str) -> (Vec<bool>, Image) {
    let mut split_iter = puzzle_input.split("\n\n");
    let algorithm: Vec<_> = split_iter.next().unwrap().chars()
        .map(|ch| match ch {
            '#' => true,
            '.' => false,
            _ => panic!(),
        })
        .collect();
    assert_eq!(512, algorithm.len());
    let image = Image::parse(split_iter.next().unwrap());
    assert!(split_iter.next().is_none());
    (algorithm, image)
}

fn part1(puzzle_input: &str) -> usize {
    let (algorithm, mut image) = parse(puzzle_input);
    for _ in 0..2 {
        image = image.enhance(&algorithm);
    }
    image.lit_pixels.len()
}

fn part2(puzzle_input: &str) -> usize {
    let (algorithm, mut image) = parse(puzzle_input);
    for _ in 0..50 {
        image = image.enhance(&algorithm);
    }
    image.lit_pixels.len()
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

    const EX: &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

    #[test]
    fn test_part1() {
        assert_eq!(35, part1(EX));
    }

    #[test]
    fn test_part2() {
        assert_eq!(3351, part2(EX));
    }
}
