use std::io::{self, Read};
use std::cmp::{min, max};
use std::collections::{BTreeSet, HashMap, VecDeque};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Pos {
    x: u64,
    y: u64,
}

fn parse(puzzle_input: &str) -> Vec<Pos> {
    let mut ret = vec![];
    for line in puzzle_input.lines() {
        let mut sp = line.split(",");
        let x = sp.next().unwrap().parse::<u64>().unwrap();
        let y = sp.next().unwrap().parse::<u64>().unwrap();
        assert!(sp.next().is_none());
        ret.push(Pos { x, y });
    }
    ret
}

fn rect_area(a: &Pos, b: &Pos) -> u64 {
    fn abs_diff(a: u64, b: u64) -> u64 {
        if a > b { a - b } else { b - a }
    }
    let dx = abs_diff(a.x, b.x);
    let dy = abs_diff(a.y, b.y);
    (dx + 1) * (dy + 1)
}

fn part1(tiles: &[Pos]) -> u64 {
    let mut max_area = 0;
    for i in 0..(tiles.len() - 1) {
        for j in (i+1)..tiles.len() {
            max_area = max(max_area, rect_area(&tiles[i], &tiles[j]));
        }
    }
    max_area
}

// Compresses the x,y coordinates into a new coordinate space such that x' may include several
// columns of the original (and likewise y' for rows). Returns (tiles, dxs, dys). Guarantees a ring
// around all tile positions to allow flood filling from outside (but makes it 0 width).
fn compress_coords(input_tiles: &[Pos]) -> (Vec<Pos>, Vec<u64>, Vec<u64>) {
    let orig_xs = input_tiles.iter().map(|pos| pos.x).collect::<BTreeSet<_>>()
        .into_iter().collect::<Vec<_>>();
    let mut dxs = vec![0];
    let mut mapx = HashMap::new();
    let mut last_x = 0;
    for x in orig_xs {
        if x > last_x + 1 {
            mapx.insert(x, dxs.len() as u64);
            dxs.push(x - last_x - 1);
        }
        mapx.insert(x, dxs.len() as u64);
        dxs.push(1);
        last_x = x;
    }
    dxs.push(0);
    

    let orig_ys = input_tiles.iter().map(|pos| pos.y).collect::<BTreeSet<_>>()
        .into_iter().collect::<Vec<_>>();
    let mut dys = vec![0];
    let mut mapy = HashMap::new();
    let mut last_y = 0;
    for y in orig_ys {
        if y > last_y + 1 {
            mapy.insert(y, dys.len() as u64);
            dys.push(y - last_y - 1);
        }
        mapy.insert(y, dys.len() as u64);
        dys.push(1);
        last_y = y;
    }
    dys.push(0);

    let mapped_tiles = input_tiles.iter()
        .map(|pos| Pos { x: mapx[&pos.x], y: mapy[&pos.y] }).collect();
    (mapped_tiles, dxs, dys)
}

#[derive(Copy, Clone)]
enum Cell {
    Red, Green, Empty, Outside,
}

fn part2(input_tiles: &[Pos]) -> u64 {
    let (mapped_tiles, dxs, dys) = compress_coords(&input_tiles);
    let idx = |x: u64, y: u64| -> usize {
        y as usize * dxs.len() + x as usize
    };
    let mut grid = vec![Cell::Empty; dxs.len() * dys.len()];
    let connect = |grid: &mut [Cell], a: Pos, b: Pos| {
        if a.x == b.x {
            if a.y < b.y {
                for y in (a.y+1)..b.y { grid[idx(a.x, y)] = Cell::Green; }
            } else if b.y < a.y {
                for y in (b.y+1)..a.y { grid[idx(a.x, y)] = Cell::Green; }
            } else {
                panic!();
            }
        } else if a.y == b.y {
            if a.x < b.x {
                for x in (a.x+1)..b.x { grid[idx(x, a.y)] = Cell::Green; }
            } else if b.x < a.x {
                for x in (b.x+1)..a.x { grid[idx(x, a.y)] = Cell::Green; }
            } else {
                panic!();
            }
        } else {
            panic!();
        }
    };

    for pos in mapped_tiles.iter() {
        grid[idx(pos.x, pos.y)] = Cell::Red;
    }
    let mut last_pos = mapped_tiles[0];
    for pos in mapped_tiles[1..].iter() {
        connect(&mut grid, last_pos, *pos);
        last_pos = *pos;
    }
    connect(&mut grid, last_pos, mapped_tiles[0]);

    {
        // Flood fill the outside, any remaining empty cells are inside (so green)
        let mut deque = VecDeque::new();
        deque.push_back((0, 0));
        while let Some((x, y)) = deque.pop_front() {
            if let Cell::Empty = grid[idx(x, y)] {
                grid[idx(x, y)] = Cell::Outside;
                if x > 0                    { deque.push_back((x - 1, y    )); }
                if x < dxs.len() as u64 - 1 { deque.push_back((x + 1, y    )); }
                if y > 0                    { deque.push_back((x    , y - 1)); }
                if y < dys.len() as u64 - 1 { deque.push_back((x    , y + 1)); }
            }
        }

        for y in 0..(dys.len() as u64) {
            for x in 0..(dxs.len() as u64) {
                if let Cell::Empty = grid[idx(x, y)] {
                    grid[idx(x, y)] = Cell::Green;
                }
            }
        }
    }

    let mapped_rect_area = |a: Pos, b: Pos| -> Option<u64> {
        let x1 = min(a.x, b.x);
        let x2 = max(a.x, b.x);
        let y1 = min(a.y, b.y);
        let y2 = max(a.y, b.y);
        for y in y1..=y2 {
            for x in x1..=x2 {
                match grid[idx(x, y)] {
                    Cell::Red | Cell::Green => (),
                    Cell::Empty | Cell::Outside => return None,
                }
            }
        }
        let mut dx = 0;
        let mut dy = 0;
        for x in x1..=x2 {
            dx += dxs[x as usize];
        }
        for y in y1..=y2 {
            dy += dys[y as usize];
        }
        Some(dx * dy)
    };

    let mut max_area = 0;
    for i in 0..(mapped_tiles.len() - 1) {
        for j in (i+1)..mapped_tiles.len() {
            if let Some(area) = mapped_rect_area(mapped_tiles[i], mapped_tiles[j]) {
                max_area = max(max_area, area);
            }
        }
    }
    max_area
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let input_tiles = parse(&puzzle_input);
    println!("{}", part1(&input_tiles));
    println!("{}", part2(&input_tiles));
}

#[cfg(test)]
mod tests {
    use super::*;

const EX: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX)), 50);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EX)), 24);
    }
}
