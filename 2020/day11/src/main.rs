use std::convert::From;
use std::fmt;
use std::io::{self, Read};

#[derive(Copy, Clone, PartialEq)]
enum Cell {
    Floor,
    Seat(bool)
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ch = match self {
            Cell::Floor => '.',
            Cell::Seat(false) => 'L',
            Cell::Seat(true) => '#'
        };
        write!(f, "{}", ch)
    }
}

impl From<char> for Cell {
    fn from(ch: char) -> Cell {
        match ch {
            'L' => Cell::Seat(false),
            '#' => Cell::Seat(true),
            '.' => Cell::Floor,
            _ => panic!()
        }
    }
}

#[derive(Clone, PartialEq)]
struct Map {
    height: usize,
    width: usize,
    cells: Vec<Cell>
}

impl From<&str> for Map {
    fn from(puzzle_input: &str) -> Map {
        let height = puzzle_input.lines().count();
        let width = puzzle_input.lines().next().unwrap().chars().count();
        let mut cells = Vec::new();
        for line in puzzle_input.lines() {
            assert_eq!(width, line.chars().count());
            for ch in line.chars() {
                cells.push(Cell::from(ch));
            }
        }
        Map { height: height, width: width, cells: cells }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            if y > 0 { write!(f, "\n")? }
            for x in 0..self.width {
                write!(f, "{}", self.get(x, y))?
            }
        }
        Ok(())
    }
}

impl Map {
    fn get(&self, x: usize, y: usize) -> Cell {
        assert!(x < self.width);
        assert!(y < self.height);
        self.cells[y*self.width + x]
    }

    fn get_mut(&mut self, x: usize, y: usize) -> &mut Cell {
        assert!(x < self.width);
        assert!(y < self.height);
        &mut self.cells[y*self.width + x]
    }

    fn get_offset(&self, xin: usize, xoff: isize, yin: usize, yoff: isize) -> Option<Cell> {
        let x = xoff + xin as isize;
        let y = yoff + yin as isize;
        if x < 0 || x as usize >= self.width { return None }
        if y < 0 || y as usize >= self.height { return None }
        Some(self.get(x as usize, y as usize))
    }
}

const NEIGHBOR_DIRS: &[(isize, isize)] = &[(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

fn part1(map: &Map) -> usize {
    fn num_occupied_neighbors(map: &Map, x: usize, y: usize) -> usize {
        NEIGHBOR_DIRS.iter()
            .filter(|&&(xoff, yoff)| map.get_offset(x, xoff, y, yoff) == Some(Cell::Seat(true)))
            .count()
    }

    let mut map = map.clone();
    loop {
        let mut next_map = map.clone();
        for y in 0..map.height {
            for x in 0..map.width {
                match (map.get(x, y), num_occupied_neighbors(&map, x, y)) {
                    (Cell::Seat(false), n) if n == 0 => { *next_map.get_mut(x, y) = Cell::Seat(true) },
                    (Cell::Seat(true), n) if n >= 4 => { *next_map.get_mut(x, y) = Cell::Seat(false) },
                    _ => ()
                }
            }
        }
        if map == next_map { break }
        map = next_map;
    }
    map.cells.iter().filter(|&&cell| Cell::Seat(true) == cell).count()
}

fn part2(map: &Map) -> usize {
    fn num_occupied_neighbors(map: &Map, x: usize, y: usize) -> usize {
        NEIGHBOR_DIRS.iter()
            .filter(|&&(xoff, yoff)| {
                for i in 1.. {
                    match map.get_offset(x, i*xoff, y, i*yoff) {
                        Some(Cell::Seat(true)) => return true,
                        Some(Cell::Seat(false)) | None => return false,
                        Some(Cell::Floor) => ()
                    }
                }
                unreachable!()
            })
            .count()
    }

    let mut map = map.clone();
    loop {
        let mut next_map = map.clone();
        for y in 0..map.height {
            for x in 0..map.width {
                match (map.get(x, y), num_occupied_neighbors(&map, x, y)) {
                    (Cell::Seat(false), n) if n == 0 => { *next_map.get_mut(x, y) = Cell::Seat(true) },
                    (Cell::Seat(true), n) if n >= 5 => { *next_map.get_mut(x, y) = Cell::Seat(false) },
                    _ => ()
                }
            }
        }
        if map == next_map { break }
        map = next_map;
    }
    map.cells.iter().filter(|&&cell| Cell::Seat(true) == cell).count()
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();
    let map = Map::from(puzzle_input.as_str());

    println!("{}", part1(&map));
    println!("{}", part2(&map));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn test_part1() {
        assert_eq!(37, part1(&Map::from(EX)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(26, part2(&Map::from(EX)));
    }
}
