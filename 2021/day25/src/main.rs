use std::collections::HashSet;
use std::io::{self, Read};

struct Grid {
    height: usize,
    width: usize,
    east: HashSet<(usize, usize)>,
    south: HashSet<(usize, usize)>,
}

impl Grid {
    fn parse(puzzle_input: &str) -> Grid {
        let height = puzzle_input.lines().count();
        let width = puzzle_input.lines().next().unwrap().len();
        let mut east = HashSet::new();
        let mut south = HashSet::new();
        for (y, line) in puzzle_input.lines().enumerate() {
            assert_eq!(width, line.len());
            for (x, ch) in line.chars().enumerate() {
                match ch {
                    '>' => { east.insert((x, y)); },
                    'v' => { south.insert((x, y)); },
                    '.' => (),
                    _ => panic!(),
                }
            }
        }
        Grid { height, width, east, south }
    }

    fn step(self) -> (bool, Self) {
        let height = self.height;
        let width = self.width;
        let mut east = HashSet::new();
        let mut south = HashSet::new();
        let mut changed = false;
        for (x, y) in self.east.iter().copied() {
            let x1 = (x + 1) % width;
            if self.east.contains(&(x1, y)) || self.south.contains(&(x1, y)) {
                east.insert((x, y));
            } else {
                changed = true;
                east.insert((x1, y));
            }
        }
        for (x, y) in self.south.iter().copied() {
            let y1 = (y + 1) % height;
            if east.contains(&(x, y1)) || self.south.contains(&(x, y1)) {
                south.insert((x, y));
            } else {
                changed = true;
                south.insert((x, y1));
            }
        }
        (changed, Self { height, width, east, south })
    }
}

fn part1(puzzle_input: &str) -> usize {
    let mut grid = Grid::parse(puzzle_input);
    for i in 1.. {
        let (changed, grid2) = grid.step();
        if !changed { return i }
        grid = grid2;
    }
    unreachable!();
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    println!("{}", part1(&puzzle_input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

    #[test]
    fn test_part1() {
        assert_eq!(58, part1(EX));
    }
}
