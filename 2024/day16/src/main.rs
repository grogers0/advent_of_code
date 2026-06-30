use std::io::{self, Read};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet, HashMap, VecDeque};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Dir {
    North, South, East, West,
}

impl Dir {
    fn rotate_left(self) -> Dir {
        match self {
            Dir::North => Dir::West,
            Dir::South => Dir::East,
            Dir::East  => Dir::North,
            Dir::West  => Dir::South,
        }
    }

    fn rotate_right(self) -> Dir {
        match self {
            Dir::North => Dir::East,
            Dir::South => Dir::West,
            Dir::East  => Dir::South,
            Dir::West  => Dir::North,
        }
    }
}

impl Pos {
    fn step(self, dir: Dir) -> Pos {
        match dir {
            Dir::North => Pos { x: self.x    , y: self.y - 1 },
            Dir::South => Pos { x: self.x    , y: self.y + 1 },
            Dir::East  => Pos { x: self.x + 1, y: self.y     },
            Dir::West  => Pos { x: self.x - 1, y: self.y     },
        }
    }
}

struct Map {
    width: usize,
    height: usize,
    walls: Vec<bool>,
    start: Pos,
    end: Pos,
}

impl Map {
    fn idx(&self, pos: Pos) -> usize {
        debug_assert!(pos.x < self.width && pos.y < self.height);
        pos.y * self.width + pos.x
    }

    fn at_wall(&self, pos: Pos) -> bool {
        self.walls[self.idx(pos)]
    }
}

fn parse(puzzle_input: &str) -> Map {
    let height = puzzle_input.lines().count();
    let width = puzzle_input.lines().next().unwrap().chars().count();
    let mut walls = vec![];
    let mut start: Option<Pos> = None;
    let mut end: Option<Pos> = None;
    for (y, line) in puzzle_input.lines().enumerate() {
        assert_eq!(width, line.chars().count());
        for (x, ch) in line.chars().enumerate() {
            let is_wall = match ch {
                '#' => true,
                '.' => false,
                'S' => {
                    assert!(start.is_none());
                    start = Some(Pos { x, y });
                    false
                },
                'E' => {
                    assert!(end.is_none());
                    end = Some(Pos { x, y });
                    false
                },
                _ => panic!(),
            };
            walls.push(is_wall);
        }
    }
    Map {
        width,
        height,
        walls,
        start: start.unwrap(),
        end: end.unwrap(),
    }
}

fn part1(map: &Map) -> usize {
    let mut seen: HashSet<(Pos, Dir)> = HashSet::new();
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), map.start, Dir::East));
    while let Some((Reverse(cost), pos, dir)) = heap.pop() {
        if pos == map.end { return cost; }
        if map.at_wall(pos) { continue; }
        if !seen.insert((pos, dir)) { continue; }
        heap.push((Reverse(cost + 1000), pos, dir.rotate_left()));
        heap.push((Reverse(cost + 1000), pos, dir.rotate_right()));
        heap.push((Reverse(cost + 1), pos.step(dir), dir));
    }
    panic!()
}

fn part2(map: &Map) -> usize {
    struct BestState {
        best_cost: usize,
        best_tiles: HashSet<Pos>,
    }
    let mut best_state_map = HashMap::new();
    let mut deque = VecDeque::new();
    best_state_map.insert(
        (map.start, Dir::East),
        BestState { best_cost: usize::max_value(), best_tiles: HashSet::new() });
    deque.push_back((0, map.start, Dir::East, map.start, Dir::East));
    while let Some((cost, pos, dir, from_pos, from_dir)) = deque.pop_front() {
        if map.at_wall(pos) { continue; }

        let path_tiles = {
            let mut tiles = best_state_map[&(from_pos, from_dir)].best_tiles.clone();
            tiles.insert(pos);
            tiles
        };
        let mut best_path_modified = false;
        best_state_map.entry((pos, dir))
            .and_modify(|best_state| {
                if cost < best_state.best_cost {
                    best_state.best_cost = cost;
                    best_state.best_tiles = path_tiles.clone();
                    best_path_modified = true;
                } else if cost == best_state.best_cost {
                    for &tile in &path_tiles {
                        if best_state.best_tiles.insert(tile) {
                            best_path_modified = true;
                        }
                    }
                }
            }).or_insert_with(|| {
                best_path_modified = true;
                BestState { best_cost: cost, best_tiles: path_tiles }
            });

        if !best_path_modified { continue; }
        deque.push_back((cost + 1000, pos, dir.rotate_left(), pos, dir));
        deque.push_back((cost + 1000, pos, dir.rotate_right(), pos, dir));
        deque.push_back((cost + 1, pos.step(dir), dir, pos, dir));
    }
    let mut best_cost = usize::max_value();
    let mut best_tiles_len = 0;
    for dir in &[Dir::East, Dir::West, Dir::North, Dir::South] {
        let best_state = &best_state_map[&(map.end, *dir)];
        if best_state.best_cost < best_cost {
            best_cost = best_state.best_cost;
            best_tiles_len = best_state.best_tiles.len();
        }
    }
    best_tiles_len
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

    const EX1: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const EX2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";


    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX1)), 7036);
        assert_eq!(part1(&parse(EX2)), 11048);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EX1)), 45);
        assert_eq!(part2(&parse(EX2)), 64);
    }
}
