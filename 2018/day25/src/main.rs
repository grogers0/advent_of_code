use std::collections::{BTreeSet, VecDeque};
use std::io::{self, Read};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Point {
    x: i64, y: i64, z: i64, t: i64
}

impl Point {
    fn new(x: i64, y: i64, z: i64, t: i64) -> Point {
        Point { x, y, z, t }
    }
}

fn parse(input: &str) -> Vec<Point> {
    input.lines().map(|line| {
        let mut elems = line.split(",");
        let x = elems.next().unwrap().parse().unwrap();
        let y = elems.next().unwrap().parse().unwrap();
        let z = elems.next().unwrap().parse().unwrap();
        let t = elems.next().unwrap().parse().unwrap();
        Point::new(x, y, z, t)
    })
    .collect()
}

fn manhattan_distance(p1: &Point, p2: &Point) -> u64 {
    ((p1.x - p2.x).abs() +
        (p1.y - p2.y).abs() +
        (p1.z - p2.z).abs() +
        (p1.t - p2.t).abs()) as u64
}

// NOTE - This is O(n^2) but is fast enough on the input size given.
fn possible_constellation_points(p: &Point, points: &Vec<Point>) -> VecDeque<Point> {
    points.iter()
        .filter(|p2| manhattan_distance(p, *p2) <= 3)
        .cloned()
        .collect()
}

fn part1(input: &str) -> usize {
    let points = parse(input);
    let mut seen = BTreeSet::new();
    let mut num_constellations = 0;
    for p in points.iter() {
        if seen.contains(p) { continue }
        num_constellations += 1;
        let mut queue = VecDeque::new();
        queue.push_back(p.clone());
        while let Some(p) = queue.pop_front() {
            if !seen.insert(p.clone()) { continue }
            queue.append(&mut possible_constellation_points(&p, &points));
        }
    }

    num_constellations
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", part1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let ex1 = "\
0,0,0,0
3,0,0,0
0,3,0,0
0,0,3,0
0,0,0,3
0,0,0,6
9,0,0,0
12,0,0,0";
        assert_eq!(part1(ex1), 2);
        let ex1a = "\
0,0,0,0
3,0,0,0
0,3,0,0
0,0,3,0
0,0,0,3
0,0,0,6
9,0,0,0
12,0,0,0
6,0,0,0";
        assert_eq!(part1(ex1a), 1);
        let ex2 = "\
-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0";
        assert_eq!(part1(ex2), 4);
        let ex3 = "\
1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2";
        assert_eq!(part1(ex3), 3);
        let ex4 = "\
1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2";
        assert_eq!(part1(ex4), 8);
    }
}
