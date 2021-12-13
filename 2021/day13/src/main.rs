use std::fmt::Write;
use std::io::{self, Read};

#[derive(Copy, Clone)]
enum Fold {
    X(usize),
    Y(usize)
}

struct Grid {
    width: usize,
    height: usize,
    dots: Vec<bool>
}

impl Grid {
    fn from_coords(coords: &[(usize, usize)]) -> Grid {
        let width = 1 + *coords.iter().map(|(x, _)| x).max().unwrap();
        let height = 1 + *coords.iter().map(|(_, y)| y).max().unwrap();
        let mut dots = vec![false; width * height];
        for &(x, y) in coords {
            dots[width * y + x] = true;
        }
        Grid { width, height, dots }
    }

    fn fold(&self, fold: Fold) -> Grid {
        let (width, height) = match fold {
            Fold::X(fold_x) => {
                assert!(self.width - 1 == fold_x * 2);
                assert!((0..self.height).all(|y| !self.dots[self.width * y + fold_x]));
                (fold_x, self.height)
            },
            Fold::Y(fold_y) => {
                assert!(self.height - 1 == fold_y * 2);
                assert!((0..self.width).all(|x| !self.dots[self.width * fold_y + x]));
                (self.width, fold_y)
            }
        };
        let mut dots = vec![false; width * height];
        match fold {
            Fold::X(fold_x) => {
                for y in 0..height {
                    for x in 0..fold_x {
                        dots[width * y + x] |= self.dots[self.width * y + x];
                    }
                    for x in fold_x+1..self.width {
                        dots[width * y + (self.width - x - 1)] |= self.dots[self.width * y + x];
                    }
                }
            },
            Fold::Y(fold_y) => {
                for x in 0..width {
                    for y in 0..fold_y {
                        dots[width * y + x] |= self.dots[self.width * y + x];
                    }
                    for y in fold_y+1..self.height {
                        dots[width * (self.height - y - 1) + x] |= self.dots[self.width * y + x];
                    }
                }
            }
        };
        Grid { width, height, dots }
    }
}

fn parse(puzzle_input: &str) -> (Vec<(usize, usize)>, Vec<Fold>) {
    let mut split_iter = puzzle_input.split("\n\n");
    let coords = split_iter.next().unwrap().lines()
        .map(|coords_str| {
            let mut split_iter = coords_str.split(",");
            let x = split_iter.next().unwrap().parse().unwrap();
            let y = split_iter.next().unwrap().parse().unwrap();
            assert!(split_iter.next().is_none());
            (x, y)
        })
        .collect();
    let folds = split_iter.next().unwrap().lines()
        .map(|fold_inst| {
            if fold_inst.starts_with("fold along x=") {
                Fold::X(fold_inst[13..].parse().unwrap())
            } else if fold_inst.starts_with("fold along y=") {
                Fold::Y(fold_inst[13..].parse().unwrap())
            } else {
                panic!()
            }
        })
        .collect();
    assert!(split_iter.next().is_none());
    (coords, folds)
}

fn part1(coords: &[(usize, usize)], folds: &[Fold]) -> usize {
    let grid = Grid::from_coords(coords);
    let grid = grid.fold(folds[0]);
    grid.dots.iter().filter(|&&dot| dot).count()
}

fn part2(coords: &[(usize, usize)], folds: &[Fold]) -> String {
    let mut grid = Grid::from_coords(coords);
    for fold in folds {
        grid = grid.fold(*fold);
    }

    let mut s = String::new();
    for y in 0..grid.height {
        for x in 0..grid.width {
            write!(s, "{}", if grid.dots[y * grid.width + x] { '#' } else { '.' });
        }
            write!(s, "\n");
    }
    ascii_bitmap::decode(&s).unwrap()
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let (coords, folds) = parse(&puzzle_input);
    println!("{}", part1(&coords, &folds));
    println!("{}", part2(&coords, &folds));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn test_part1() {
        let (coords, folds) = parse(&EX);
        assert_eq!(17, part1(&coords, &folds));
    }
}
