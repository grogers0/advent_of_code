use std::io::{self, Read};

#[derive(Copy, Clone, PartialEq, Eq)]
enum Dir {
    Up, Down, Left, Right,
}

#[derive(Copy, Clone)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone)]
enum Cell {
    Wall, Empty, WholeBox, LeftHalfBox, RightHalfBox,
}

#[derive(Clone)]
struct Map {
    width: usize,
    height: usize,
    robot: Pos,
    grid: Vec<Cell>,
}

impl Pos {
    fn step(self, dir: Dir) -> Pos {
        match dir {
            Dir::Up    => Pos { x: self.x,     y: self.y - 1 },
            Dir::Down  => Pos { x: self.x,     y: self.y + 1 },
            Dir::Left  => Pos { x: self.x - 1, y: self.y     },
            Dir::Right => Pos { x: self.x + 1, y: self.y     },
        }
    }

    fn coord(self) -> usize {
        self.y * 100 + self.x
    }
}

impl Map {
    fn idx(&self, pos: Pos) -> usize {
        pos.y * self.width + pos.x
    }

    fn can_step_box(&self, pos: Pos, dir: Dir) -> bool {
        let curr_cell = self.grid[self.idx(pos)];
        match curr_cell {
            Cell::WholeBox => {
                let next = pos.step(dir);
                match self.grid[self.idx(next)] {
                    Cell::Empty => true,
                    Cell::Wall => false,
                    Cell::WholeBox => self.can_step_box(next, dir),
                    Cell::LeftHalfBox | Cell::RightHalfBox => panic!(),
                }
            },
            Cell::LeftHalfBox | Cell::RightHalfBox => {
                match dir {
                    Dir::Left | Dir::Right => {
                        let same_box = match (dir, curr_cell) {
                            (Dir::Left, Cell::RightHalfBox) => true,
                            (Dir::Left, Cell::LeftHalfBox) => false,
                            (Dir::Right, Cell::RightHalfBox) => false,
                            (Dir::Right, Cell::LeftHalfBox) => true,
                            _ => panic!(),
                        };
                        let next = pos.step(dir);
                        if same_box {
                            self.can_step_box(next, dir)
                        } else {
                            match self.grid[self.idx(next)] {
                                Cell::Empty => true,
                                Cell::Wall => false,
                                Cell::LeftHalfBox | Cell::RightHalfBox => self.can_step_box(next, dir),
                                Cell::WholeBox => panic!(),
                            }
                        }
                    },
                    Dir::Up | Dir::Down => {
                        let (left_next, right_next) = match curr_cell {
                            Cell::LeftHalfBox => (pos.step(dir), pos.step(Dir::Right).step(dir)),
                            Cell::RightHalfBox => (pos.step(Dir::Left).step(dir), pos.step(dir)),
                            _ => unreachable!(),
                        };
                        let can_step_left = match self.grid[self.idx(left_next)] {
                            Cell::Empty => true,
                            Cell::Wall => false,
                            Cell::LeftHalfBox | Cell::RightHalfBox => self.can_step_box(left_next, dir),
                            Cell::WholeBox => panic!(),
                        };
                        let can_step_right = match self.grid[self.idx(right_next)] {
                            Cell::Empty => true,
                            Cell::Wall => false,
                            Cell::LeftHalfBox | Cell::RightHalfBox => self.can_step_box(right_next, dir),
                            Cell::WholeBox => panic!(),
                        };
                        can_step_left && can_step_right
                    },
                }
            },
            Cell::Empty | Cell::Wall => panic!(),
        }
    }

    fn step_box(&mut self, pos: Pos, dir: Dir) {
        match self.grid[self.idx(pos)] {
            Cell::WholeBox => {
                let next = pos.step(dir);
                let next_idx = self.idx(next);
                let curr_idx = self.idx(pos);
                self.grid[curr_idx] = Cell::Empty;
                match self.grid[self.idx(next)] {
                    Cell::Empty => self.grid[next_idx] = Cell::WholeBox,
                    Cell::Wall => panic!(),
                    Cell::WholeBox => {
                        self.step_box(next, dir);
                        self.grid[next_idx] = Cell::WholeBox;
                    },
                    Cell::LeftHalfBox | Cell::RightHalfBox => panic!(),
                }
            },
            Cell::LeftHalfBox | Cell::RightHalfBox => {
                let (left_curr, right_curr) = match self.grid[self.idx(pos)] {
                    Cell::LeftHalfBox => (pos, pos.step(Dir::Right)),
                    Cell::RightHalfBox => (pos.step(Dir::Left), pos),
                    _ => unreachable!(),
                };
                let left_next = left_curr.step(dir);
                let right_next = right_curr.step(dir);

                let left_curr_idx = self.idx(left_curr);
                let right_curr_idx = self.idx(right_curr);
                self.grid[left_curr_idx] = Cell::Empty;
                self.grid[right_curr_idx] = Cell::Empty;

                match self.grid[self.idx(left_next)] {
                    Cell::Empty => (),
                    Cell::LeftHalfBox | Cell::RightHalfBox => self.step_box(left_next, dir),
                    Cell::WholeBox | Cell::Wall => panic!(),
                };
                match self.grid[self.idx(right_next)] {
                    Cell::Empty => (),
                    Cell::LeftHalfBox | Cell::RightHalfBox => self.step_box(right_next, dir),
                    Cell::WholeBox | Cell::Wall => panic!(),
                };

                let left_next_idx = self.idx(left_next);
                let right_next_idx = self.idx(right_next);
                self.grid[left_next_idx] = Cell::LeftHalfBox;
                self.grid[right_next_idx] = Cell::RightHalfBox;
            },
            Cell::Empty | Cell::Wall => panic!(),
        }
    }

