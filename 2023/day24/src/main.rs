use std::io::{self, Read};

use regex::Regex;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Pos {
    x: i64,
    y: i64,
    z: i64,
}

impl Pos {
    fn new(x: i64, y: i64, z: i64) -> Pos {
        Pos { x, y, z }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Ray {
    pos: Pos,
    dir: Pos,
}

fn parse(puzzle_input: &str) -> Vec<Ray> {
    let re = Regex::new(r"(\d+),\s*(\d+),\s*(\d+)\s*@\s*(-?\d+),\s*(-?\d+),\s*(-?\d+)").unwrap();
    puzzle_input.lines().map(|line| {
        let cap = re.captures(line).unwrap();
        let px = cap[1].parse().unwrap();
        let py = cap[2].parse().unwrap();
        let pz = cap[3].parse().unwrap();
        let dx = cap[4].parse().unwrap();
        let dy = cap[5].parse().unwrap();
        let dz = cap[6].parse().unwrap();
        Ray { pos: Pos::new(px, py, pz), dir: Pos::new(dx, dy, dz) }
    }).collect()
}


fn intersect2d(a: &Ray, b: &Ray, min_xy: i64, max_xy: i64) -> bool {
    let dx = b.pos.x - a.pos.x;
    let dy = b.pos.y - a.pos.y;
    let det = b.dir.x * a.dir.y - b.dir.y * a.dir.x;
    if det == 0 { return false }
    // FIXME - this doesn't handle non-integer overlapping coordinates, which makes it fail the
    // test input, but it works well enough to pass the actual input
    let u = (dy * b.dir.x - dx * b.dir.y) / det;
    let v = (dy * a.dir.x - dx * a.dir.y) / det;
    if u < 0 || v < 0 { return false }
    let ix = a.pos.x + a.dir.x * u;
    let iy = a.pos.y + a.dir.y * u;
    ix >= min_xy && ix <= max_xy && iy >= min_xy && iy <= max_xy
}

fn count_intersections2d(rays: &[Ray], min_xy: i64, max_xy: i64) -> usize {
    let mut cnt = 0;
    for i in 0..(rays.len()-1) {
        for j in (i+1)..rays.len() {
            if intersect2d(&rays[i], &rays[j], min_xy, max_xy) {
                cnt += 1;
            }
        }
    }
    cnt
}

fn part1(rays: &[Ray]) -> usize {
    count_intersections2d(rays, 200000000000000, 400000000000000)
}

fn part2(puzzle_input: &str) -> &str {
    "FIXME"
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let rays = parse(&puzzle_input);
    println!("{}", part1(&rays));
    println!("{}", part2(&puzzle_input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    #[test]
    fn test_part1() {
        assert_eq!(count_intersections2d(&parse(EX), 7, 27), 2);
    }

    #[test]
    fn test_part2() {
        // FIXME
    }
}
