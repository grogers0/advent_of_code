use std::collections::HashSet;
use std::cmp::{min, max};
use std::io::{self, Read};
use std::fmt;

#[derive(Copy, Clone)]
enum Dir {
    Left, Right, Up, Down
}

struct Instruction {
    dir: Dir,
    dist: usize,
    // color: u32,
}

struct Map {
    width: usize,
    height: usize,
    start_x: usize,
    start_y: usize,
    tiles: Vec<bool>,
}

impl Map {
    fn idx(&self, x: usize, y: usize) -> usize {
        assert!(x < self.width && y < self.height);
        y * self.width + x
    }

    fn fill(&mut self, x: usize, y: usize) {
        let i = self.idx(x, y);
        self.tiles[i] = true;
    }

    fn is_filled(&self, x: usize, y: usize) -> bool {
        self.tiles[self.idx(x, y)]
    }
}

fn parse(puzzle_input: &str) -> Vec<Instruction> {
    puzzle_input.lines().map(|line| {
        let mut sp_iter = line.split(" ");
        let dir = match sp_iter.next().unwrap() {
            "R" => Dir::Right,
            "L" => Dir::Left,
            "D" => Dir::Down,
            "U" => Dir::Up,
            _ => panic!(),
        };
        let dist = sp_iter.next().unwrap().parse().unwrap();
        Instruction { dir, dist }
    }).collect()
}

// NOTE - the map is not filled
fn discover_bounds(instructions: &[Instruction]) -> Map {
    let mut x = 0isize;
    let mut y = 0isize;
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;
    for inst in instructions {
        for _ in 0..inst.dist {
            match inst.dir {
                Dir::Up => y -= 1,
                Dir::Down => y += 1,
                Dir::Left => x -= 1,
                Dir::Right => x += 1,
            }
        }
        min_x = min(min_x, x);
        max_x = max(max_x, x);
        min_y = min(min_y, y);
        max_y = max(max_y, y);
    }
    let width = (max_x - min_x + 1) as usize;
    let height = (max_y - min_y + 1) as usize;
    let tiles = vec![false; width * height];
    Map {
        width,
        height,
        start_x: (-min_x) as usize,
        start_y: (-min_y) as usize,
        tiles,
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            if y != 0 { write!(f, "\n")?; }
            for x in 0..self.width {
                if self.is_filled(x, y) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
        }
        Ok(())
    }
}

fn part1(instructions: &[Instruction]) -> usize {
    let mut map = discover_bounds(instructions);
    let mut x = map.start_x;
    let mut y = map.start_y;
    map.fill(x, y);
    for inst in instructions {
        for _ in 0..inst.dist {
            match inst.dir {
                Dir::Up => y -= 1,
                Dir::Down => y += 1,
                Dir::Left => x -= 1,
                Dir::Right => x += 1,
            }
            map.fill(x, y);
        }
    }

    let mut outside = HashSet::new();
    let mut pending = Vec::new();
    for y in 0..map.height {
        for x in [0, map.width - 1] {
            pending.push((x, y));
        }
    }
    for x in 0..map.width {
        for y in [0, map.height - 1] {
            pending.push((x, y));
        }
    }
    while let Some((x, y)) = pending.pop() {
        if map.is_filled(x, y) { continue; }
        if !outside.insert((x, y)) { continue; }
        if x > 0            { pending.push((x-1, y)); }
        if x < map.width-1  { pending.push((x+1, y)); }
        if y > 0            { pending.push((x, y-1)); }
        if y < map.height-1 { pending.push((x, y+1)); }
    }

    (map.width * map.height) - outside.len()
}

fn part2(puzzle_input: &str) -> &str {
    "FIXME"
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let instructions = parse(&puzzle_input);
    println!("{}", part1(&instructions));
    println!("{}", part2(&puzzle_input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
    
    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX)), 62);
    }

    #[test]
    fn test_part2() {
        // FIXME
    }
}
