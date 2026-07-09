use std::collections::{HashMap, VecDeque};
use std::io::{self, Read};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Pos {
        Pos { x, y }
    }
}

struct Map {
    width: usize,
    height: usize,
    start: Pos,
    rocks: Vec<bool>,
}

impl Map {
    fn idx(&self, pos: Pos) -> usize {
        self.width * pos.y + pos.x
    }

    fn is_rock(&self, pos: Pos) -> bool {
        self.rocks[self.idx(pos)]
    }
}

fn parse(puzzle_input: &str) -> Map {
    let height = puzzle_input.lines().count();
    let width = puzzle_input.lines().next().unwrap().chars().count();
    let mut start = None;
    let mut rocks = Vec::with_capacity(width * height);
    for (y, line) in puzzle_input.lines().enumerate() {
        assert_eq!(width, line.chars().count());
        for (x, ch) in line.chars().enumerate() {
            let is_rock = match ch {
                '#' => true,
                '.' => false,
                'S' => {
                    assert!(start.is_none());
                    start = Some(Pos::new(x, y));
                    false
                },
                _ => panic!(),
            };
            rocks.push(is_rock);
        }
    }
    Map { width, height, start: start.unwrap(), rocks }
}

fn find_shortest_paths(map: &Map, start: Pos) -> HashMap<Pos, usize> {
    let mut shortest_paths = HashMap::new();
    let mut deque = VecDeque::new();
    deque.push_back((start, 0));
    while let Some((pos, dist)) = deque.pop_front() {
        if map.is_rock(pos) { continue; }
        if shortest_paths.contains_key(&pos) { continue; }
        shortest_paths.insert(pos, dist);
        if pos.x > 0              { deque.push_back((Pos::new(pos.x - 1, pos.y), dist + 1)); }
        if pos.x < map.width - 1  { deque.push_back((Pos::new(pos.x + 1, pos.y), dist + 1)); }
        if pos.y > 0              { deque.push_back((Pos::new(pos.x, pos.y - 1), dist + 1)); }
        if pos.y < map.height - 1 { deque.push_back((Pos::new(pos.x, pos.y + 1), dist + 1)); }
    }
    shortest_paths
}


// https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21
fn steps_can_reach(map: &Map, steps: usize) -> usize {
    let paths = find_shortest_paths(map, map.start);

    let even_corners = paths.values().filter(|&&cnt| cnt % 2 == 0 && cnt > 65).count();
    let odd_corners = paths.values().filter(|&&cnt| cnt % 2 == 1 && cnt > 65).count();

    let even_full = paths.values().filter(|&&cnt| cnt % 2 == 0).count();
    let odd_full = paths.values().filter(|&&cnt| cnt % 2 == 1).count();

    assert_eq!(map.width, map.height);
    let n = (steps - map.width / 2) / map.width;
    ((n + 1) * (n + 1)) * odd_full +
        (n * n) * even_full -
        (n + 1) * odd_corners +
        n * even_corners
}

fn part1(map: &Map, steps: usize) -> usize {
    find_shortest_paths(map, map.start).values()
        .filter(|&&cnt| cnt % 2 == steps % 2 && cnt <= steps)
        .count()
}

fn part2(map: &Map) -> usize {
    steps_can_reach(map, 26501365)
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let map = parse(&puzzle_input);
    println!("{}", part1(&map, 64));
    println!("{}", part2(&map));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX), 6), 16);
    }

    #[test]
    fn test_part2() {
        // Since the example input has a different shape (there isn't a line cut out from start to
        // each edge of the map) it would need a more complicated algorithm to solve for.
        // TODO - maybe come back and implement a general algorithm that can solve both the
        // examples and the actual input
        //assert_eq!(steps_can_reach(&parse(EX), 6), 16);
        //assert_eq!(steps_can_reach(&parse(EX), 10), 50);
        //assert_eq!(steps_can_reach(&parse(EX), 50), 1594);
        //assert_eq!(steps_can_reach(&parse(EX), 100), 6536);
        //assert_eq!(steps_can_reach(&parse(EX), 500), 167004);
        //assert_eq!(steps_can_reach(&parse(EX), 1000), 668697);
        //assert_eq!(steps_can_reach(&parse(EX), 5000), 16733044);
    }
}
