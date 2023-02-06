use std::collections::HashMap;
use std::io::{self, Read};
use std::ops::{Add, AddAssign};

#[derive(Copy, Clone)]
enum Turn {
    Left, Right
}

enum Step {
    Walk(usize),
    Turn(Turn),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Dir {
    Up, Down, Left, Right
}

impl Dir {
    fn turn(self, turn: Turn) -> Dir {
        match turn {
            Turn::Left => match self {
                Dir::Up => Dir::Left,
                Dir::Right => Dir::Up,
                Dir::Down => Dir::Right,
                Dir::Left => Dir::Down,
            },
            Turn::Right => match self {
                Dir::Up => Dir::Right,
                Dir::Right => Dir::Down,
                Dir::Down => Dir::Left,
                Dir::Left => Dir::Up,
            },
        }
    }

    fn score(self) -> isize {
        match self {
            Dir::Right => 0,
            Dir::Down  => 1,
            Dir::Left  => 2,
            Dir::Up    => 3,
        }
    }

    fn as_offset(self) -> Pos {
        match self {
            Dir::Up    => Pos { x:  0,  y: -1 },
            Dir::Down  => Pos { x:  0,  y:  1 },
            Dir::Left  => Pos { x: -1,  y:  0 },
            Dir::Right => Pos { x:  1,  y:  0 },
        }
    }

