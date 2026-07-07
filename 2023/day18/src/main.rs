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
}

struct Map {
    width: usize,
    height: usize,
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

fn parse(puzzle_input: &str) -> (Vec<Instruction>, Vec<Instruction>) {
    let mut instructions1 = Vec::new();
    let mut instructions2 = Vec::new();
    for line in puzzle_input.lines() {
        let mut sp = line.split(" ");
        {
            let dir = match sp.next().unwrap() {
                "R" => Dir::Right,
                "L" => Dir::Left,
                "D" => Dir::Down,
                "U" => Dir::Up,
                _ => panic!(),
            };
            let dist = sp.next().unwrap().parse().unwrap();
            instructions1.push(Instruction { dir, dist });
        }

        {
            let color = sp.next().unwrap();
            assert!(sp.next().is_none());
            assert!(color.starts_with("(#"));
            assert!(color.ends_with(")"));
            let dir = match &color[(color.len() - 2)..(color.len() - 1)] {
                "0" => Dir::Right,
                "1" => Dir::Down,
                "2" => Dir::Left,
                "3" => Dir::Up,
                _ => panic!(),
            };
            let dist = usize::from_str_radix(&color[2..(color.len() - 2)], 16).unwrap();
            instructions2.push(Instruction { dir, dist });
        }
    }
    (instructions1, instructions2)
}

// The instructions may start anywhere on the map, find the bounds of the map and then shift the
// x/y start to the appropriate position to make indexes only in 0..N
// Returns (width, height, xstart, ystart)
fn discover_bounds(instructions: &[Instruction]) -> (usize, usize, usize, usize) {
    let mut x = 0isize;
    let mut y = 0isize;
    let mut xmin = 0;
    let mut xmax = 0;
    let mut ymin = 0;
    let mut ymax = 0;
    for inst in instructions {
        match inst.dir {
            Dir::Up => y -= inst.dist as isize,
            Dir::Down => y += inst.dist as isize,
            Dir::Left => x -= inst.dist as isize,
            Dir::Right => x += inst.dist as isize,
        }
        xmin = min(xmin, x);
        xmax = max(xmax, x);
        ymin = min(ymin, y);
        ymax = max(ymax, y);
    }
    let width = (xmax - xmin + 1) as usize;
    let height = (ymax - ymin + 1) as usize;
    let xstart = (-xmin) as usize;
    let ystart = (-ymin) as usize;
    (width, height, xstart, ystart)
}

// Calculate the change of basis for x/y to compress the coordinates
fn change_basis(instructions: &[Instruction], xstart: usize, ystart: usize) -> (Vec<usize>, Vec<usize>) {
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    let mut x = xstart;
    let mut y = ystart;

    xs.push(x);
    ys.push(y);
    for inst in instructions {
        match inst.dir {
            Dir::Up => y -= inst.dist,
            Dir::Down => y += inst.dist,
            Dir::Left => x -= inst.dist,
            Dir::Right => x += inst.dist,
        }
        xs.push(x);
        ys.push(y);
    }
    xs.sort();
    ys.sort();
    xs.dedup();
    ys.dedup();

    let compress_basis = |vals: &[usize]| -> Vec<usize> {
        let mut counts = Vec::new();
        let mut prev = 0;
        for &curr in vals {
            if curr - prev > 1 {
                counts.push(curr - prev - 1);
            }
            counts.push(1);
            prev = curr;
        }
        counts
    };
    let xcounts = compress_basis(&xs);
    let ycounts = compress_basis(&ys);
    (xcounts, ycounts)
}

fn build_map(instructions: &[Instruction], xstart: usize, ystart: usize, xcounts: &[usize], ycounts: &[usize]) -> Map {
    let find_start = |start: usize, counts: &[usize]| -> usize {
        let mut sum = 0;
        for i in 0..counts.len() {
            assert!(start >= sum);
            if start == sum { return i; }
            sum += counts[i];
        }
        sum
    };
    let mut map = Map {
        width: xcounts.len(),
        height: ycounts.len(),
        tiles: vec![false; xcounts.len() * ycounts.len()],
    };
    let mut x = find_start(xstart, xcounts);
    let mut y = find_start(ystart, ycounts);
    map.fill(x, y);
    for inst in instructions {
        let mut dist = 0;
        while dist < inst.dist {
            match inst.dir {
                Dir::Up => { dist += ycounts[y]; y -= 1; },
                Dir::Down => { dist += ycounts[y]; y += 1; },
                Dir::Left => { dist += xcounts[x]; x -= 1; },
                Dir::Right => { dist += xcounts[x]; x += 1; },
            }
            map.fill(x, y);
        }
        assert_eq!(dist, inst.dist);
    }

    map
}

fn calc(instructions: &[Instruction]) -> u64 {
    let (_width, _height, xstart, ystart) = discover_bounds(instructions);
    let (xcounts, ycounts) = change_basis(instructions, xstart, ystart);
    let map = build_map(instructions, xstart, ystart, &xcounts, &ycounts);

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

    let mut ret = xcounts.iter().cloned().sum::<usize>() as u64 *
        ycounts.iter().cloned().sum::<usize>() as u64;
    for (x, y) in outside {
        ret -= xcounts[x] as u64 * ycounts[y] as u64;
    }
    ret
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let (instructions1, instructions2) = parse(&puzzle_input);
    println!("{}", calc(&instructions1));
    println!("{}", calc(&instructions2));
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
        assert_eq!(calc(&parse(EX).0), 62);
    }

    #[test]
    fn test_part2() {
        assert_eq!(calc(&parse(EX).1), 952408144115);
    }
}
