use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

type Pos = (i32, i32);
enum Dir { Left, Right, Up, Down }
impl Into<Pos> for &Dir {
    fn into(self) -> Pos {
        match self {
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
            Dir::Up => (0, -1),
            Dir::Down => (0, 1)
        }
    }
}
type Path = Vec<(Dir, i32)>;

fn parse(input: &str) -> [Path; 2] {
    let mut ret = [Vec::new(), Vec::new()];
    for (i, line) in input.lines().enumerate() {
        let mut path = Vec::new();
        for segment in line.split(",") {
            let dir = match &segment[0..1] {
                "L" => Dir::Left,
                "R" => Dir::Right,
                "U" => Dir::Up,
                "D" => Dir::Down,
                _ => panic!()
            };
            let steps = segment[1..].parse().unwrap();
            path.push((dir, steps));
        }
        ret[i] = path;
    }
    ret
}
fn points_touched(path: &Path) -> HashSet<Pos> {
    let mut ret = HashSet::new();
    let mut pos = (0, 0);
    ret.insert(pos);
    for (dir, steps) in path {
        let offset: Pos = dir.into();
        for _ in 0 .. *steps {
            pos.0 += offset.0;
            pos.1 += offset.1;
            ret.insert(pos);
        }
    }
    ret
}

fn signal_delays(path: &Path) -> HashMap<Pos, i32> {
    let mut ret = HashMap::new();
    let mut pos = (0, 0);
    let mut dist = 0;
    ret.insert(pos, dist);
    for (dir, steps) in path {
        let offset: Pos = dir.into();
        for _ in 0 .. *steps {
            pos.0 += offset.0;
            pos.1 += offset.1;
            dist += 1;
            if !ret.contains_key(&pos) { ret.insert(pos, dist); }
        }
    }
    ret
}

fn part1(input: &str) -> i32 {
    let [path1, path2] = parse(input);
    let [points1, points2] = [points_touched(&path1), points_touched(&path2)];

    points1.intersection(&points2)
        .map(|p| p.0.abs() + p.1.abs())
        .filter(|d| *d != 0)
        .min().unwrap()
}

fn part2(input: &str) -> i32 {
    let [path1, path2] = parse(input);
    let [points1, points2] = [points_touched(&path1), points_touched(&path2)];
    let [signal_delays1, signal_delays2] = [signal_delays(&path1), signal_delays(&path2)];

    points1.intersection(&points2)
        .map(|p| signal_delays1.get(p).unwrap() + signal_delays2.get(p).unwrap())
        .filter(|d| *d != 0)
        .min().unwrap()
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
        assert_eq!(part1("R8,U5,L5,D3\nU7,R6,D4,L4"), 6);
        assert_eq!(part1("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"), 159);
        assert_eq!(part1("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"), 135);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("R8,U5,L5,D3\nU7,R6,D4,L4"), 30);
        assert_eq!(part2("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"), 610);
        assert_eq!(part2("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"), 410);
    }
}
