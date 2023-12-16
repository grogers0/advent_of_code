use std::collections::VecDeque;
use std::io::{self, Read};

enum Dir {
    North,
    South,
    East,
    West,
}

#[derive(Copy, Clone)]
enum Tile {
    VerticalPipe, // |
    HorizontalPipe, // -
    NorthToEastPipe, // L
    NorthToWestPipe, // J
    SouthToWestPipe, // 7
    SouthToEastPipe, // F
    Ground, // .
    StartingPosition, // S
}

struct Map {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl Map {
    fn idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
    fn at(&self, x: usize, y: usize) -> Tile {
        self.tiles[self.idx(x, y)]
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
                '|' => Tile::VerticalPipe,
                '-' => Tile::HorizontalPipe,
                'L' => Tile::NorthToEastPipe,
                'J' => Tile::NorthToWestPipe,
                '7' => Tile::SouthToWestPipe,
                'F' => Tile::SouthToEastPipe,
                '.' => Tile::Ground,
                'S' => Tile::StartingPosition,
                _ => panic!(),
            };
            tiles.push(tile);
        }
    }
    Map { width, height, tiles }
}

fn get_starting_position(map: &Map) -> (usize, usize) {
    for y in 0..map.height {
        for x in 0..map.width {
            if let Tile::StartingPosition = map.at(x, y) {
                return (x, y);
            }
        }
    }
    panic!()
}

fn part1(map: &Map) -> usize {
    let (start_x, start_y) = get_starting_position(map);
    let mut distances = vec![0; map.width * map.height];
    let mut deque = VecDeque::new();
    if start_x > 0              { deque.push_back((start_x - 1, start_y, 1, Dir::East)); }
    if start_x < map.width - 1  { deque.push_back((start_x + 1, start_y, 1, Dir::West)); }
    if start_y > 0              { deque.push_back((start_x, start_y - 1, 1, Dir::South)); }
    if start_y < map.height - 1 { deque.push_back((start_x, start_y + 1, 1, Dir::North)); }
    let mut best_d = 0;
    let mut best_x = 0;
    let mut best_y = 0;
    let mut best_match = 0;
    while let Some((x, y, dist, dir)) = deque.pop_front() {
        if x == start_x && y == start_y { continue }

        let mut allows_north = false;
        let mut allows_south = false;
        let mut allows_east = false;
        let mut allows_west = false;
        match map.at(x, y) {
            Tile::VerticalPipe =>     { allows_north = true; allows_south = true; },
            Tile::HorizontalPipe =>   { allows_east = true; allows_west = true; },
            Tile::NorthToEastPipe =>  { allows_north = true; allows_east = true; },
            Tile::NorthToWestPipe =>  { allows_north = true; allows_west = true; },
            Tile::SouthToWestPipe =>  { allows_south = true; allows_west = true; },
            Tile::SouthToEastPipe =>  { allows_south = true; allows_east = true; },
            Tile::Ground => (),
            Tile::StartingPosition => (),
        };
        let allows_from = match dir {
            Dir::North => allows_north,
            Dir::South => allows_south,
            Dir::East  => allows_east,
            Dir::West  => allows_west,
        };
        if !allows_from { continue }
        if x == best_x && y == best_y && dist == best_d {
            // Matched from the other side
            best_match = dist;
        }
        if distances[map.idx(x, y)] != 0 { continue }
        if dist > best_d {
            best_d = dist;
            best_x = x;
            best_y = y;
        }
        distances[map.idx(x, y)] = dist;

        if allows_north && y > 0              { deque.push_back((x, y - 1, dist + 1, Dir::South)); }
        if allows_south && y < map.height - 1 { deque.push_back((x, y + 1, dist + 1, Dir::North)); }
        if allows_west  && x > 0              { deque.push_back((x - 1, y, dist + 1, Dir::East)); }
        if allows_east  && x < map.width - 1  { deque.push_back((x + 1, y, dist + 1, Dir::West)); }
    }
    best_match
}

fn part2(puzzle_input: &str) -> &str {
    "FIXME"
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let map = parse(&puzzle_input);
    println!("{}", part1(&map));
    println!("{}", part2(&puzzle_input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

    const EX2: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";


    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX1)), 4);
        assert_eq!(part1(&parse(EX2)), 8);
    }

    #[test]
    fn test_part2() {
        // FIXME
    }
}