    fn try_step(&mut self, dir: Dir) {
        let first = self.robot.step(dir);
        match self.grid[self.idx(first)] {
            Cell::Wall => return,
            Cell::Empty => {
                self.robot = first;
                return;
            },
            Cell::WholeBox | Cell::LeftHalfBox | Cell::RightHalfBox => (),
        };
        if !self.can_step_box(first, dir) { return; }
        self.step_box(first, dir);
        self.robot = first;
    }

    fn scale_up(&self) -> Map {
        let mut grid = vec![];
        for cell in &self.grid {
            let [a, b] = match *cell {
                Cell::Empty    => [Cell::Empty, Cell::Empty],
                Cell::Wall     => [Cell::Wall, Cell::Wall],
                Cell::WholeBox => [Cell::LeftHalfBox, Cell::RightHalfBox],
                Cell::LeftHalfBox | Cell::RightHalfBox => panic!(),
            };
            grid.push(a);
            grid.push(b);
        }
        Map {
            width: self.width * 2,
            height: self.height,
            robot: Pos { x: self.robot.x * 2, y: self.robot.y },
            grid,
        }
    }
}

fn parse_map(map_input: &str) -> Map {
    let height = map_input.lines().count();
    let width = map_input.lines().next().unwrap().chars().count();
    let mut grid = vec![];
    let mut robot: Option<Pos> = None;
    for (y, line) in map_input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let cell = match ch {
                '#' => Cell::Wall,
                '.' => Cell::Empty,
                'O' => Cell::WholeBox,
                '@' => {
                    assert!(robot.is_none());
                    robot = Some(Pos { x, y });
                    Cell::Empty
                },
                _ => panic!(),
            };
            grid.push(cell);
        }
    }
    Map { width, height, robot: robot.unwrap(), grid }
}

fn parse_directions(directions_input: &str) -> Vec<Dir> {
    directions_input.chars().filter(|ch| !ch.is_whitespace()).map(|ch| match ch {
        '^' => Dir::Up,
        'v' => Dir::Down,
        '<' => Dir::Left,
        '>' => Dir::Right,
        _   => panic!(),
    }).collect()
}

fn parse(puzzle_input: &str) -> (Map, Vec<Dir>) {
    let mut sp = puzzle_input.split("\n\n");
    let map = parse_map(sp.next().unwrap());
    let directions = parse_directions(sp.next().unwrap());
    assert!(sp.next().is_none());
    (map, directions)
}

fn sum_coords(map: &Map) -> usize {
    let mut sum = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            let pos = Pos { x, y };
            if let Cell::WholeBox | Cell::LeftHalfBox = map.grid[map.idx(pos)] {
                sum += pos.coord();
            }
        }
    }
    sum
}

fn part1(mut map: Map, directions: &[Dir]) -> usize {
    for &dir in directions {
        map.try_step(dir);
    }
    sum_coords(&map)
}

fn part2(map: &Map, directions: &[Dir]) -> usize {
    let mut map = map.scale_up();
    for &dir in directions {
        map.try_step(dir);
    }
    sum_coords(&map)
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let (map, directions) = parse(&puzzle_input);
    println!("{}", part1(map.clone(), &directions));
    println!("{}", part2(&map, &directions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const LARGE_EX: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

     const SMALL_EX: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";


    #[test]
    fn test_part1() {
        let (map, directions) = parse(SMALL_EX);
        assert_eq!(part1(map, &directions), 2028);
        let (map, directions) = parse(LARGE_EX);
        assert_eq!(part1(map, &directions), 10092);
    }

    #[test]
    fn test_part2() {
        let (map, directions) = parse(LARGE_EX);
        assert_eq!(part2(&map, &directions), 9021);
    }
}
