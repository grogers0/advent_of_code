use std::collections::HashSet;
use std::io::{self, Read};

#[derive(Copy, Clone)]
enum Dir {
    E, SE, SW, W, NW, NE
}

// Axial coordinates, e.g. in the points around the origin:
//   x-coord     y-coord
//    0   1      -1  -1
// -1   0   1   0   0   0
//   -1   0       1   1
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Coord {
    x: i32,
    y: i32
}

impl Dir {
    fn to_coord(&self) -> Coord {
        match self {
            Dir::E  => Coord { x:  1, y:  0 },
            Dir::SE => Coord { x:  0, y:  1 },
            Dir::NE => Coord { x:  1, y: -1 },
            Dir::W  => Coord { x: -1, y:  0 },
            Dir::SW => Coord { x: -1, y:  1 },
            Dir::NW => Coord { x:  0, y: -1 },
        }
    }
}

impl Coord {
    fn origin() -> Coord {
        Coord { x: 0, y: 0 }
    }
}

impl std::ops::Add for Coord {
    type Output = Coord;
    fn add(self, other: Coord) -> Coord {
        Coord { x: self.x + other.x, y: self.y + other.y }
    }
}

fn parse(puzzle_input: &str) -> Vec<Vec<Dir>> {
    puzzle_input.lines()
        .map(|line| {
            let mut dirs = Vec::new();
            let mut carry = None;
            for ch in line.chars() {
                let (dir, nextcarry) = match (ch, carry) {
                    ('e', None)      => (Some(Dir::E),  None),
                    ('e', Some('s')) => (Some(Dir::SE), None),
                    ('e', Some('n')) => (Some(Dir::NE), None),
                    ('w', None)      => (Some(Dir::W),  None),
                    ('w', Some('s')) => (Some(Dir::SW), None),
                    ('w', Some('n')) => (Some(Dir::NW), None),
                    ('s', None)      => (None, Some('s')),
                    ('n', None)      => (None, Some('n')),
                    _ => panic!()
                };
                dir.map(|d| dirs.push(d));
                carry = nextcarry;
            }
            dirs
        })
        .collect()
}

fn flip(pos: Coord, black_tiles: &mut HashSet<Coord>) {
    if black_tiles.contains(&pos) {
        black_tiles.remove(&pos);
    } else {
        black_tiles.insert(pos);
    }
}

fn initial_black_tiles(paths: &Vec<Vec<Dir>>) -> HashSet<Coord> {
    let mut black_tiles = HashSet::new();
    for path in paths {
        let mut pos = Coord::origin();
        for dir in path {
            pos = pos + dir.to_coord();
        }
        flip(pos, &mut black_tiles);
    }
    black_tiles
}

fn adjacent_tiles(pos: Coord) -> Vec<Coord> {
    [Dir::E, Dir::SE, Dir::SW, Dir::W, Dir::NW, Dir::NE].iter()
        .map(|dir| pos + dir.to_coord()).collect()
}

fn count_adjacent_black_tiles(pos: Coord, black_tiles: &HashSet<Coord>) -> usize {
    adjacent_tiles(pos).into_iter().filter(|adj| black_tiles.contains(adj)).count()
}

fn cycle(prev: HashSet<Coord>) -> HashSet<Coord> {
    let mut interesting_tiles = HashSet::new();
    for &pos in &prev {
        for adj in adjacent_tiles(pos) {
            interesting_tiles.insert(adj);
        }
        interesting_tiles.insert(pos);
    }
    let mut next = HashSet::new();
    for pos in interesting_tiles {
        let was_black = prev.contains(&pos);
        let adj_black = count_adjacent_black_tiles(pos, &prev);
        let will_black = match (was_black, adj_black) {
            (true, 1..=2) => true,
            (true, _) => false,
            (false, 2) => true,
            (false, _) => false
        };
        if will_black { next.insert(pos); }
    }

    next
}

fn part1(puzzle_input: &str) -> usize {
    let paths = parse(puzzle_input);
    let black_tiles = initial_black_tiles(&paths);
    black_tiles.len()
}

fn part2(puzzle_input: &str) -> usize {
    let paths = parse(puzzle_input);
    let mut black_tiles = initial_black_tiles(&paths);
    for _ in 0..100 {
        black_tiles = cycle(black_tiles);
    }
    black_tiles.len()
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

    const EX: &str = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

    #[test]
    fn test_part1() {
        assert_eq!(10, part1(EX));
    }

    #[test]
    fn test_part2() {
        assert_eq!(2208, part2(EX));
    }
}
