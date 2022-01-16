use std::cmp::Ordering;
use std::collections::{BinaryHeap, BTreeMap, BTreeSet, HashSet};
use std::io::{self, Read};

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Wall,
    Hallway(bool), // whether the hallway is just outside a room
    Room(char),
}

struct Grid {
    width: usize,
    tiles: Vec<Tile>,
    rooms: BTreeMap<char, BTreeSet<Pos>>,
}

impl Grid {
    fn tile(&self, pos: Pos) -> Tile {
        self.tiles[self.width * pos.y as usize + pos.x as usize]
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Pos {
    x: u8,
    y: u8
}

impl Pos {
    fn up(self) -> Self {
        Pos { x: self.x, y: self.y - 1 }
    }

    fn down(self) -> Self {
        Pos { x: self.x, y: self.y + 1 }
    }

    fn left(self) -> Self {
        Pos { x: self.x - 1, y: self.y }
    }

    fn right(self) -> Self {
        Pos { x: self.x + 1, y: self.y }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    positions: BTreeMap<char, BTreeSet<Pos>>,
    energy: u64,
}

impl State {
    fn all_positions(&self) -> BTreeSet<Pos> {
        let mut ret = BTreeSet::new();
        for positions in self.positions.values() {
            for pos in positions.iter() {
                ret.insert(*pos);
            }
        }
        ret
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reversed order because BinaryHeap max-heap...
        other.energy.cmp(&self.energy)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


fn parse(puzzle_input: &str) -> (Grid, State) {
    let width = puzzle_input.lines().next().unwrap().len();
    let mut tiles = Vec::new();
    let mut rooms = BTreeMap::new();
    let mut positions = BTreeMap::new();
    for (y, line) in puzzle_input.lines().enumerate() {
        assert!(width >= line.len());
        let mut room_chs = 'A'..;
        for (x, ch) in line.chars().enumerate() {
            let tile = match ch {
                '#' | ' ' => Tile::Wall,
                '.' => Tile::Hallway(false),
                'A'..='D' => {
                    if let Tile::Hallway(ref mut room_adjacent) = &mut tiles[width * (y - 1) + x] {
                        *room_adjacent = true;
                    }
                    let pos = Pos { x: x as u8, y: y as u8 };
                    positions.entry(ch).or_insert(BTreeSet::new()).insert(pos);
                    let room_ch = room_chs.next().unwrap();
                    rooms.entry(room_ch).or_insert(BTreeSet::new()).insert(pos);
                    Tile::Room(room_ch)
                },
                _ => panic!(),
            };
            tiles.push(tile);
        }
        for _ in 0 .. (width - line.len()) {
            tiles.push(Tile::Wall)
        }
    }
    let grid = Grid { width, tiles, rooms };
    let state = State { positions, energy: 0 };
    (grid, state)
}

fn move_cost(ch: char) -> u64 {
    match ch {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => panic!(),
    }
}

fn is_final_state(grid: &Grid, state: &State) -> bool {
    grid.rooms == state.positions
}

fn homogenous_room(grid: &Grid, state: &State, room_ch: char) -> bool {
    for (ch, positions) in state.positions.iter() {
        if *ch == room_ch { continue }
        for pos in positions.iter() {
            if Tile::Room(room_ch) == grid.tile(*pos) { return false }
        }
    }
    true
}

fn try_move_out(grid: &Grid, state: &State, queue: &mut BinaryHeap<State>,
    all_positions: &BTreeSet<Pos>, ch: char, orig_pos: Pos, pos: Pos,
    energy_added: u64, seen: &mut HashSet<Pos>)
{
    if !seen.insert(pos) { return }
    if let Tile::Hallway(false) = grid.tile(pos) {
        let mut state = state.clone();
        state.positions.get_mut(&ch).unwrap().remove(&orig_pos);
        state.positions.get_mut(&ch).unwrap().insert(pos);
        state.energy += energy_added;
        queue.push(state);
    }

    for pos in [pos.left(), pos.right(), pos.up(), pos.down()] {
        let can_move = match grid.tile(pos) {
            Tile::Wall => false,
            Tile::Hallway(_) => true,
            room@Tile::Room(_) => room == grid.tile(orig_pos),
        };
        if can_move && !all_positions.contains(&pos) {
            try_move_out(grid, state, queue, all_positions,
                ch, orig_pos, pos, energy_added + move_cost(ch), seen);
        }
    }
}

fn try_move_in(grid: &Grid, state: &State, queue: &mut BinaryHeap<State>,
    all_positions: &BTreeSet<Pos>, ch: char, orig_pos: Pos, pos: Pos,
    energy_added: u64, seen: &mut HashSet<Pos>)
{
    if !seen.insert(pos) { return }
    if Tile::Room(ch) == grid.tile(pos) &&
        (Tile::Wall == grid.tile(pos.down()) || all_positions.contains(&pos.down()))
    {
        let mut state = state.clone();
        state.positions.get_mut(&ch).unwrap().remove(&orig_pos);
        state.positions.get_mut(&ch).unwrap().insert(pos);
        state.energy += energy_added;
        queue.push(state);
        return;
    }

    for pos in [pos.left(), pos.right(), pos.up(), pos.down()] {
        let can_move = match grid.tile(pos) {
            Tile::Wall => false,
            Tile::Hallway(_) => true,
            Tile::Room(room_ch) => room_ch == ch,
        };
        if can_move && !all_positions.contains(&pos) {
            try_move_in(grid, state, queue, all_positions,
                ch, orig_pos, pos, energy_added + move_cost(ch), seen);
        }
    }
}

#[allow(dead_code)]
fn state_to_string(grid: &Grid, state: &State) -> String {
    let mut ret = String::new();
    for y in 0 .. grid.tiles.len()/grid.width {
        if y > 0 { ret.push('\n') }
        'x:
        for x in 0 .. grid.width {
            let pos = Pos { x: x as u8, y: y as u8 };
            match grid.tile(pos) {
                Tile::Wall => ret.push('#'),
                Tile::Hallway(_) | Tile::Room(_) => {
                    for (ch, positions) in state.positions.iter() {
                        if positions.contains(&pos) {
                            ret.push(*ch);
                            continue 'x;
                        }
                    }
                    ret.push('.');
                },
            }
        }
    }
    ret
}

fn find_lowest_energy_path(grid: &Grid, state: State) -> u64 {
    let mut queue = BinaryHeap::new();
    queue.push(state);
    let mut seen = HashSet::new();

    while let Some(state) = queue.pop() {
        if !seen.insert(state.positions.clone()) { continue }
        if is_final_state(grid, &state) {
            return state.energy;
        }

        let all_positions = state.all_positions();
        for (ch, positions) in state.positions.iter() {
            for pos in positions.iter() {
                if let Tile::Room(room_ch) = grid.tile(*pos) {
                    if !homogenous_room(grid, &state, room_ch) {
                        try_move_out(grid, &state, &mut queue, &all_positions,
                            *ch, *pos, *pos, 0, &mut HashSet::new());
                    }
                } else {
                    if homogenous_room(grid, &state, *ch) {
                        try_move_in(grid, &state, &mut queue, &all_positions,
                            *ch, *pos, *pos, 0, &mut HashSet::new());
                    }
                }
            }
        }
    }

    panic!("found no path to the solution")
}

fn part1(puzzle_input: &str) -> u64 {
    let (grid, state) = parse(puzzle_input);
    find_lowest_energy_path(&grid, state)
}

fn part2(puzzle_input: &str) -> u64 {
    let mut mangled = String::new();
    for (i, line) in puzzle_input.lines().enumerate() {
        if i == 3 {
            mangled.push_str("  #D#C#B#A#\n  #D#B#A#C#\n");
        }
        mangled.push_str(line);
        mangled.push('\n');
    }
    let (grid, state) = parse(&mangled);
    find_lowest_energy_path(&grid, state)
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

    const EX: &str = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";

    #[test]
    fn test_part1() {
        assert_eq!(12521, part1(EX));
    }

    #[test]
    fn test_part2() {
        assert_eq!(44169, part2(EX));
    }
}
