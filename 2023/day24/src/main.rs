use std::io::{self, Read};

use regex::Regex;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Pos {
    x: i128,
    y: i128,
    z: i128,
}

impl Pos {
    fn new(x: i128, y: i128, z: i128) -> Pos {
        Pos { x, y, z }
    }

    fn cross(self, other: Pos) -> Pos {
        Pos {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    fn dot(self, other: Pos) -> i128 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn mul(self, factor: i128) -> Pos {
        Pos {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        }
    }

    fn div(self, factor: i128) -> Pos {
        Pos {
            x: self.x / factor,
            y: self.y / factor,
            z: self.z / factor,
        }
    }
}

impl std::ops::Sub for Pos {
    type Output = Pos;
    fn sub(self, other: Pos) -> Pos {
        Pos {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Add for Pos {
    type Output = Pos;
    fn add(self, other: Pos) -> Pos {
        Pos {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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


fn intersect2d(a: Ray, b: Ray, min_xy: i128, max_xy: i128) -> bool {
    let dx = b.pos.x - a.pos.x;
    let dy = b.pos.y - a.pos.y;
    let mut det = b.dir.x * a.dir.y - b.dir.y * a.dir.x;
    if det == 0 { return false }
    let mut u_num = dy * b.dir.x - dx * b.dir.y;
    let mut v_num = dy * a.dir.x - dx * a.dir.y;
    if det < 0 {
        det = -det;
        u_num = -u_num;
        v_num = -v_num;
    }
    if u_num < 0 || v_num < 0 { return false }
    let ix = (a.pos.x * det + a.dir.x * u_num) / det;
    let iy = (a.pos.y * det + a.dir.y * u_num) / det;
    ix >= min_xy && ix <= max_xy && iy >= min_xy && iy <= max_xy
}

fn part1(rays: &[Ray], min_xy: i128, max_xy: i128) -> usize {
    let mut cnt = 0;
    for i in 0..(rays.len()-1) {
        for j in (i+1)..rays.len() {
            if intersect2d(rays[i], rays[j], min_xy, max_xy) {
                cnt += 1;
            }
        }
    }
    cnt
}

// I didn't feel like trying to understand the math of intersections in 3d so I just followed:
// https://www.reddit.com/r/adventofcode/comments/18pnycy/comment/kxqjg33/
fn part2(rays: &[Ray]) -> i128 {
    let ray0 = rays[0];
    let ray1 = rays[1];
    let ray2 = rays[2];
    let p1 = ray1.pos - ray0.pos;
    let v1 = ray1.dir - ray0.dir;
    let p2 = ray2.pos - ray0.pos;
    let v2 = ray2.dir - ray0.dir;
    let t1 = -(p1.cross(p2).dot(v2) / v1.cross(p2).dot(v2));
    let t2 = -(p1.cross(p2).dot(v1) / p1.cross(v2).dot(v1));
    let c1 = ray1.pos + ray1.dir.mul(t1);
    let c2 = ray2.pos + ray2.dir.mul(t2);
    let v = (c2 - c1).div(t2 - t1);
    let p = c1 - v.mul(t1);
    p.x + p.y + p.z
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let rays = parse(&puzzle_input);
    println!("{}", part1(&rays, 200000000000000, 400000000000000));
    println!("{}", part2(&rays));
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
        assert_eq!(part1(&parse(EX), 7, 27), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EX)), 47);
    }
}
