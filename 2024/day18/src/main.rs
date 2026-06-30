use std::io::{self, Read};
use std::collections::{HashSet, VecDeque};

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Pos {
    x: usize,
    y: usize,
}

fn parse(puzzle_input: &str) -> Vec<Pos> {
    puzzle_input.lines().map(|line| {
        let mut sp = line.split(",");
        let x = sp.next().unwrap().parse::<usize>().unwrap();
        let y = sp.next().unwrap().parse::<usize>().unwrap();
        assert!(sp.next().is_none());
        Pos { x, y }
    }).collect()
}

fn walls_from_positions(width: usize, height: usize, positions: &[Pos]) -> Vec<bool> {
    let mut walls = vec![false; width * height];
    for &pos in positions {
        walls[pos.y * width + pos.x] = true;
    }
    walls
}

fn find_shortest_path(width: usize, height: usize, walls: &[bool]) -> Option<usize> {
    let mut deque = VecDeque::new();
    let finish = Pos { x: width - 1, y: height - 1 };
    deque.push_back((Pos { x: 0, y: 0 }, 0));
    let mut seen = HashSet::new();
    while let Some((pos, steps)) = deque.pop_front() {
        if pos == finish { return Some(steps); }
        if walls[pos.y * width + pos.x] { continue; }
        if !seen.insert(pos) { continue; }

        let steps = steps + 1;
        if pos.x > 0          { deque.push_back((Pos { x: pos.x - 1, y: pos.y     }, steps)); }
        if pos.x < width - 1  { deque.push_back((Pos { x: pos.x + 1, y: pos.y     }, steps)); }
        if pos.y > 0          { deque.push_back((Pos { x: pos.x    , y: pos.y - 1 }, steps)); }
        if pos.y < height - 1 { deque.push_back((Pos { x: pos.x    , y: pos.y + 1 }, steps)); }
    }
    None
}

fn part1(width: usize, height: usize, positions: &[Pos]) -> usize {
    let walls = walls_from_positions(width, height, positions);
    find_shortest_path(width, height, &walls).unwrap()
}

fn part2(width: usize, height: usize, positions: &[Pos]) -> String {
    let mut walls = walls_from_positions(width, height, &[]);
    for pos in positions {
        walls[pos.y * width + pos.x] = true;
        if find_shortest_path(width, height, &walls).is_none() {
            return format!("{},{}", pos.x, pos.y);
        }
    }
    unreachable!();
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let positions = parse(&puzzle_input);
    println!("{}", part1(71, 71, &positions[0..1024]));
    println!("{}", part2(71, 71, &positions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn test_part1() {
        assert_eq!(part1(7, 7, &parse(EX)[0..12]), 22);
    }

    #[test]
    fn test_part2() {
        assert_eq!(&part2(7, 7, &parse(EX)), "6,1");
    }
}
