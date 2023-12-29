use std::collections::HashSet;
use std::io::{self, Read};

use regex::Regex;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Orient {
    X, Y, Z,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Pos {
    x: u32,
    y: u32,
    z: u32,
}

impl Pos {
    fn new(x: u32, y: u32, z: u32) -> Pos {
        Pos { x, y, z }
    }
}

#[derive(Clone, Debug)]
struct Piece {
    pos: Pos,
    orient: Orient,
    len: u32,
}

impl Piece {
    fn with_pos(&self, pos: Pos) -> Piece {
        Piece { pos, orient: self.orient, len: self.len }
    }
}

fn parse(puzzle_input: &str) -> Vec<Piece> {
    let re = Regex::new(r"(\d+),(\d+),(\d+)~(\d+),(\d+),(\d+)").unwrap();
    puzzle_input.lines().map(|line| {
        let cap = re.captures(line).unwrap();
        let x1 = cap[1].parse().unwrap();
        let y1 = cap[2].parse().unwrap();
        let z1 = cap[3].parse().unwrap();
        let x2 = cap[4].parse().unwrap();
        let y2 = cap[5].parse().unwrap();
        let z2 = cap[6].parse().unwrap();
        let pos = Pos::new(x1, y1, z1);
        if x1 != x2 {
            assert!(y1 == y2 && z1 == z2 && x1 < x2);
            let len = x2 - x1 + 1;
            let orient = Orient::X;
            Piece { pos, orient, len }
        } else if y1 != y2 {
            assert!(x1 == x2 && z1 == z2 && y1 < y2);
            let len = y2 - y1 + 1;
            let orient = Orient::Y;
            Piece { pos, orient, len }
        } else {
            assert!(x1 == x2 && y1 == y2 && z1 <= z2);
            let len = z2 - z1 + 1;
            let orient = Orient::Z;
            Piece { pos, orient, len }
        }
    }).collect()
}

fn fill_piece(piece: &Piece, filled: &mut HashSet<Pos>) {
    let x = piece.pos.x;
    let y = piece.pos.y;
    let z = piece.pos.z;
    match piece.orient {
        Orient::X => {
            for x in x..(x+piece.len) {
                assert!(filled.insert(Pos::new(x, y, z)));
            }
        },
        Orient::Y => {
            for y in y..(y+piece.len) {
                assert!(filled.insert(Pos::new(x, y, z)));
            }
        },
        Orient::Z => {
            for z in z..(z+piece.len) {
                assert!(filled.insert(Pos::new(x, y, z)));
            }
        },
    }
}

fn remove_piece(piece: &Piece, filled: &mut HashSet<Pos>) {
    let mut piece_filled = HashSet::new();
    fill_piece(piece, &mut piece_filled);
    for pos in piece_filled {
        assert!(filled.remove(&pos));
    }
}

fn piece_can_drop(piece: &Piece, filled: &HashSet<Pos>) -> bool {
    if piece.pos.z <= 1 { return false; }
    let new_pos = Pos::new(piece.pos.x, piece.pos.y, piece.pos.z - 1);
    if let Orient::Z = piece.orient {
        return !filled.contains(&new_pos);
    }
    let piece = piece.with_pos(new_pos);
    let mut piece_filled = HashSet::new();
    fill_piece(&piece, &mut piece_filled);
    !piece_filled.into_iter().any(|pos| filled.contains(&pos))
}

fn drop_piece(piece: &mut Piece, filled: &mut HashSet<Pos>) {
    debug_assert!(piece_can_drop(piece, filled));
    remove_piece(piece, filled);
    *piece = piece.with_pos(Pos::new(piece.pos.x, piece.pos.y, piece.pos.z - 1));
    fill_piece(piece, filled);
}

fn has_pieces_above(piece: &Piece, pieces: &[Piece]) -> bool {
    let piece = piece.with_pos(Pos::new(piece.pos.x, piece.pos.y, piece.pos.z + 1));
    let mut filled = HashSet::new();
    fill_piece(&piece, &mut filled);

    for piece in pieces {
        let mut piece_filled = HashSet::new();
        fill_piece(piece, &mut piece_filled);
        if piece_filled.iter().any(|pos| filled.contains(pos)) {
            return true;
        }
    }
    false
}

fn can_any_piece_drop(pieces: &[Piece], filled: &HashSet<Pos>) -> bool {
    pieces.iter().any(|piece| piece_can_drop(piece, filled))
}

fn drop_all_pieces_once(pieces: &mut [Piece], filled: &mut HashSet<Pos>) -> usize {
    let mut cnt = 0;
    for piece in pieces.iter_mut(){
        let mut piece_shifted = false;
        while piece_can_drop(piece, filled) {
            piece_shifted = true;
            drop_piece(piece, filled);
        }
        if piece_shifted {
            cnt += 1;
        }
    }
    cnt
}

fn drop_all_pieces(pieces: &mut [Piece]) -> HashSet<Pos> {
    let mut filled = HashSet::new();
    for piece in pieces.iter() {
        fill_piece(piece, &mut filled);
    }
    pieces.sort_by_key(|piece| piece.pos.z);
    while drop_all_pieces_once(pieces, &mut filled) > 0 {
        pieces.sort_by_key(|piece| piece.pos.z);
    }
    filled
}

fn part1(pieces: &[Piece], mut filled: HashSet<Pos>) -> usize {
    let mut cnt = 0;
    for i in 0..pieces.len() {
        remove_piece(&pieces[i], &mut filled);
        if !has_pieces_above(&pieces[i], &pieces[(i+1)..]) {
            cnt += 1;
        } else if !can_any_piece_drop(&pieces[(i+1)..], &filled) {
            cnt += 1;
        }
        fill_piece(&pieces[i], &mut filled);
    }
    cnt
}

fn part2(pieces: &Vec<Piece>, filled: &HashSet<Pos>) -> usize {
    let mut cnt = 0;
    for i in 0..pieces.len() {
        let mut filled = filled.clone();
        let mut pieces = pieces.clone();
        remove_piece(&pieces[i], &mut filled);
        cnt += drop_all_pieces_once(&mut pieces[(i+1)..], &mut filled);
    }
    cnt
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let mut pieces = parse(&puzzle_input);
    let filled = drop_all_pieces(&mut pieces);
    println!("{}", part1(&pieces, filled.clone()));
    println!("{}", part2(&pieces, &filled));
}


#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";


    #[test]
    fn test_part1() {
        let mut pieces = parse(EX);
        let filled = drop_all_pieces(&mut pieces);
        assert_eq!(part1(&pieces, filled), 5);
    }

    #[test]
    fn test_part2() {
        let mut pieces = parse(EX);
        let filled = drop_all_pieces(&mut pieces);
        assert_eq!(part2(&pieces, &filled), 7);
    }
}
