use std::io::{self, Read};
use std::collections::HashSet;

// Bitset where bits correspond to positions:
//
// 012
// 345
// 678
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Shape(u16);

impl Shape {
    // Rotated clockwise
    fn rotated(self) -> Self {
        let mut out = Shape(self.0 & (1<<4));
        if self.0 & (1<<0) != 0 { out.0 |= 1<<2; }
        if self.0 & (1<<1) != 0 { out.0 |= 1<<5; }
        if self.0 & (1<<2) != 0 { out.0 |= 1<<8; }
        if self.0 & (1<<5) != 0 { out.0 |= 1<<7; }
        if self.0 & (1<<8) != 0 { out.0 |= 1<<6; }
        if self.0 & (1<<7) != 0 { out.0 |= 1<<3; }
        if self.0 & (1<<6) != 0 { out.0 |= 1<<0; }
        if self.0 & (1<<3) != 0 { out.0 |= 1<<1; }
        out
    }

    // Flipped along vertical axis
    fn flipped(self) -> Self {
        Shape(
            (self.0 & ((1<<1) | (1<<4) | (1<<7))) |
            ((self.0 & ((1<<0) | (1<<3) | (1<<6))) << 2) |
            ((self.0 & ((1<<2) | (1<<5) | (1<<8))) >> 2))
    }

    fn orientations(self) -> HashSet<Shape> {
        let mut ret = HashSet::new();
        ret.insert(self);
        ret.insert(self.rotated());
        ret.insert(self.rotated().rotated());
        ret.insert(self.rotated().rotated().rotated());
        ret.insert(self.flipped());
        ret.insert(self.flipped().rotated());
        ret.insert(self.flipped().rotated().rotated());
        ret.insert(self.flipped().rotated().rotated().rotated());
        ret
    }

    fn xoffset(&self) -> usize {
        for i in 0..3 {
            if self.0 & (1<<i) != 0 {
                return i;
            }
        }
        panic!("Offset for y is not implemented, none of the rotations should produce this");
    }
}

#[derive(Clone)]
struct Region {
    width: usize,
    height: usize,
    shapes: Vec<usize>,
    grid: Vec<bool>,
}

impl Region {
    fn idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    // NOTE - placing at x,y means that the cell at x,y is guaranteed to be filled by the
    // operation. Shifting where the shape is placed in the x coord is handled automatically
    fn can_place(&self, x: usize, y: usize, shape: Shape) -> bool {
        if shape.xoffset() > x { return false; }
        let x = x - shape.xoffset();
        if x + 2 >= self.width { return false; }
        if y + 2 >= self.height { return false; }
        if self.grid[self.idx(x  , y  )] && shape.0 & (1<<0) != 0 { return false; }
        if self.grid[self.idx(x+1, y  )] && shape.0 & (1<<1) != 0 { return false; }
        if self.grid[self.idx(x+2, y  )] && shape.0 & (1<<2) != 0 { return false; }
        if self.grid[self.idx(x  , y+1)] && shape.0 & (1<<3) != 0 { return false; }
        if self.grid[self.idx(x+1, y+1)] && shape.0 & (1<<4) != 0 { return false; }
        if self.grid[self.idx(x+2, y+1)] && shape.0 & (1<<5) != 0 { return false; }
        if self.grid[self.idx(x  , y+2)] && shape.0 & (1<<6) != 0 { return false; }
        if self.grid[self.idx(x+1, y+2)] && shape.0 & (1<<7) != 0 { return false; }
        if self.grid[self.idx(x+2, y+2)] && shape.0 & (1<<8) != 0 { return false; }
        true
    }

    fn place_unplace_inner(&mut self, x: usize, y: usize, shape: Shape, val: bool) {
        let x = x - shape.xoffset();
        if shape.0 & (1<<0) != 0 { let i = self.idx(x  , y  ); self.grid[i] = val; }
        if shape.0 & (1<<1) != 0 { let i = self.idx(x+1, y  ); self.grid[i] = val; }
        if shape.0 & (1<<2) != 0 { let i = self.idx(x+2, y  ); self.grid[i] = val; }
        if shape.0 & (1<<3) != 0 { let i = self.idx(x  , y+1); self.grid[i] = val; }
        if shape.0 & (1<<4) != 0 { let i = self.idx(x+1, y+1); self.grid[i] = val; }
        if shape.0 & (1<<5) != 0 { let i = self.idx(x+2, y+1); self.grid[i] = val; }
        if shape.0 & (1<<6) != 0 { let i = self.idx(x  , y+2); self.grid[i] = val; }
        if shape.0 & (1<<7) != 0 { let i = self.idx(x+1, y+2); self.grid[i] = val; }
        if shape.0 & (1<<8) != 0 { let i = self.idx(x+2, y+2); self.grid[i] = val; }
    }


