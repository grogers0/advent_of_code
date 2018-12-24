use std::fmt;
use std::io::{self, Read};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Copy, Clone, Debug)]
struct Point {
    x: i64, y: i64, z: i64
}

impl Point {
    fn new(x: i64, y: i64, z: i64) -> Point {
        Point { x: x, y: y, z: z }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{},{},{}>", self.x, self.y, self.z)
    }
}

#[derive(Clone, Debug)]
struct Nanobot {
    position: Point,
    signal_radius: u64
}

fn parse(input: &str) -> Vec<Nanobot> {
    lazy_static!{
        static ref RE: Regex = Regex::new("^pos=<(-?\\d+),(-?\\d+),(-?\\d+)>, r=(\\d+)$").unwrap();
    }
    input.lines()
        .map(|line| {
            let cap = RE.captures(line).unwrap();
            Nanobot {
                position: Point::new(cap[1].parse().unwrap(), cap[2].parse().unwrap(), cap[3].parse().unwrap()),
                signal_radius: cap[4].parse().unwrap()
            }
        })
        .collect()
}

fn manhattan_distance(pos1: Point, pos2: Point) -> u64 {
    ((pos1.x - pos2.x).abs() +
     (pos1.y - pos2.y).abs() +
     (pos1.z - pos2.z).abs()) as u64
}

fn in_range(bot: &Nanobot, pos: Point) -> bool {
    manhattan_distance(bot.position, pos) <= bot.signal_radius
}

fn part1(input: &str) -> usize {
    let bots = parse(input);
    let ref strongest_bot = bots.iter().max_by_key(|bot| bot.signal_radius).unwrap();
    bots.iter().filter(|bot| in_range(strongest_bot, bot.position)).count()
}

fn min_max<F: Fn(&Nanobot) -> i64>(bots: &Vec<Nanobot>, f: &F) -> (i64, i64) {
    let min = bots.iter().map(f).min().unwrap();
    let max = bots.iter().map(f).max().unwrap();
    (min, max)
}

// NOTE - I don't think this is guaranteed to return the optimal square, however it seems to mostly
// work because the input contains bots with similar radii. It's calculating the square with the
// most overlaps on a scaled-down version, and then shrinks the size of the square each iteration,
// only searching within the previous best square.
fn part2(input: &str) -> u64 {
    let bots = parse(input);
    let origin = Point::new(0, 0, 0);
    let (mut min_x, mut max_x) = min_max(&bots, &|bot: &Nanobot| bot.position.x);
    let (mut min_y, mut max_y) = min_max(&bots, &|bot: &Nanobot| bot.position.y);
    let (mut min_z, mut max_z) = min_max(&bots, &|bot: &Nanobot| bot.position.z);

    let mut step = 1;
    while step < max_x - min_x {
        step *= 2;
    }

    loop {
        let mut best_pos = origin;
        let mut best_cnt = 0;

        for x in (min_x..=max_x).step_by(step as usize) {
            for y in (min_y..=max_y).step_by(step as usize) {
                for z in (min_z..=max_z).step_by(step as usize) {
                    let pos = Point::new(x, y, z);
                    let cnt = bots.iter()
                        .filter(|bot| manhattan_distance(bot.position, pos) / step as u64 <= bot.signal_radius / step as u64)
                        .count();
                    if cnt > best_cnt {
                        best_cnt = cnt;
                        best_pos = pos;
                    } else if cnt == best_cnt && manhattan_distance(pos, origin) < manhattan_distance(best_pos, origin) {
                        best_pos = pos;
                    }
                }
            }
        }

        if step == 1 {
            return manhattan_distance(best_pos, origin);
        }
        
        min_x = best_pos.x - step; max_x = best_pos.x + step;
        min_y = best_pos.y - step; max_y = best_pos.y + step;
        min_z = best_pos.z - step; max_z = best_pos.z + step;
        step /= 2;
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let ex = "\
pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1";
        assert_eq!(part1(ex), 7);
    }

    #[test]
    fn test_part2() {
        let ex = "\
pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5";
        assert_eq!(part2(ex), 36);
    }
}
