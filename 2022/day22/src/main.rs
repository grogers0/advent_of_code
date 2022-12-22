use std::io::{self, Read};
use std::ops::Index;

#[derive(Copy, Clone)]
enum Turn {
    Left, Right
}

enum Step {
    Walk(usize),
    Turn(Turn),
}

#[derive(Copy, Clone, Debug)]
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

    fn score(self) -> usize {
        match self {
            Dir::Right => 0,
            Dir::Down  => 1,
            Dir::Left  => 2,
            Dir::Up    => 3,
        }
    }
}

#[derive(Copy, Clone)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Open, Wall, StepThrough
}



struct Grid {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl Grid {
    fn walk_one(&self, pos: Pos, dir: Dir) -> Pos {
        let init_pos = pos;
        match dir {
            Dir::Up => {
                let x = pos.x;
                let ys1 = (0..pos.y).rev();
                let ys2 = (pos.y..self.height).rev();
                for y in ys1.chain(ys2) {
                    match self[Pos{x, y}] {
                        Tile::Open => return Pos{x, y},
                        Tile::Wall => return init_pos,
                        Tile::StepThrough => (),
                    }
                }
                panic!()
            },
            Dir::Down => {
                let x = pos.x;
                let ys1 = (pos.y+1)..self.height;
                let ys2 = 0..pos.y;
                for y in ys1.chain(ys2) {
                    match self[Pos{x, y}] {
                        Tile::Open => return Pos{x, y},
                        Tile::Wall => return init_pos,
                        Tile::StepThrough => (),
                    }
                }
                panic!()
            },
            Dir::Left => {
                let y = pos.y;
                let xs1 = (0..pos.x).rev();
                let xs2 = (pos.x..self.width).rev();
                for x in xs1.chain(xs2) {
                    match self[Pos{x, y}] {
                        Tile::Open => return Pos{x, y},
                        Tile::Wall => return init_pos,
                        Tile::StepThrough => (),
                    }
                }
                panic!()
            },
            Dir::Right => {
                let y = pos.y;
                let xs1 = (pos.x+1)..self.width;
                let xs2 = 0..pos.x;
                for x in xs1.chain(xs2) {
                    match self[Pos{x, y}] {
                        Tile::Open => return Pos{x, y},
                        Tile::Wall => return init_pos,
                        Tile::StepThrough => (),
                    }
                }
                panic!()
            },
        }
    }
}

impl Index<Pos> for Grid {
    type Output = Tile;
    fn index(&self, pos: Pos) -> &Tile {
        debug_assert!(pos.x < self.width);
        debug_assert!(pos.y < self.height);
        &self.tiles[self.width * pos.y + pos.x]
    }
}

fn parse_grid(input: &str) -> Grid {
    let height = input.lines().count();
    let width = input.lines().map(|line| line.len()).max().unwrap();
    let mut tiles = vec![Tile::StepThrough; width * height];
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let tile = match ch {
                ' ' => Tile::StepThrough,
                '#' => Tile::Wall,
                '.' => Tile::Open,
                _ => panic!(),
            };
            tiles[y * width + x] = tile;
        }
    }
    Grid { width, height, tiles }
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

fn parse(puzzle_input: &str) -> (Grid, Vec<Step>) {
    let puzzle_input = puzzle_input.trim_end();
    let mut sp_it = puzzle_input.split("\n\n");
    let grid = parse_grid(sp_it.next().unwrap());
    let steps = parse_steps(sp_it.next().unwrap());
    assert!(sp_it.next().is_none());
    (grid, steps)
}

fn initial_tile(grid: &Grid) -> Pos {
    for x in 0..grid.width {
        let pos = Pos{ x, y: 0 };
        if grid[pos] == Tile::Open {
            return pos;
        }
    }
    panic!()
}

fn part1(puzzle_input: &str) -> usize {
    let (grid, steps) = parse(puzzle_input);
    let mut pos = initial_tile(&grid);
    let mut dir = Dir::Right;
    for step in &steps {
        match step {
            Step::Walk(num) => for _ in 0..*num {
                pos = grid.walk_one(pos, dir);
            },
            Step::Turn(turn) => dir = dir.turn(*turn),
        }
    }
    1000 * (pos.y + 1) + 4 * (pos.x + 1) + dir.score()
}

fn part2(puzzle_input: &str) -> &str {
    "FIXME"
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
        // FIXME
    }
}