    fn flip(self) -> Dir {
        match self {
            Dir::Up    => Dir::Down,
            Dir::Down  => Dir::Up,
            Dir::Left  => Dir::Right,
            Dir::Right => Dir::Left,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Pos {
    x: isize,
    y: isize,
}

impl Add<Pos> for Pos {
    type Output = Pos;
    fn add(self, other: Pos) -> Pos {
        let mut pos = self;
        pos += other;
        pos
    }
}

impl AddAssign<Pos> for Pos {
    fn add_assign(&mut self, other: Pos) {
        self.x += other.x;
        self.y += other.y;
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Open, Wall
}



struct Grid {
    initial_pos: Pos,
    tiles: HashMap<Pos, Tile>,
    block_size: usize,
    jumps: HashMap<(Pos, Dir), (Pos, Dir)>,
}

impl Grid {
    fn walk_one(&self, pos: Pos, dir: Dir) -> (Pos, Dir) {
        if let Some((next_pos, next_dir)) = self.jumps.get(&(pos, dir)) {
            match self.tiles[next_pos] {
                Tile::Open => (*next_pos, *next_dir),
                Tile::Wall => (pos, dir),
            }
        } else {
            match self.tiles[&(pos + dir.as_offset())] {
                Tile::Open => (pos + dir.as_offset(), dir),
                Tile::Wall => (pos, dir),
            }
        }
    }
}

fn parse_raw_grid(input: &str, block_size: usize) -> Grid {
    let mut initial_pos: Option<Pos> = None;
    let mut tiles = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        let y = y as isize;
        for (x, ch) in line.chars().enumerate() {
            let pos = Pos { x: x as isize, y };
            let tile = match ch {
                ' ' => continue,
                '#' => Tile::Wall,
                '.' => Tile::Open,
                _ => panic!(),
            };
            if tile == Tile::Open && initial_pos == None {
                initial_pos = Some(pos);
            }
            tiles.insert(pos, tile);
        }
    }
    Grid {
        initial_pos: initial_pos.unwrap(),
        tiles,
        block_size,
        jumps: HashMap::new(),
    }
}

fn parse_steps(input: &str) -> Vec<Step> {
    let mut ret = Vec::new();
    let mut last_walk: Option<usize> = None;
    for ch in input.chars() {
        match ch {
            '0'..='9' => {
                let num = ch as usize - '0' as usize;
                if let Some(prev) = last_walk {
                    last_walk = Some(prev * 10 + num);
                } else {
                    last_walk = Some(num);
                }
            },
            _ => {
                if let Some(num) = last_walk {
                    ret.push(Step::Walk(num))
                }
                last_walk = None;
                match ch {
                    'L' => ret.push(Step::Turn(Turn::Left)),
                    'R' => ret.push(Step::Turn(Turn::Right)),
                    _ => panic!(),
                }
            },
        }
    }
    if let Some(num) = last_walk {
        ret.push(Step::Walk(num));
    }
    ret
}

fn parse_raw(puzzle_input: &str, block_size: usize) -> (Grid, Vec<Step>) {
    let puzzle_input = puzzle_input.trim_end();
    let mut sp_it = puzzle_input.split("\n\n");
    let grid = parse_raw_grid(sp_it.next().unwrap(), block_size);
    let steps = parse_steps(sp_it.next().unwrap());
    assert!(sp_it.next().is_none());
    (grid, steps)
}

fn fill_jumps_part1(grid: &mut Grid) {
    let height = grid.tiles.keys().map(|pos| pos.y).max().unwrap() + 1;
    let width = grid.tiles.keys().map(|pos| pos.x).max().unwrap() + 1;
    for pos in grid.tiles.keys().copied() {
        for dir in [Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
            let mut pos2 = pos + dir.as_offset();
            if grid.tiles.contains_key(&pos2) {
                continue; // Just a normal step
            }
            loop {
                debug_assert!(pos2 != pos);
                if grid.tiles.contains_key(&pos2) {
                    grid.jumps.insert((pos, dir), (pos2, dir));
                    break;
                }
                pos2 += dir.as_offset();
                if pos2.x >= width { pos2.x = 0 };
                if pos2.x < 0 { pos2.x = width - 1 };
                if pos2.y >= height { pos2.y = 0 };
                if pos2.y < 0 { pos2.y = height - 1 };
            }
        }
    }
}

fn walk_all(grid: &Grid, steps: &[Step]) -> isize {
    let mut pos = grid.initial_pos;
    let mut dir = Dir::Right;
    for step in steps {
        match step {
            Step::Walk(num) => for _ in 0..*num {
                (pos, dir) = grid.walk_one(pos, dir);
            },
            Step::Turn(turn) => dir = dir.turn(*turn),
        }
    }
    1000 * (pos.y + 1) + 4 * (pos.x + 1) + dir.score()
}

fn part1(puzzle_input: &str) -> isize {
    let (mut grid, steps) = parse_raw(puzzle_input, 50 /* only needed for part2 */);
    fill_jumps_part1(&mut grid);
    walk_all(&grid, &steps)
}

fn fill_jumps_part2(grid: &mut Grid, start_pos1: Pos, dir1: Dir, start_pos2: Pos, dir2: Dir) {
    let mut pos1s = Vec::with_capacity(grid.block_size);
    let mut pos2s = Vec::with_capacity(grid.block_size);
    let mut pos1 = start_pos1;
    let mut pos2 = start_pos2;
    for _ in 0..grid.block_size {
        pos1s.push(pos1);
        pos1 += dir1.as_offset();
        pos2s.push(pos2);
        pos2 += dir2.as_offset();
    }
    // Set the travel dirs to the direction toward empty space
    let mut dir1 = dir1.turn(Turn::Right);
    if grid.tiles.contains_key(&(start_pos1 + dir1.as_offset())) {
        dir1 = dir1.flip();
        assert!(!grid.tiles.contains_key(&(start_pos1 + dir1.as_offset())));
    }
    let mut dir2 = dir2.turn(Turn::Right);
    if grid.tiles.contains_key(&(start_pos2 + dir2.as_offset())) {
        dir2 = dir2.flip();
        assert!(!grid.tiles.contains_key(&(start_pos2 + dir2.as_offset())));
    }

    for i in 0..grid.block_size {
        grid.jumps.insert((pos1s[i], dir1), (pos2s[i], dir2.flip()));
        grid.jumps.insert((pos2s[i], dir2), (pos1s[i], dir1.flip()));
    }
}

fn part2(puzzle_input: &str) -> isize {
    let (mut grid, steps) = parse_raw(puzzle_input, 50);
    // FIXME - OK this is super hacky and only works for MY input. But I'm too lazy to map the
    // input onto a cube programmatically
    fill_jumps_part2(&mut grid, Pos { x: 50, y: 0 }, Dir::Right, Pos { x: 0, y: 150 }, Dir::Down);
    fill_jumps_part2(&mut grid, Pos { x: 100, y: 0 }, Dir::Right, Pos { x: 0, y: 199 }, Dir::Right);
    fill_jumps_part2(&mut grid, Pos { x: 50, y: 0 }, Dir::Down, Pos { x: 0, y: 149 }, Dir::Up);
    fill_jumps_part2(&mut grid, Pos { x: 149, y: 0 }, Dir::Down, Pos { x: 99, y: 149 }, Dir::Up);
    fill_jumps_part2(&mut grid, Pos { x: 100, y: 49 }, Dir::Right, Pos { x: 99, y: 50 }, Dir::Down);
    fill_jumps_part2(&mut grid, Pos { x: 50, y: 50 }, Dir::Down, Pos { x: 0, y: 100 }, Dir::Right);
    fill_jumps_part2(&mut grid, Pos { x: 50, y: 149 }, Dir::Right, Pos { x: 49, y: 150 }, Dir::Down);
    walk_all(&grid, &steps)
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    println!("{}", part1(&puzzle_input));
    println!("{}", part2(&puzzle_input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), 6032);
    }

    #[test]
    fn test_part2() {
        let (mut grid, steps) = parse_raw(EX, 4);
        fill_jumps_part2(&mut grid, Pos { x: 8, y: 0 }, Dir::Right, Pos { x: 3, y: 4 }, Dir::Left);
        fill_jumps_part2(&mut grid, Pos { x: 8, y: 0 }, Dir::Down, Pos { x: 4, y: 4 }, Dir::Right);
        fill_jumps_part2(&mut grid, Pos { x: 11, y: 0 }, Dir::Down, Pos { x: 15, y: 11 }, Dir::Up);
        fill_jumps_part2(&mut grid, Pos { x: 0, y: 4 }, Dir::Down, Pos { x: 15, y: 11 }, Dir::Left);
        fill_jumps_part2(&mut grid, Pos { x: 11, y: 4 }, Dir::Down, Pos { x: 15, y: 8 }, Dir::Left);
        fill_jumps_part2(&mut grid, Pos { x: 0, y: 7 }, Dir::Right, Pos { x: 11, y: 11 }, Dir::Left);
        fill_jumps_part2(&mut grid, Pos { x: 4, y: 7 }, Dir::Right, Pos { x: 8, y: 11 }, Dir::Up);
        assert_eq!(walk_all(&grid, &steps), 5031);
    }
}
