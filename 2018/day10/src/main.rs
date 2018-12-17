use std::io::{self, Read};
use std::ops::RangeInclusive;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32
}

fn parse(input: &str) -> Vec<Point> {
    lazy_static!{
        static ref RE: Regex = Regex::new("^position=< *([0-9-]+), *([0-9-]+)> velocity=< *([0-9-]+), *([0-9-]+)>$").unwrap();
    }

    input.lines().map(|line| {
        let cap = RE.captures(line).unwrap();
        Point {
            x: cap[1].parse().unwrap(),
            y: cap[2].parse().unwrap(),
            dx: cap[3].parse().unwrap(),
            dy: cap[4].parse().unwrap()
        }
    })
    .collect()
}

fn bounding_box(points: &Vec<Point>) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    let mut min_x = std::i32::MAX;
    let mut max_x = std::i32::MIN;
    let mut min_y = std::i32::MAX;
    let mut max_y = std::i32::MIN;
    for p in points {
        if p.x < min_x { min_x = p.x }
        if p.x > max_x { max_x = p.x }
        if p.y < min_y { min_y = p.y }
        if p.y > max_y { max_y = p.y }
    }
    (min_x..=max_x, min_y..=max_y)
}

fn bounding_box_size(points: &Vec<Point>) -> usize {
    let (range_x, range_y) = bounding_box(points);
    (range_x.end() - range_x.start() + 1) as usize +
        (range_y.end() - range_y.start() + 1) as usize
}


fn calc(input: &str) -> (usize, String) {
    let mut points = parse(input);
    for i in 0.. {
        let next_points: Vec<Point> = points.iter()
            .map(|p| {
                Point {
                    x: p.x + p.dx,
                    y: p.y + p.dy,
                    dx: p.dx,
                    dy: p.dy
                }
            })
            .collect();
        if bounding_box_size(&next_points) > bounding_box_size(&points) {
            let (range_x, range_y) = bounding_box(&points);
            let mut output = String::new();
            for y in range_y.clone() {
                for x in range_x.clone() {
                    if points.iter().any(|p| p.x == x && p.y == y) {
                        output.push('#');
                    } else {
                        output.push('.');
                    }
                }
                if y != *range_y.end() {
                    output.push('\n');
                }
            }
            return (i, output);
        }

        points = next_points;
    }
    unreachable!();
}

fn part1(input: &str) -> String {
    calc(input).1
}

fn part2(input: &str) -> usize {
    calc(input).0
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

    const EX: &str = "\
position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), "\
#...#..###
#...#...#.
#...#...#.
#####...#.
#...#...#.
#...#...#.
#...#...#.
#...#..###".to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EX), 3);
    }

}
