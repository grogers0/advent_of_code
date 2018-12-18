use std::io::{self, Read};

use lazy_static::lazy_static;
use regex::Regex;

enum Cmd {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateCol(usize, usize)
}

fn parse(input: &str) -> Vec<Cmd> {
    lazy_static!{
        static ref RECT_RE: Regex = Regex::new("^rect (\\d+)x(\\d+)$").unwrap();
        static ref ROT_ROW_RE: Regex = Regex::new("^rotate row y=(\\d+) by (\\d+)$").unwrap();
        static ref ROT_COL_RE: Regex = Regex::new("^rotate column x=(\\d+) by (\\d+)$").unwrap();
    }
    input.lines()
        .map(|line| {
            if let Some(cap) = RECT_RE.captures(line) {
                Cmd::Rect(cap[1].parse().unwrap(), cap[2].parse().unwrap())
            } else if let Some(cap) = ROT_ROW_RE.captures(line) {
                Cmd::RotateRow(cap[1].parse().unwrap(), cap[2].parse().unwrap())
            } else if let Some(cap) = ROT_COL_RE.captures(line) {
                Cmd::RotateCol(cap[1].parse().unwrap(), cap[2].parse().unwrap())
            } else {
                unreachable!()
            }
        })
        .collect()
}

fn new_screen(width: usize, height: usize) -> Vec<Vec<bool>> {
    vec![vec![false; width]; height]
}

fn rotate(dots: &Vec<bool>, shift: usize) -> Vec<bool> {
    let mut ret = vec![false; dots.len()];
    for i in 0..dots.len() {
        ret[(i + shift) % dots.len()] = dots[i];
    }
    ret
}

fn execute(screen: &mut Vec<Vec<bool>>, cmd: &Cmd) {
    match *cmd {
        Cmd::Rect(width, height) => {
            for y in 0..height {
                for x in 0..width {
                    screen[y][x] = true;
                }
            }
        },
        Cmd::RotateRow(y, shift) => {
            screen[y] = rotate(&screen[y], shift);
        },
        Cmd::RotateCol(x, shift) => {
            let column = (0..screen.len()).map(|y| screen[y][x]).collect();
            let column = rotate(&column, shift);
            for (y, dot) in column.into_iter().enumerate() {
                screen[y][x] = dot;
            }
        }
    }
}

fn lit_pixels(screen: &Vec<Vec<bool>>) -> usize {
    screen.iter()
        .map(|row| row.iter().filter(|dot| **dot).count())
        .sum()
}

fn screen_to_string(screen: &Vec<Vec<bool>>) -> String {
    let mut ret = String::new();
    for row in screen.iter() {
        for dot in row.iter() {
            ret.push(if *dot { '#' } else { '.' });
        }
        ret.push('\n');
    }
    ret
}

fn part1(input: &str) -> usize {
    let mut screen = new_screen(50, 6);
    for cmd in parse(input).iter() {
        execute(&mut screen, cmd);
    }
    lit_pixels(&screen)
}

fn part2(input: &str) -> String {
    let mut screen = new_screen(50, 6);
    for cmd in parse(input).iter() {
        execute(&mut screen, cmd);
    }
    screen_to_string(&screen)
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
rect 3x2
rotate column x=1 by 1
rotate row y=0 by 4
rotate column x=1 by 1";

        let mut screen = new_screen(7, 3);
        for cmd in parse(ex).iter() {
            execute(&mut screen, cmd);
        }

        let result = "\
.#..#.#
#.#....
.#.....";

        assert_eq!(screen_to_string(&screen).trim_end(), result);
    }
}
