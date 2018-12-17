use std::io::{self, Read};

use lazy_static::lazy_static;
use regex::Regex;

fn parse_line(line: &str) -> Vec<(usize,usize)> {
    lazy_static!{
        static ref RE1: Regex = Regex::new("^x=(\\d+), y=(\\d+)..(\\d+)$").unwrap();
        static ref RE2: Regex = Regex::new("^y=(\\d+), x=(\\d+)..(\\d+)$").unwrap();
    }

    if let Some(cap) = RE1.captures(line) {
        let x = cap[1].parse().unwrap();
        let range = cap[2].parse().unwrap() ..= cap[3].parse().unwrap();
        range.into_iter().map(|y| (x, y)).collect()
    } else if let Some(cap) = RE2.captures(line) {
        let y = cap[1].parse().unwrap();
        let range = cap[2].parse().unwrap() ..= cap[3].parse().unwrap();
        range.into_iter().map(|x| (x, y)).collect()
    } else {
        unreachable!();
    }
}

fn parse(input: &str) -> Vec<Vec<char>> {
    let walls = input.lines().map(|line| parse_line(line)).collect::<Vec<_>>()
        .into_iter().flat_map(|walls| walls).collect::<Vec<_>>();

    let min_x = walls.iter().map(|(x,_)| *x).min().unwrap();
    let max_x = walls.iter().map(|(x,_)| *x).max().unwrap();
    let min_y = walls.iter().map(|(_,y)| *y).min().unwrap();
    let max_y = walls.iter().map(|(_,y)| *y).max().unwrap();

    let mut ret = vec![vec!['.'; max_x-min_x+3]; max_y-min_y+2];
    ret[0][500 - min_x + 1] = '+'; // Spring at (500, 0)
    for (x, y) in walls {
        ret[y - min_y + 1][x - min_x + 1] = '#';
    }

    ret
}

#[allow(dead_code)]
fn print_map(map: &Vec<Vec<char>>) -> String {
    let mut ret = String::new();
    for row in map.iter() {
        for ch in row.iter() {
            ret.push(*ch);
        }
        ret.push('\n');
    }
    ret
}

fn pour_water(map: &mut Vec<Vec<char>>) {
    let height = map.len();
    let width = map[0].len();
    for y in 0..height-1 {
        for x in 0..width {
            if (map[y][x] == '+' || map[y][x] == '|') && map[y+1][x] == '.' {
                map[y+1][x] = '|';
            }
        }
    }
}

fn spread_water(map: &mut Vec<Vec<char>>) {
    let height = map.len();
    let width = map[0].len();
    for y in 0..height-1 {
        for x in 0..width {
            if map[y][x] == '|' && (map[y+1][x] == '~' || map[y+1][x] == '#') {
                if x+1 < width && map[y][x+1] == '.' { map[y][x+1] = '|'; }
                if x > 0 && map[y][x-1] == '.' { map[y][x-1] = '|'; }
            }
        }
    }
}

fn is_puddle(map: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if map[y][x] != '|' { return false; }
    let width = map[0].len();

    for i in 1.. {
        if x+i >= width { return false; }
        match map[y][x+i] {
            '#' => break,
            '|' => (),
            '+' | '.' => { return false; }
            _ => unreachable!()
        }
    }
    for i in 1.. {
        if i > x { return false; }
        match map[y][x-i] {
            '#' => break,
            '|' => (),
            '+' | '.' => { return false; }
            _ => unreachable!()
        }
    }
    true
}

fn fill_puddle(map: &mut Vec<Vec<char>>, x: usize, y: usize) {
    map[y][x] = '~';
    for i in 1.. {
        if map[y][x+i] == '|' {
            map[y][x+i] = '~';
        } else {
            break;
        }
    }
    for i in 1.. {
        if map[y][x-i] == '|' {
            map[y][x-i] = '~';
        } else {
            break;
        }
    }
}

fn pool_water(map: &mut Vec<Vec<char>>) {
    let height = map.len();
    let width = map[0].len();
    for y in 0..height-1 {
        for x in 1..width-1 {
            if is_puddle(map, x, y) {
                fill_puddle(map, x, y);
            }
        }
    }
}

// NOTE - this is pretty slow, it takes about 20 seconds to run on the input file. Instead we
// should be only operating on the squares we know are already water, instead of testing every
// square.
fn step(map: &mut Vec<Vec<char>>) {
    //println!("{}", print_map(map));
    pour_water(map);
    spread_water(map);
    pool_water(map);
}

fn step_until_full(map: &mut Vec<Vec<char>>) {
    loop {
        let orig_map = map.clone();
        step(map);
        if orig_map == *map {
            break;
        }
    }
}

fn part1(input: &str) -> usize {
    let mut map = parse(input);
    step_until_full(&mut map);

    map.iter().map(|row| {
        row.iter().filter(|ch| **ch == '|' || **ch == '~').count()
    }).sum()
}

fn part2(input: &str) -> usize {
    let mut map = parse(input);
    step_until_full(&mut map);

    map.iter().map(|row| {
        row.iter().filter(|ch| **ch == '~').count()
    }).sum()
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
x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), 57);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EX), 29);
    }
}