    fn place(&mut self, x: usize, y: usize, shape: Shape) {
        self.place_unplace_inner(x, y, shape, true);
    }

    fn unplace(&mut self, x: usize, y: usize, shape: Shape) {
        self.place_unplace_inner(x, y, shape, false);
    }
}

struct Summary {
    shapes: Vec<Shape>,
    regions: Vec<Region>,
}


fn parse_regions(input: &str) -> Vec<Region> {
    let mut ret = Vec::new();
    for line in input.lines() {
        let mut sp = line.split(" ");
        let mut width = 0;
        let mut height = 0;
        {
            let size_str = sp.next().unwrap();
            let mut seenx = false;
            let mut seencolon = false;
            for ch in size_str.chars() {
                assert!(!seencolon); // last char
                match ch {
                    '0'..='9' => {
                        let digit = ch.to_digit(10).unwrap() as usize;
                        if seenx {
                            height = height * 10 + digit;
                        } else {
                            width = width * 10 + digit;
                        };
                    },
                    'x' => {
                        assert!(!seenx);
                        seenx = true;
                    },
                    ':' => seencolon = true,
                    _ => panic!(),
                };
            }
        }
        let shapes = sp.map(|s| s.parse::<usize>().unwrap()).collect();
        let grid = vec![false; width * height];
        ret.push(Region { width, height, shapes, grid });
    }
    ret
}

fn parse(puzzle_input: &str) -> Summary {
    let mut shapes = Vec::new();
    let mut sp = puzzle_input.split("\n\n").collect::<Vec<_>>();
    let regions = parse_regions(sp.pop().unwrap());
    for shape_input in sp {
        let mut lines = shape_input.lines();
        lines.next().unwrap();
        let mut shape_val = 0;
        for (y, line) in lines.enumerate() {
            for (x, ch) in line.chars().enumerate() {
                assert!(ch == '#' || ch == '.');
                if ch == '#' {
                    shape_val |= 1 << (y * 3 + x);
                }
            }
        }
        shapes.push(Shape(shape_val));
    }

    Summary { shapes, regions }
}

fn can_fit_conservatively(region: &Region, shapes: &[Shape], x: usize, y: usize) -> bool {
    let available = region.grid[region.idx(x, y)..].iter().filter(|&&b| !b).count();
    let required = region.shapes.iter().enumerate()
        .map(|(i,&cnt)| cnt * shapes[i].0.count_ones() as usize).sum::<usize>();
    available >= required
}

fn try_place_all(region: &mut Region, shapes: &[Shape], x: usize, y: usize) -> bool {
    if region.shapes.iter().all(|&s| s == 0) { return true; }
    if y >= region.height { return false; }
    if !can_fit_conservatively(region, shapes, x, y) { return false; }
    let (x2, y2) = if x + 1 == region.width { (0, y + 1) } else { (x + 1, y) };

    assert_eq!(shapes.len(), region.shapes.len());
    if !region.grid[region.idx(x, y)] {
        for i in 0..shapes.len() {
            if region.shapes[i] == 0 { continue; }
            region.shapes[i] -= 1;
            for shape in shapes[i].orientations() {
                if region.can_place(x, y, shape) {
                    region.place(x, y, shape);
                    if try_place_all(region, shapes, x2, y2) {
                        return true;
                    }
                    region.unplace(x, y, shape);
                }
            }
            region.shapes[i] += 1;
        }
    }

    // NOTE - x,y may be filled already, but we can also try with leaving it blank
    try_place_all(region, shapes, x2, y2)
}

fn part1(summary: &Summary) -> usize {
    let mut cnt = 0;
    for region in &summary.regions {
        if try_place_all(&mut region.clone(), &summary.shapes, 0, 0) {
            cnt += 1;
        }
    }
    cnt
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let summary = parse(&puzzle_input);
    println!("{}", part1(&summary));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX)), 2);
    }
}
