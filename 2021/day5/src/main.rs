use std::collections::HashMap;
use std::io::{self, Read};

#[derive(Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn parse(input: &str) -> Point {
        let mut split_iter = input.split(",");
        let x = split_iter.next().unwrap().parse().unwrap();
        let y = split_iter.next().unwrap().parse().unwrap();
        assert!(split_iter.next().is_none());
        Point { x, y }
    }
}

struct LineSegment(Point, Point);

impl LineSegment {
    fn parse(input: &str) -> LineSegment {
        let mut split_iter = input.split(" -> ");
        let p1 = Point::parse(split_iter.next().unwrap());
        let p2 = Point::parse(split_iter.next().unwrap());
        assert!(split_iter.next().is_none());
        LineSegment(p1, p2)
    }
}

fn parse(puzzle_input: &str) -> Vec<LineSegment> {
    puzzle_input.lines().map(|line| LineSegment::parse(line)).collect()
}

fn count_overlapping<'a, I>(segments: I) -> usize
where I: Iterator<Item=&'a LineSegment>
{
    let mut point_counts = HashMap::new();
    for seg in segments {
        let mut x = seg.0.x;
        let mut y = seg.0.y;
        let last_x = seg.1.x;
        let last_y = seg.1.y;
        let step_x = if last_x > x { 1 } else if last_x == x { 0 } else { -1 };
        let step_y = if last_y > y { 1 } else if last_y == y { 0 } else { -1 };

        loop {
            point_counts.entry(Point { x, y })
                .and_modify(|cnt| *cnt += 1)
                .or_insert(1);

            if x == last_x && y == last_y { break }
            x += step_x;
            y += step_y;
        }
    }

    point_counts.values().filter(|&&cnt| cnt > 1).count()
}

fn part1(segments: &[LineSegment]) -> usize {
    count_overlapping(segments.iter()
        .filter(|&seg| seg.0.x == seg.1.x || seg.0.y == seg.1.y))
}

fn part2(segments: &[LineSegment]) -> usize {
    count_overlapping(segments.iter())
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let segments = parse(&puzzle_input);
    println!("{}", part1(&segments));
    println!("{}", part2(&segments));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_part1() {
        let segments = parse(EX);
        assert_eq!(5, part1(&segments));
    }

    #[test]
    fn test_part2() {
        let segments = parse(EX);
        assert_eq!(12, part2(&segments));
    }
}
