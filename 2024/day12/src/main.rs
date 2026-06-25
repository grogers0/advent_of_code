use std::io::{self, Read};
use std::collections::{HashSet, VecDeque};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Edge {
    Upper, Lower, Left, Right
}

struct Map {
    width: usize,
    height: usize,
    grid: Vec<char>,
}

impl Map {
    fn idx(&self, pos: Pos) -> usize {
        self.width * pos.y + pos.x
    }
}

fn parse(puzzle_input: &str) -> Map {
    let height = puzzle_input.lines().count();
    let width = puzzle_input.lines().next().unwrap().chars().count();
    let mut grid = vec![];
    for line in puzzle_input.lines() {
        assert_eq!(line.chars().count(), width);
        for ch in line.chars() {
            grid.push(ch);
        }
    }
    Map { width, height, grid }
}

fn compute(map: &Map) -> Vec<(usize, HashSet<(Pos, Edge)>)> {
    let mut ret = vec![];
    let mut seen = vec![false; map.width * map.height];
    for y in 0..map.height {
        for x in 0..map.width {
            let init_pos = Pos { x, y };
            let init_idx = map.idx(init_pos);
            if seen[init_idx] { continue; }
            let init_ch = map.grid[init_idx];
            let mut queue = VecDeque::new();
            queue.push_back((init_pos, init_pos, Edge::Upper));

            let mut area = 0;
            let mut perimeter = HashSet::new();
            while let Some((pos, prev_pos, edge)) = queue.pop_front() {
                let idx = map.idx(pos);
                if map.grid[idx] != init_ch {
                    perimeter.insert((prev_pos, edge));
                    continue;
                }
                if seen[idx] { continue; }
                seen[idx] = true;
                area += 1;
                if pos.x > 0 {
                    queue.push_back((Pos { x: pos.x - 1, y: pos.y }, pos, Edge::Left));
                } else {
                    perimeter.insert((pos, Edge::Left));
                }
                if pos.x < map.width - 1 {
                    queue.push_back((Pos { x: pos.x + 1, y: pos.y }, pos, Edge::Right));
                } else {
                    perimeter.insert((pos, Edge::Right));
                }

                if pos.y > 0 {
                    queue.push_back((Pos { x: pos.x, y: pos.y - 1 }, pos, Edge::Upper));
                } else {
                    perimeter.insert((pos, Edge::Upper));
                }
                if pos.y < map.height - 1 {
                    queue.push_back((Pos { x: pos.x, y: pos.y + 1 }, pos, Edge::Lower));
                } else {
                    perimeter.insert((pos, Edge::Lower));
                }
            }
            ret.push((area, perimeter));
        }
    }
    ret
}

fn part1(computed_areas: &[(usize, HashSet<(Pos, Edge)>)]) -> usize {
    let mut sum = 0;
    for (area, perimeter) in computed_areas {
        sum += *area * perimeter.len();
    }
    sum
}

fn part2(computed_areas: &[(usize, HashSet<(Pos, Edge)>)]) -> usize {
    let mut sum = 0;
    for (area, perimeter) in computed_areas {
        let mut seen = HashSet::new();
        let mut straight_edges = 0;
        for (pos, edge) in perimeter {
            let edge = *edge;
            if !seen.insert((*pos, edge)) { continue; }
            straight_edges += 1;
            match edge {
                Edge::Upper | Edge::Lower => {
                    for x in (0..pos.x).rev() {
                        let pos2 = Pos { x, y: pos.y };
                        if !perimeter.contains(&(pos2, edge)) { break; }
                        if !seen.insert((pos2, edge)) { break; }
                    }
                    for x in (pos.x + 1).. {
                        let pos2 = Pos { x, y: pos.y };
                        if !perimeter.contains(&(pos2, edge)) { break; }
                        if !seen.insert((pos2, edge)) { break; }
                    }
                },
                Edge::Left | Edge::Right => {
                    for y in (0..pos.y).rev() {
                        let pos2 = Pos { x: pos.x, y };
                        if !perimeter.contains(&(pos2, edge)) { break; }
                        if !seen.insert((pos2, edge)) { break; }
                    }
                    for y in (pos.y + 1).. {
                        let pos2 = Pos { x: pos.x, y };
                        if !perimeter.contains(&(pos2, edge)) { break; }
                        if !seen.insert((pos2, edge)) { break; }
                    }
                },
            }
        }

        sum += *area * straight_edges;
    }
    sum
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let map = parse(&puzzle_input);
    let computed_areas = compute(&map);
    println!("{}", part1(&computed_areas));
    println!("{}", part2(&computed_areas));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1: &str = "AAAA
BBCD
BBCC
EEEC";

    const EX2: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";


    const EX3: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    const EX4: &str = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

    const EX5: &str = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";


    #[test]
    fn test_part1() {
        assert_eq!(part1(&compute(&parse(EX1))), 140);
        assert_eq!(part1(&compute(&parse(EX2))), 772);
        assert_eq!(part1(&compute(&parse(EX3))), 1930);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&compute(&parse(EX1))), 80);
        assert_eq!(part2(&compute(&parse(EX2))), 436);
        assert_eq!(part2(&compute(&parse(EX3))), 1206);
        assert_eq!(part2(&compute(&parse(EX4))), 236);
        assert_eq!(part2(&compute(&parse(EX5))), 368);
    }
}
