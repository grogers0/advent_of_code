use std::collections::BTreeSet;
use std::io::{self, Read};
use std::mem;

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines()
        .map(|line| line.chars().map(|ch| ch).collect())
        .collect()
}

#[allow(dead_code)]
fn map_to_string(map: &Vec<Vec<char>>) -> String {
    let mut ret = String::new();
    for row in map.iter() {
        for ch in row.iter() {
            ret.push(*ch);
        }
        ret.push('\n');
    }
    ret
}

fn adjacent(map: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<char> {
    let height = map.len();
    let width = map[0].len();
    let mut ret = Vec::new();
    if y > 0 {
        if x > 0 { ret.push(map[y - 1][x - 1]); }
        ret.push(map[y - 1][x]);
        if x < width - 1 { ret.push(map[y - 1][x + 1]); }
    }
    if x > 0 { ret.push(map[y][x - 1]); }
    if x < width - 1 { ret.push(map[y][x + 1]); }
    if y < height - 1 {
        if x > 0 { ret.push(map[y + 1][x - 1]); }
        ret.push(map[y + 1][x]);
        if x < width - 1 { ret.push(map[y + 1][x + 1]); }
    }
    ret
}

fn iterate(old_map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let height = old_map.len();
    let width = old_map[0].len();
    let mut map = vec![vec![' '; width]; height];
    for y in 0..height {
        for x in 0..width {
            let adj = adjacent(old_map, x, y);
            map[y][x] = match old_map[y][x] {
                '.' => if adj.iter().filter(|c| **c == '|').count() >= 3 { '|' } else { '.' },
                '|' => if adj.iter().filter(|c| **c == '#').count() >= 3 { '#' } else { '|' },
                '#' => if adj.iter().filter(|c| **c == '#').count() >= 1 && adj.iter().filter(|c| **c == '|').count() >= 1 { '#' } else { '.' },
                _ => unreachable!()
            };
        }
    }
    map
}

fn resource_value(map: &Vec<Vec<char>>) -> usize {
    let num_wooded = map.iter().flat_map(|row| row.iter()).filter(|c| **c == '|').count();
    let num_lumber = map.iter().flat_map(|row| row.iter()).filter(|c| **c == '#').count();
    num_wooded * num_lumber
}

fn part1(input: &str) -> usize {
    let mut map = parse(input);
    for _ in 0..10 {
        map = iterate(&map);
    }
    resource_value(&map)
}

fn part2(input: &str) -> usize {
    let mut map = parse(input);
    let mut iters = 0;
    let max = 1000000000;
    let mut seen = BTreeSet::new();
    while iters < max {
        if !seen.insert(map.clone()) {
            break;
        }
        iters += 1;
        map = iterate(&map);
    }
    mem::drop(seen);

    let mut cycle_len = 0;
    let old_map = map.clone();
    while {
        iters += 1;
        cycle_len += 1;
        map = iterate(&map);

        old_map != map && iters < max
    } {}

    let remaining = (max - iters) % cycle_len;
    for _ in 0..remaining {
        map = iterate(&map);
    }

    resource_value(&map)
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
.#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), 1147);
    }
}
