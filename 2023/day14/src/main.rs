use std::collections::HashMap;
use std::io::{self, Read};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Tile {
    RoundRock,
    CubeRock,
    Empty,
}

#[derive(Clone)]
struct Map {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl Map {
    fn idx(&self, x: usize, y: usize) -> usize {
        self.width * y + x
    }

    fn at(&self, x: usize, y: usize) -> Tile {
        self.tiles[self.idx(x, y)]
    }

    fn set(&mut self, x: usize, y: usize, tile: Tile) {
        let i = self.idx(x, y);
        self.tiles[i] = tile;
    }
}

fn parse(puzzle_input: &str) -> Map {
    let height = puzzle_input.lines().count();
    let width = puzzle_input.lines().next().unwrap().chars().count();
    let mut tiles = Vec::with_capacity(width * height);
    for line in puzzle_input.lines() {
        assert_eq!(width, line.chars().count());
        for ch in line.chars() {
            let tile = match ch {
                'O' => Tile::RoundRock,
                '#' => Tile::CubeRock,
                '.' => Tile::Empty,
                _ => panic!(),
            };
            tiles.push(tile);
        }
    }
    Map { width, height, tiles }
}

fn shift_rocks_north(map: &mut Map) {
    for y in 0..map.height {
        for x in 0..map.width {
            if let Tile::RoundRock = map.at(x, y) {
                let mut y2 = y;
                while y2 > 0 {
                    if let Tile::Empty = map.at(x, y2-1) {
                        map.set(x, y2, Tile::Empty);
                        map.set(x, y2-1, Tile::RoundRock);
                        y2 -= 1;
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

fn shift_rocks_south(map: &mut Map) {
    for y in (0..map.height).rev() {
        for x in 0..map.width {
            if let Tile::RoundRock = map.at(x, y) {
                let mut y2 = y;
                while y2 < map.height - 1 {
                    if let Tile::Empty = map.at(x, y2+1) {
                        map.set(x, y2, Tile::Empty);
                        map.set(x, y2+1, Tile::RoundRock);
                        y2 += 1;
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

fn shift_rocks_west(map: &mut Map) {
    for y in 0..map.height {
        for x in 0..map.width {
            if let Tile::RoundRock = map.at(x, y) {
                let mut x2 = x;
                while x2 > 0 {
                    if let Tile::Empty = map.at(x2-1, y) {
                        map.set(x2, y, Tile::Empty);
                        map.set(x2-1, y, Tile::RoundRock);
                        x2 -= 1;
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

fn shift_rocks_east(map: &mut Map) {
    for y in 0..map.height {
        for x in (0..map.width).rev() {
            if let Tile::RoundRock = map.at(x, y) {
                let mut x2 = x;
                while x2 < map.width - 1 {
                    if let Tile::Empty = map.at(x2+1, y) {
                        map.set(x2, y, Tile::Empty);
                        map.set(x2+1, y, Tile::RoundRock);
                        x2 += 1;
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

fn shift_rocks_cycle(map: &mut Map) {
    shift_rocks_north(map);
    shift_rocks_west(map);
    shift_rocks_south(map);
    shift_rocks_east(map);
}

#[allow(dead_code)]
fn print_map(map: &Map) -> String {
    let mut s = String::new();
    for y in 0..map.height {
        if y != 0 { s.push('\n'); }
        for x in 0..map.width {
            s.push(match map.at(x, y) {
                Tile::RoundRock => 'O',
                Tile::CubeRock => '#',
                Tile::Empty => '.',
            });
        }
    }
    s
}

fn calc_load(map: &Map) -> usize {
    let mut sum = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            if let Tile::RoundRock = map.at(x, y) {
                sum += map.height - y;
            }
        }
    }
    sum
}

fn part1(mut map: Map) -> usize {
    shift_rocks_north(&mut map);
    calc_load(&map)
}

fn part2(mut map: Map) -> usize {
    let mut saved = HashMap::new();
    const N_ITERS: u32 = 1_000_000_000;
    for i in 0..N_ITERS {
        if let Some(prior_iter) = saved.insert(map.tiles.clone(), i) {
            let j = i + ((N_ITERS - i) / (i - prior_iter) * (i - prior_iter));
            for _ in j..N_ITERS {
                shift_rocks_cycle(&mut map);
            }
            return calc_load(&map);
        }
        shift_rocks_cycle(&mut map);
    }
    panic!()
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let map = parse(&puzzle_input);
    println!("{}", part1(map.clone()));
    println!("{}", part2(map));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse(EX)), 136);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse(EX)), 64);
    }
}
