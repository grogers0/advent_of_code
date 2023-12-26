use std::io::{self, Read};

#[derive(Copy, Clone, Eq, PartialEq)]
enum Dir {
    Up, Down, Left, Right,
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Path,
    Forest,
    Slope(Dir),
}

struct Map {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Pos {
        Pos { x, y }
    }
}

impl Map {
    fn idx(&self, pos: Pos) -> usize {
        pos.y * self.width + pos.x
    }

    fn at(&self, pos: Pos) -> Tile {
        self.tiles[self.idx(pos)]
    }
}

fn parse(puzzle_input: &str) -> Map {
    let height = puzzle_input.lines().count();
    let width = puzzle_input.lines().next().unwrap().chars().count();
    let mut tiles = Vec::with_capacity(width * height);
    for line in puzzle_input.lines() {
        assert_eq!(width, line.len());
        for ch in line.chars() {
            tiles.push(match ch {
                '.' => Tile::Path,
                '#' => Tile::Forest,
                '>' => Tile::Slope(Dir::Right),
                '^' => Tile::Slope(Dir::Up),
                'v' => Tile::Slope(Dir::Down),
                '<' => Tile::Slope(Dir::Left),
                _ => panic!(),
            });
        }
    }
    Map { width, height, tiles }
}


fn find_start_or_end(map: &Map, y: usize) -> Pos {
    for x in 0..map.width {
        let pos = Pos::new(x, y);
        if let Tile::Path = map.at(pos) {
            return pos;
        }
    }
    panic!()
}

fn find_start(map: &Map) -> Pos {
    find_start_or_end(map, 0)
}

fn find_end(map: &Map) -> Pos {
    find_start_or_end(map, map.height - 1)
}

fn find_longest_path_len_between(map: &Map, start_pos: Pos, end_pos: Pos, is_slippery: bool) -> usize {
    let mut longest_path_len = None;
    let mut update_longest_path_len = |maybe_path_len| {
        if let Some(path_len) = maybe_path_len {
            if let Some(max_path_len) = longest_path_len {
                longest_path_len = Some(std::cmp::max(path_len, max_path_len));
            } else {
                longest_path_len = maybe_path_len;
            }
        }
    };
    enum Mode {
        Moved(Pos, usize),
        Reset(Pos),
    }

    let mut stack = Vec::new();
    let mut seen = vec![false; map.width * map.height];
    stack.push(Mode::Moved(start_pos, 0));
    while let Some(elem) = stack.pop() {
        match elem {
            Mode::Moved(pos, len) => {
                let idx = map.idx(pos);
                if seen[idx] { continue; }
                if pos == end_pos {
                    update_longest_path_len(Some(len));
                    continue;
                }
                let tile = map.at(pos);
                if tile == Tile::Forest { continue }
                seen[idx] = true;

                stack.push(Mode::Reset(pos));
                if pos.y > 0 && (tile == Tile::Slope(Dir::Up) || tile == Tile::Path || !is_slippery) {
                    stack.push(Mode::Moved(Pos::new(pos.x, pos.y-1), len+1));
                }
                if pos.y < map.height-1 && (tile == Tile::Slope(Dir::Down) || tile == Tile::Path || !is_slippery) {
                    stack.push(Mode::Moved(Pos::new(pos.x, pos.y+1), len+1));
                }
                if pos.x > 0 && (tile == Tile::Slope(Dir::Left) || tile == Tile::Path || !is_slippery) {
                    stack.push(Mode::Moved(Pos::new(pos.x-1, pos.y), len+1));
                }
                if pos.x < map.width-1 && (tile == Tile::Slope(Dir::Right) || tile == Tile::Path || !is_slippery) {
                    stack.push(Mode::Moved(Pos::new(pos.x+1, pos.y), len+1));
                }
            },
            Mode::Reset(pos) => seen[map.idx(pos)] = false,
        };
    }

    longest_path_len.unwrap()
}


fn part1(map: &Map) -> usize {
    find_longest_path_len_between(map, find_start(map), find_end(map), true)
}

fn part2(map: &Map) -> usize {
    find_longest_path_len_between(map, find_start(map), find_end(map), false)
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

    const EX: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX)), 94);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EX)), 154);
    }
}
