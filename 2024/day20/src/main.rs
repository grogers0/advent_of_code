use std::io::{self, Read};
use std::collections::{HashSet, HashMap, VecDeque};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Pos {
    x: usize,
    y: usize,
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
        self.width * pos.y + pos.x
    }
}

fn parse(puzzle_input: &str) -> Map {
    let height = puzzle_input.lines().count();
    let width = puzzle_input.lines().next().unwrap().chars().count();
    let mut walls = vec![];
    let mut start: Option<Pos> = None;
    let mut end: Option<Pos> = None;
    for (y, line) in puzzle_input.lines().enumerate() {
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
    Map { width, height, walls, start: start.unwrap(), end: end.unwrap() }
}

fn calc_durations(map: &Map) -> Vec<usize> {
    let mut durations = vec![0; map.width * map.height];
    let mut queue = VecDeque::new();
    queue.push_back((1, map.start));
    while let Some((dur, pos)) = queue.pop_front() {
        let idx = map.idx(pos);
        if map.walls[idx] { continue; }
        if durations[idx] != 0 { continue; }
        durations[idx] = dur;
        if map.end == pos { continue; }
        let dur = dur + 1;
        if pos.x > 0              { queue.push_back((dur, Pos { x: pos.x - 1, y: pos.y     })); }
        if pos.x < map.width - 1  { queue.push_back((dur, Pos { x: pos.x + 1, y: pos.y     })); }
        if pos.y > 0              { queue.push_back((dur, Pos { x: pos.x    , y: pos.y - 1 })); }
        if pos.y < map.height - 1 { queue.push_back((dur, Pos { x: pos.x    , y: pos.y + 1 })); }
    }
    durations
}

fn find_cheats_from_pos(map: &Map, durations: &[usize], max_cheat: usize, min_saved: usize,
    init_pos: Pos, all_cheats: &mut HashMap<(Pos, Pos), usize>) {
    let init_dur = durations[map.idx(init_pos)];
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    queue.push_back((init_pos, 0));

    while let Some((pos, cheat_used)) = queue.pop_front() {
        if cheat_used > max_cheat { continue; }
        if !map.walls[map.idx(pos)] && (pos != init_pos || cheat_used > 0) {
            let dur = durations[map.idx(pos)];
            if pos != init_pos && dur >= init_dur + cheat_used + min_saved {
                let curr_saved = dur - init_dur - cheat_used;
                all_cheats.entry((init_pos, pos))
                    .and_modify(|best_saved| *best_saved = std::cmp::max(*best_saved, curr_saved))
                    .or_insert(curr_saved);
            }
        }
        if !seen.insert(pos) { continue; }

        if pos.x > 0 {
            queue.push_back((Pos { x: pos.x - 1, y: pos.y }, cheat_used + 1));
        }
        if pos.x < map.width - 1 {
            queue.push_back((Pos { x: pos.x + 1, y: pos.y }, cheat_used + 1));
        }
        if pos.y > 0 {
            queue.push_back((Pos { x: pos.x, y: pos.y - 1 }, cheat_used + 1));
        }
        if pos.y < map.height - 1 {
            queue.push_back((Pos { x: pos.x, y: pos.y + 1 }, cheat_used + 1));
        }
    }
}

fn find_cheats(map: &Map, durations: &[usize], max_cheat: usize, min_saved: usize) -> usize {
    let mut all_cheats = HashMap::new();
    for y in 0..map.height {
        for x in 0..map.width {
            let pos = Pos { x, y };
            if map.walls[map.idx(pos)] { continue; }
            find_cheats_from_pos(map, durations, max_cheat, min_saved, pos, &mut all_cheats);
        }
    }
    all_cheats.len()
}

fn part1(map: &Map) -> usize {
    let durations = calc_durations(map);
    find_cheats(&map, &durations, 2, 100)
}

fn part2(map: &Map) -> usize {
    let durations = calc_durations(map);
    find_cheats(&map, &durations, 20, 100)
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

    const EX: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn test_part1() {
        let map = parse(EX);
        assert_eq!(find_cheats(&map, &calc_durations(&map), 2, 10), 10);
    }

    #[test]
    fn test_part2() {
        let map = parse(EX);
        assert_eq!(find_cheats(&map, &calc_durations(&map), 20, 70), 41);
    }
}
