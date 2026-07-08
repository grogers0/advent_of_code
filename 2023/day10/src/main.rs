use std::collections::{VecDeque, HashSet};
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
    start_x: usize,
    start_y: usize,
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
    let mut start_x = 0;
    let mut start_y = 0;
    let mut start_found = false;
    for (y, line) in puzzle_input.lines().enumerate() {
        assert_eq!(width, line.chars().count());
        for (x, ch) in line.chars().enumerate() {
            let tile = match ch {
                '|' => Tile::VerticalPipe,
                '-' => Tile::HorizontalPipe,
                'L' => Tile::NorthToEastPipe,
                'J' => Tile::NorthToWestPipe,
                '7' => Tile::SouthToWestPipe,
                'F' => Tile::SouthToEastPipe,
                '.' => Tile::Ground,
                'S' => {
                    assert!(!start_found);
                    start_x = x;
                    start_y = y;
                    start_found = true;
                    Tile::StartingPosition
                },
                _ => panic!(),
            };
            tiles.push(tile);
        }
    }
    Map { width, height, tiles, start_x, start_y }
}

fn allowed_directions(tile: Tile) -> (bool, bool, bool, bool) {
    let mut allows_north = false;
    let mut allows_south = false;
    let mut allows_east = false;
    let mut allows_west = false;
    match tile {
        Tile::VerticalPipe =>     { allows_north = true; allows_south = true; },
        Tile::HorizontalPipe =>   { allows_east = true; allows_west = true; },
        Tile::NorthToEastPipe =>  { allows_north = true; allows_east = true; },
        Tile::NorthToWestPipe =>  { allows_north = true; allows_west = true; },
        Tile::SouthToWestPipe =>  { allows_south = true; allows_west = true; },
        Tile::SouthToEastPipe =>  { allows_south = true; allows_east = true; },
        Tile::Ground => (),
        Tile::StartingPosition => {
            allows_north = true;
            allows_south = true;
            allows_east = true;
            allows_west = true;
        },
    };
    (allows_north, allows_south, allows_east, allows_west)
}

// Returns the positions of all elements on the loop
fn find_biggest_loop(map: &Map) -> HashSet<(usize, usize)> {
    let mut deque = VecDeque::new();
    deque.push_back((map.start_x, map.start_y, Dir::North)); // Arbitrary dir

    let mut best_x = 0;
    let mut best_y = 0;
    let mut seen = HashSet::new();
    while let Some((x, y, dir)) = deque.pop_front() {
        let (allows_north, allows_south, allows_east, allows_west) =
            allowed_directions(map.at(x, y));
        match dir {
            Dir::North => if !allows_north { continue; },
            Dir::South => if !allows_south { continue; },
            Dir::East  => if !allows_east  { continue; },
            Dir::West  => if !allows_west  { continue; },
        };
        if !seen.insert((x, y)) {
            // Matched from the other side
            best_x = x;
            best_y = y;
            continue;
        }
        // NOTE - the directions look backwards since we need to be able to move *back* in that
        // direction to get where we were
        if allows_west  && x > 0              { deque.push_back((x - 1, y, Dir::East)); }
        if allows_east  && x < map.width - 1  { deque.push_back((x + 1, y, Dir::West)); }
        if allows_north && y > 0              { deque.push_back((x, y - 1, Dir::South)); }
        if allows_south && y < map.height - 1 { deque.push_back((x, y + 1, Dir::North)); }
    }

    let mut contained = HashSet::new();
    contained.insert((map.start_x, map.start_y));
    let mut deque = VecDeque::new();
    deque.push_back((best_x, best_y));
    while let Some((x, y)) = deque.pop_front() {
        if !contained.insert((x, y)) { continue; }
        let (allows_north, allows_south, allows_east, allows_west) =
            allowed_directions(map.at(x, y));
        if allows_west  { deque.push_back((x - 1, y)); }
        if allows_east  { deque.push_back((x + 1, y)); }
        if allows_north { deque.push_back((x, y - 1)); }
        if allows_south { deque.push_back((x, y + 1)); }
    }

    contained
}

fn part1(map: &Map) -> usize {
    find_biggest_loop(map).len() / 2
}

fn part2(map: &Map) -> usize {
    // Scale up everything by 3x so that adjacent pipes can let us flood fill through
    let scaled_width = map.width * 3;
    let scaled_height = map.height * 3;
    let mut on_loop = HashSet::new();
    for (x, y) in find_biggest_loop(map) {
        let (allows_north, allows_south, allows_east, allows_west) =
            allowed_directions(map.at(x, y));
        on_loop.insert((x * 3 + 1, y * 3 + 1));
        if allows_west  { on_loop.insert((x * 3    , y * 3 + 1)); }
        if allows_east  { on_loop.insert((x * 3 + 2, y * 3 + 1)); }
        if allows_north { on_loop.insert((x * 3 + 1, y * 3    )); }
        if allows_south { on_loop.insert((x * 3 + 1, y * 3 + 2)); }
    }

    let mut outside_loop = HashSet::new();
    let mut pending = VecDeque::new();
    for x in 0..scaled_width {
        for y in [0, scaled_height - 1] { pending.push_back((x, y)); }
    }
    for y in 0..scaled_height {
        for x in [0, scaled_width - 1] { pending.push_back((x, y)); }
    }
    while let Some((x, y)) = pending.pop_front() {
        if on_loop.contains(&(x, y)) { continue; }
        if !outside_loop.insert((x, y)) { continue; }
        if x > 0                 { pending.push_back((x - 1, y)); }
        if x < scaled_width - 1  { pending.push_back((x + 1, y)); }
        if y > 0                 { pending.push_back((x, y - 1)); }
        if y < scaled_height - 1 { pending.push_back((x, y + 1)); }

    }

    let mut inside_cnt = 0;
    for x in 0..map.width {
        for y in 0..map.height {
            // Scaled x, y
            let x = x * 3 + 1;
            let y = y * 3 + 1;
            if !on_loop.contains(&(x, y)) && !outside_loop.contains(&(x, y)) {
                inside_cnt += 1;
            }
        }
    }
    inside_cnt
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let map = parse(&puzzle_input);
    println!("{}", part1(&map));
    println!("{}", part2(&map));
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

    const EX3: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    const EX4: &str = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";

    const EX5: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    const EX6: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX1)), 4);
        assert_eq!(part1(&parse(EX2)), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EX3)), 4);
        assert_eq!(part2(&parse(EX4)), 4);
        assert_eq!(part2(&parse(EX5)), 8);
        assert_eq!(part2(&parse(EX6)), 10);
    }
}
